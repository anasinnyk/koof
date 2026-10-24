use crate::{
    http::Operation,
    resource::{AuthScheme, GroupKind, ManagedResource, ValueExpr},
};
use axum::{
    Json, Router,
    body::Bytes,
    extract::State,
    http::{HeaderMap, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
};
use serde_json::{Map, Value, json};
use std::{
    collections::BTreeMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lifecycle {
    Observe,
    Create,
    Update,
    Delete,
}

#[derive(Debug)]
struct Route {
    kind: GroupKind,
    lifecycle: Lifecycle,
    method: Method,
    template: Vec<&'static str>,
    parameters: Vec<(String, &'static str)>,
    auth: Option<AuthScheme>,
}

#[derive(Debug, Clone)]
struct Shape {
    creatable: bool,
    key: Vec<String>,
    response_path: &'static str,
    create_path: &'static str,
    id_field: &'static str,
    numeric_id: bool,
    fields: Vec<(String, Value)>,
    schema: Value,
}

#[derive(Debug, Default)]
pub struct Fake {
    routes: Vec<Route>,
    shapes: BTreeMap<GroupKind, Shape>,
}

type Key = (GroupKind, Vec<String>);

struct Store {
    fake: Fake,
    objects: Mutex<BTreeMap<Key, Value>>,
    next_id: AtomicU64,
}

impl Fake {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn resource<R: ManagedResource>(mut self) -> Self {
        let descriptor = R::descriptor();
        let kind = descriptor.group_kind();
        let auth = descriptor
            .credentials
            .as_ref()
            .map(|credentials| credentials.auth);
        let mut route = |lifecycle,
                         method: &Method,
                         path: &'static str,
                         parameters: &'static [(&'static str, ValueExpr)]| {
            self.routes.push(Route {
                kind,
                lifecycle,
                method: method.clone(),
                template: path.split('/').collect(),
                parameters: parameters
                    .iter()
                    .map(|(name, expression)| (expression_key(expression), *name))
                    .collect(),
                auth,
            });
        };

        let observe = &descriptor.observe.operation;
        route(
            Lifecycle::Observe,
            &R::Observe::METHOD,
            R::Observe::PATH,
            observe.path_parameters,
        );
        if let Some(create) = &descriptor.create {
            route(
                Lifecycle::Create,
                &R::Create::METHOD,
                R::Create::PATH,
                create.operation.path_parameters,
            );
        }
        if let Some(update) = &descriptor.update {
            route(
                Lifecycle::Update,
                &R::Update::METHOD,
                R::Update::PATH,
                update.operation.path_parameters,
            );
        }
        if let Some(delete) = &descriptor.delete {
            route(
                Lifecycle::Delete,
                &R::Delete::METHOD,
                R::Delete::PATH,
                delete.path_parameters,
            );
        }

        let (create_path, id_field) = descriptor.create.as_ref().map_or(("", "id"), |create| {
            create
                .external_name
                .rsplit_once('/')
                .unwrap_or(("", create.external_name))
        });
        self.shapes.insert(
            kind,
            Shape {
                creatable: descriptor.create.is_some(),
                key: {
                    let mut key: Vec<String> = observe
                        .path_parameters
                        .iter()
                        .map(|(_, expression)| expression_key(expression))
                        .collect();
                    key.sort_unstable();
                    key
                },
                response_path: observe.response_path,
                create_path,
                id_field,
                numeric_id: is_integer(&schemars::schema_for!(R::AtProvider), id_field),
                fields: top_level_fields(&schemars::schema_for!(R::AtProvider)),
                schema: schemars::schema_for!(R::AtProvider).to_value(),
            },
        );
        self
    }

    pub fn router(mut self) -> Router {
        self.routes.sort_by_key(|route| {
            std::cmp::Reverse(route.template.iter().filter(|s| !is_parameter(s)).count())
        });
        let store = Arc::new(Store {
            fake: self,
            objects: Mutex::default(),
            next_id: AtomicU64::new(1),
        });
        Router::new()
            .route("/__fake/state", get(state))
            .fallback(handle)
            .with_state(store)
    }

    /// Serves on `KOOF_FAKE_ADDR` (default `0.0.0.0:8081`).
    ///
    /// # Errors
    ///
    /// Returns an error when the address cannot be bound.
    pub async fn run(self) -> std::io::Result<()> {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .try_init();
        let address = std::env::var("KOOF_FAKE_ADDR").unwrap_or_else(|_| "0.0.0.0:8081".to_owned());
        let listener = tokio::net::TcpListener::bind(&address).await?;
        tracing::info!(%address, resources = ?self.shapes.keys().collect::<Vec<_>>(), "fake API listening");
        axum::serve(listener, self.router())
            .with_graceful_shutdown(async {
                let _ = tokio::signal::ctrl_c().await;
            })
            .await
    }
}

impl Shape {
    fn complete(&self, object: &mut Value) {
        if let Value::Object(object) = object {
            for (field, unset) in &self.fields {
                object.entry(field.clone()).or_insert_with(|| unset.clone());
            }
        }
        fill_required(&self.schema, &self.schema, object);
    }
}

fn resolve<'a>(root: &'a Value, schema: &'a Value) -> &'a Value {
    let mut schema = schema;
    for _ in 0..16 {
        let Some(target) = schema
            .get("$ref")
            .and_then(Value::as_str)
            .and_then(|reference| reference.strip_prefix('#'))
            .and_then(|pointer| root.pointer(pointer))
        else {
            break;
        };
        schema = target;
    }
    schema
}

fn alternatives<'a>(root: &'a Value, schema: &'a Value) -> Vec<&'a Value> {
    let schema = resolve(root, schema);
    let mut all = vec![schema];
    for keyword in ["oneOf", "anyOf", "allOf"] {
        for member in schema
            .get(keyword)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            all.extend(alternatives(root, member));
        }
    }
    all
}

