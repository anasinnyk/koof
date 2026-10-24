#[derive(Debug, thiserror::Error)]
pub enum CredentialError {
    #[error("Kubernetes API request failed")]
    Kubernetes(#[from] kube::Error),

    #[error("{kind} has no credentials")]
    NoCredentials { kind: &'static str },

    #[error("credentials of {kind} do not end at a Secret")]
    NotSecret { kind: &'static str },

    #[error("credential field {path} is not set")]
    FieldNotSet { path: &'static str },

    #[error("Secret {secret} does not contain key {key:?}")]
    SecretKeyMissing { secret: String, key: String },

    #[error(transparent)]
    Resolve(#[from] ResolveValueError),

    #[error("Secret {secret:?} key {key:?} is not valid UTF-8")]
    InvalidSecretUtf8 {
        secret: String,
        key: String,
        #[source]
        source: std::string::FromUtf8Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ResolveValueError {
    #[error("failed to serialize resource")]
    Serialization(#[from] serde_json::Error),

    #[error("request field {path} could not be resolved")]
    RequestFieldNotSet { path: &'static str },

    #[error("{kind}: could not resolve path parameter {parameter}")]
    PathParameterNotSet {
        kind: &'static str,
        parameter: &'static str,
    },

    #[error("{kind} has no identifier {name}")]
    UnknownIdentifier {
        kind: &'static str,
        name: &'static str,
    },

    #[error("{kind} has no reference {name}")]
    UnknownReference { kind: &'static str, name: String },

    #[error("resource reference {from} is not set")]
    ReferenceNotSet { from: &'static str },

    #[error("resource reference {from} cannot point at kind {kind}")]
    ReferenceKind { from: &'static str, kind: String },

    #[error("cluster-scoped {kind} cannot reference namespaced {target}")]
    ClusterScopedReference {
        kind: &'static str,
        target: &'static str,
    },

    #[error("field {path:?} is not a scalar value")]
    NonScalar { path: String },

    #[error("reference not ready: {kind} referenced via {via} has no {identifier} yet")]
    ReferenceNotReady {
        kind: &'static str,
        via: String,
        identifier: &'static str,
    },

    #[error("Kubernetes API request failed")]
    Kubernetes(#[from] kube::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ReconcileError {
    #[error(transparent)]
    Credentials(#[from] CredentialError),

    #[error(transparent)]
    ResolveValue(#[from] ResolveValueError),

    #[error(transparent)]
    Http(#[from] crate::http::HttpError),

    #[error("JSON (de)serialization failed")]
    Serialization(#[from] serde_json::Error),

    #[error("Kubernetes API request failed")]
    Kubernetes(#[from] kube::Error),

    #[error(transparent)]
    Policy(#[from] crate::controller::policy::PolicyError),

    #[error("{kind} has no {lifecycle} operation")]
    Unsupported {
        kind: &'static str,
        lifecycle: &'static str,
    },

    #[error("request path {path} is not a valid JSON pointer")]
    RequestPath {
        path: &'static str,
        #[source]
        source: jsonptr::ParseError,
    },

    #[error("cannot set request field {path}")]
    RequestAssign {
        path: &'static str,
        #[source]
        source: jsonptr::assign::Error,
    },

    #[error("request body built from {from} does not match the API")]
    InvalidRequest {
        from: &'static str,
        #[source]
        source: serde_json::Error,
    },

    #[error(
        "a previous create may have succeeded ({pending} is set, no outcome recorded); if the \
         external resource exists set {external_name} to its ID, otherwise remove {pending}"
    )]
    CreatePending {
        pending: String,
        external_name: String,
    },

    #[error(
        "create succeeded but the response has no `{pointer}`; set the {annotation} annotation \
         to the new resource's ID"
    )]
    CreateNoIdentifier {
        pointer: &'static str,
        annotation: String,
    },

    #[error(
        "created external resource {external_name} but could not record it; set {annotation} \
         to {external_name}"
    )]
    CreateNotRecorded {
        external_name: String,
        annotation: String,
        #[source]
        source: Box<ReconcileError>,
    },
}

#[must_use]
pub fn chain(error: &dyn std::error::Error) -> String {
    let mut message = error.to_string();
    let mut source = error.source();
    while let Some(cause) = source {
        let cause_message = cause.to_string();
        if !message.contains(&cause_message) {
            message.push_str(": ");
            message.push_str(&cause_message);
        }
        source = cause.source();
    }
    message
}