fn types(root: &Value, schema: &Value) -> Vec<String> {
    alternatives(root, schema)
        .into_iter()
        .filter_map(|schema| schema.get("type"))
        .flat_map(|type_| match type_ {
            Value::String(type_) => vec![type_.clone()],
            Value::Array(types) => types
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect(),
            _ => Vec::new(),
        })
        .filter(|type_| type_ != "null")
        .collect()
}

fn nullable(root: &Value, schema: &Value) -> bool {
    alternatives(root, schema)
        .into_iter()
        .any(|schema| match schema.get("type") {
            Some(Value::String(type_)) => type_ == "null",
            Some(Value::Array(types)) => types.contains(&json!("null")),
            _ => false,
        })
}

fn fill_required(root: &Value, schema: &Value, value: &mut Value) {
    let alternatives = alternatives(root, schema);
    match value {
        Value::Object(object) => {
            for alternative in &alternatives {
                let properties = alternative.get("properties");
                for name in alternative
                    .get("required")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                {
                    if !object.contains_key(name) {
                        let property = properties.and_then(|properties| properties.get(name));
                        object.insert(
                            name.to_owned(),
                            property.map_or(Value::Null, |property| placeholder(root, property)),
                        );
                    }
                }
            }
            for (name, member) in object.iter_mut() {
                for alternative in &alternatives {
                    if let Some(property) = alternative
                        .get("properties")
                        .and_then(|properties| properties.get(name))
                    {
                        fill_required(root, property, member);
                    }
                }
            }
        }
        Value::Array(items) => {
            for alternative in &alternatives {
                if let Some(item_schema) = alternative.get("items") {
                    for item in items.iter_mut() {
                        fill_required(root, item_schema, item);
                    }
                }
            }
        }
        _ => {}
    }
}

fn placeholder(root: &Value, schema: &Value) -> Value {
    let alternatives = alternatives(root, schema);
    if let Some(value) = alternatives.iter().find_map(|schema| {
        schema
            .get("enum")
            .and_then(Value::as_array)
            .and_then(|values| values.iter().find(|value| !value.is_null()))
            .or_else(|| schema.get("const"))
    }) {
        return value.clone();
    }
    let format = alternatives
        .iter()
        .find_map(|schema| schema.get("format").and_then(Value::as_str));
    let minimum = alternatives
        .iter()
        .find_map(|schema| schema.get("minimum").and_then(Value::as_f64));
    match types(root, schema).first().map(String::as_str) {
        Some("string") => json!(match format {
            Some("date-time") => "2026-01-01T00:00:00Z",
            Some("date") => "2026-01-01",
            Some("uuid") => "00000000-0000-0000-0000-000000000001",
            _ => fitting_string(&alternatives),
        }),
        #[allow(clippy::cast_possible_truncation)]
        Some("integer") => json!(minimum.map_or(1, |minimum| minimum.max(1.0) as i64)),
        Some("number") => json!(minimum.unwrap_or(1.0).max(1.0)),
        Some("boolean") => json!(false),
        Some("array") => json!([]),
        Some("object") => {
            let mut object = json!({});
            fill_required(root, schema, &mut object);
            object
        }
        _ => Value::Null,
    }
}

fn fitting_string(alternatives: &[&Value]) -> &'static str {
    const CANDIDATES: [&str; 6] = [
        "00000000000000000000000000000001",
        "fake",
        "00000000-0000-0000-0000-000000000001",
        "1",
        "fake.example.com",
        "fake@example.com",
    ];
    let Some(pattern) = alternatives
        .iter()
        .find_map(|schema| schema.get("pattern").and_then(Value::as_str))
        .and_then(|pattern| regress::Regex::new(pattern).ok())
    else {
        return CANDIDATES[0];
    };
    CANDIDATES
        .into_iter()
        .find(|candidate| pattern.find(candidate).is_some())
        .unwrap_or(CANDIDATES[0])
}

fn is_integer(schema: &schemars::Schema, field: &str) -> bool {
    let root = schema.as_value();
    alternatives(root, root)
        .into_iter()
        .filter_map(|object| object.get("properties")?.get(field))
        .any(|property| types(root, property).iter().any(|type_| type_ == "integer"))
}

fn top_level_fields(schema: &schemars::Schema) -> Vec<(String, Value)> {
    let root = schema.as_value();
    let mut fields: Vec<(String, Value)> = alternatives(root, root)
        .into_iter()
        .filter_map(|object| object.get("properties").and_then(Value::as_object))
        .flatten()
        .map(|(name, property)| {
            let unset = if nullable(root, property) {
                Value::Null
            } else {
                match types(root, property).first().map(String::as_str) {
                    Some("array") => json!([]),
                    Some("object") => json!({}),
                    _ => Value::Null,
                }
            };
            (name.clone(), unset)
        })
        .collect();
    fields.sort_by(|a, b| a.0.cmp(&b.0));
    fields.dedup_by(|a, b| a.0 == b.0);
    fields
}

fn is_parameter(segment: &str) -> bool {
    segment.starts_with('{') && segment.ends_with('}')
}

fn is_external_name(expression: &ValueExpr) -> bool {
    match expression {
        ValueExpr::ExternalName => true,
        ValueExpr::Coalesce(values) => values.iter().any(is_external_name),
        _ => false,
    }
}

const EXTERNAL_NAME: &str = "external name";

fn expression_key(expression: &ValueExpr) -> String {
    if is_external_name(expression) {
        EXTERNAL_NAME.to_owned()
    } else {
        format!("{expression:?}")
    }
}

impl Route {
    fn matches(
        &self,
        method: &Method,
        segments: &[&str],
    ) -> Option<BTreeMap<&'static str, String>> {
        if *method != self.method || segments.len() != self.template.len() {
            return None;
        }
        let mut parameters = BTreeMap::new();
        for (template, segment) in self.template.iter().zip(segments) {
            if is_parameter(template) {
                if segment.is_empty() {
                    return None;
                }
                let name = &template[1..template.len() - 1];
                let value = urlencoding::decode(segment)
                    .map_or_else(|_| (*segment).to_owned(), std::borrow::Cow::into_owned);
                parameters.insert(name, value);
            } else if template != segment {
                return None;
            }
        }
        Some(parameters)
    }

    fn authorized(&self, headers: &HeaderMap) -> bool {
        let present = |name: &str, prefix: &str| {
            headers
                .get(name)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.strip_prefix(prefix))
                .is_some_and(|value| !value.trim().is_empty())
        };
        match self.auth {
            None => true,
            Some(AuthScheme::Bearer) => present("authorization", "Bearer "),
            Some(AuthScheme::Header(name)) => present(name, ""),
        }
    }

    fn key(
        &self,
        shape: &Shape,
        parameters: &BTreeMap<&'static str, String>,
        id: Option<&str>,
    ) -> Vec<String> {
        shape
            .key
            .iter()
            .map(|expression| match id {
                Some(id) if expression == EXTERNAL_NAME => id.to_owned(),
                _ => self
                    .parameters
                    .iter()
                    .find(|(candidate, _)| candidate == expression)
                    .and_then(|(_, name)| parameters.get(name))
                    .cloned()
                    .unwrap_or_default(),
            })
            .collect()
    }
}

async fn state(State(store): State<Arc<Store>>) -> Json<Value> {
    let objects = store
        .objects
        .lock()
        .map(|objects| objects.clone())
        .unwrap_or_default();
    let mut state: BTreeMap<String, Map<String, Value>> = BTreeMap::new();
    for ((kind, key), object) in objects {
        state
            .entry(kind.to_string())
            .or_default()
            .insert(key.join("/"), object);
    }
    Json(json!(state))
}

async fn handle(
    State(store): State<Arc<Store>>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let segments: Vec<&str> = uri.path().split('/').collect();
    let Some((route, parameters)) = store
        .fake
        .routes
        .iter()
        .find_map(|route| Some((route, route.matches(&method, &segments)?)))
    else {
        tracing::warn!(%method, path = uri.path(), "no fake route");
        return error(StatusCode::NOT_FOUND, "no such route in the fake API");
    };
    let response = if route.authorized(&headers) {
        store.apply(route, &parameters, &body)
    } else {
        error(StatusCode::UNAUTHORIZED, "missing credentials")
    };
    tracing::info!(
        %method,
        path = uri.path(),
        kind = %route.kind,
        lifecycle = ?route.lifecycle,
        status = response.status().as_u16(),
        "request"
    );
    response
}

fn error(status: StatusCode, message: &str) -> Response {
    (
        status,
        Json(json!({ "success": false, "errors": [{ "message": message }] })),
    )
        .into_response()
}

fn wrap(path: &str, object: Value) -> Response {
    let mut body = Value::Null;
    let assigned = jsonptr::Pointer::parse(path)
        .map_err(|parse| parse.to_string())
        .and_then(|pointer| {
            pointer
                .assign(&mut body, object)
                .map_err(|assign| assign.to_string())
        });
    if let Err(message) = assigned {
        return error(StatusCode::INTERNAL_SERVER_ERROR, &message);
    }
    (StatusCode::OK, Json(body)).into_response()
}

fn insert(object: &mut Value, field: &str, value: Value) {
    if let Some(object) = object.as_object_mut() {
        object.insert(field.to_owned(), value);
    }
}

impl Store {
    fn apply(
        &self,
        route: &Route,
        parameters: &BTreeMap<&'static str, String>,
        body: &[u8],
    ) -> Response {
        let Some(shape) = self.fake.shapes.get(&route.kind) else {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "unregistered kind");
        };
        let body: Value = if body.is_empty() {
            Value::Object(Map::new())
        } else {
            match serde_json::from_slice(body) {
                Ok(body) => body,
                Err(error_) => return error(StatusCode::BAD_REQUEST, &error_.to_string()),
            }
        };
        let Ok(mut objects) = self.objects.lock() else {
            return error(StatusCode::INTERNAL_SERVER_ERROR, "store poisoned");
        };
        let key = (route.kind, route.key(shape, parameters, None));

        match route.lifecycle {
            Lifecycle::Create => {
                let mut object = body;
                let id = match object.get(shape.id_field) {
                    Some(Value::String(id)) if !id.is_empty() => id.clone(),
                    Some(Value::Number(id)) => id.to_string(),
                    _ => {
                        let next = self.next_id.fetch_add(1, Ordering::Relaxed);
                        if shape.numeric_id {
                            next.to_string()
                        } else {
                            format!("{next:032x}")
                        }
                    }
                };
                let id_value = match id.parse::<u64>() {
                    Ok(number) if shape.numeric_id => json!(number),
                    _ => Value::String(id.clone()),
                };
                insert(&mut object, shape.id_field, id_value);
                shape.complete(&mut object);
                let key = (route.kind, route.key(shape, parameters, Some(&id)));
                if objects.contains_key(&key) {
                    return error(StatusCode::CONFLICT, "already exists");
                }
                objects.insert(key, object.clone());
                wrap(shape.create_path, object)
            }
            Lifecycle::Observe => match objects.get(&key) {
                Some(object) => wrap(shape.response_path, object.clone()),
                None if !shape.creatable => {
                    let mut object = Value::Object(Map::new());
                    shape.complete(&mut object);
                    wrap(shape.response_path, object)
                }
                None => error(StatusCode::NOT_FOUND, "not found"),
            },
            Lifecycle::Update => {
                let object = match objects.get_mut(&key) {
                    Some(object) => object,
                    None if !shape.creatable => objects.entry(key).or_insert_with(|| json!({})),
                    None => return error(StatusCode::NOT_FOUND, "not found"),
                };
                if route.method == Method::PATCH {
                    merge(object, body);
                } else {
                    let id = object.get(shape.id_field).cloned();
                    *object = body;
                    if let Some(id) = id {
                        insert(object, shape.id_field, id);
                    }
                    shape.complete(object);
                }
                wrap(shape.response_path, object.clone())
            }
            Lifecycle::Delete => match objects.remove(&key) {
                Some(object) => {
                    let id = object.get(shape.id_field).cloned().unwrap_or(Value::Null);
                    let mut deleted = Map::new();
                    deleted.insert(shape.id_field.to_owned(), id);
                    wrap(shape.response_path, Value::Object(deleted))
                }
                None => error(StatusCode::NOT_FOUND, "not found"),
            },
        }
    }
}

fn merge(target: &mut Value, patch: Value) {
    let Value::Object(patch) = patch else {
        *target = patch;
        return;
    };
    if !target.is_object() {
        *target = Value::Object(Map::new());
    }
    let Value::Object(target) = target else {
        return;
    };
    for (key, value) in patch {
        if value.is_null() {
            target.remove(&key);
        } else {
            merge(target.entry(key).or_insert(Value::Null), value);
        }
    }
}
