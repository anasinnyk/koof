//! This file is generated. Do not edit manually.
#![allow(
    dead_code,
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used
)]
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateRequest {
    ///The saved end time for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///The saved environment filter for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub environment: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ///The saved filters for this dashboard.
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub filters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///A dashboard's unique id.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    ///Favorite the dashboard automatically for the request user
    #[serde(default)]
    pub is_favorited: bool,
    ///The saved time range period for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub permissions: ::std::option::Option<DashboardCreateRequestPermissions>,
    ///The saved projects filter for this dashboard.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub projects: ::std::vec::Vec<i64>,
    ///The saved start time for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///The user defined title for this dashboard.
    pub title: DashboardCreateRequestTitle,
    ///Setting that lets you display saved time range for this dashboard in UTC.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub utc: ::std::option::Option<bool>,
    ///A json list of widgets saved in this dashboard.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub widgets: ::std::vec::Vec<DashboardCreateRequestWidgetsItem>,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateRequestPermissions {
    ///Whether the dashboard is editable by everyone.
    pub is_editable_by_everyone: bool,
    ///List of team IDs that have edit access to a dashboard.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub teams_with_edit_access: ::std::vec::Vec<i64>,
}
///The user defined title for this dashboard.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DashboardCreateRequestTitle(::std::string::String);
impl ::std::ops::Deref for DashboardCreateRequestTitle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DashboardCreateRequestTitle> for ::std::string::String {
    fn from(value: DashboardCreateRequestTitle) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DashboardCreateRequestTitle {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DashboardCreateRequestTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DashboardCreateRequestTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DashboardCreateRequestTitle {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateRequestWidgetsItem {
    #[doc = "* `auto`\n* `dataMin`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub axis_range: ::std::option::Option<DashboardCreateRequestWidgetsItemAxisRange>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "* `line`\n* `area`\n* `bar`\n* `table`\n* `big_number`\n* `details`\n* `categorical_bar`\n* `wheel`\n* `rage_and_dead_clicks`\n* `server_tree`\n* `text`\n* `agents_traces_table`\n* `heatmap`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub display_type: ::std::option::Option<
        DashboardCreateRequestWidgetsItemDisplayType,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub interval: ::std::option::Option<DashboardCreateRequestWidgetsItemInterval>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub layout: ::std::option::Option<DashboardCreateRequestWidgetsItemLayout>,
    #[doc = "* `default`\n* `breakdown`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub legend_type: ::std::option::Option<DashboardCreateRequestWidgetsItemLegendType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub queries: ::std::vec::Vec<DashboardCreateRequestWidgetsItemQueriesItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub thresholds: ::std::option::Option<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<DashboardCreateRequestWidgetsItemTitle>,
    #[doc = "* `discover`\n* `issue`\n* `metrics`\n* `error-events`\n* `transaction-like`\n* `spans`\n* `logs`\n* `tracemetrics`\n* `preprod-app-size`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub widget_type: ::std::option::Option<DashboardCreateRequestWidgetsItemWidgetType>,
}
#[doc = "* `auto`\n* `dataMin`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum DashboardCreateRequestWidgetsItemAxisRange {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "dataMin")]
    DataMin,
}
impl ::std::fmt::Display for DashboardCreateRequestWidgetsItemAxisRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::DataMin => f.write_str("dataMin"),
        }
    }
}
impl ::std::str::FromStr for DashboardCreateRequestWidgetsItemAxisRange {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "dataMin" => Ok(Self::DataMin),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DashboardCreateRequestWidgetsItemAxisRange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardCreateRequestWidgetsItemAxisRange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "* `line`\n* `area`\n* `bar`\n* `table`\n* `big_number`\n* `details`\n* `categorical_bar`\n* `wheel`\n* `rage_and_dead_clicks`\n* `server_tree`\n* `text`\n* `agents_traces_table`\n* `heatmap`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum DashboardCreateRequestWidgetsItemDisplayType {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "area")]
    Area,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "big_number")]
    BigNumber,
    #[serde(rename = "details")]
    Details,
    #[serde(rename = "categorical_bar")]
    CategoricalBar,
    #[serde(rename = "wheel")]
    Wheel,
    #[serde(rename = "rage_and_dead_clicks")]
    RageAndDeadClicks,
    #[serde(rename = "server_tree")]
    ServerTree,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "agents_traces_table")]
    AgentsTracesTable,
    #[serde(rename = "heatmap")]
    Heatmap,
}
impl ::std::fmt::Display for DashboardCreateRequestWidgetsItemDisplayType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Line => f.write_str("line"),
            Self::Area => f.write_str("area"),
            Self::Bar => f.write_str("bar"),
            Self::Table => f.write_str("table"),
            Self::BigNumber => f.write_str("big_number"),
            Self::Details => f.write_str("details"),
            Self::CategoricalBar => f.write_str("categorical_bar"),
            Self::Wheel => f.write_str("wheel"),
            Self::RageAndDeadClicks => f.write_str("rage_and_dead_clicks"),
            Self::ServerTree => f.write_str("server_tree"),
            Self::Text => f.write_str("text"),
            Self::AgentsTracesTable => f.write_str("agents_traces_table"),
            Self::Heatmap => f.write_str("heatmap"),
        }
    }
}
impl ::std::str::FromStr for DashboardCreateRequestWidgetsItemDisplayType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "line" => Ok(Self::Line),
            "area" => Ok(Self::Area),
            "bar" => Ok(Self::Bar),
            "table" => Ok(Self::Table),
            "big_number" => Ok(Self::BigNumber),
            "details" => Ok(Self::Details),
            "categorical_bar" => Ok(Self::CategoricalBar),
            "wheel" => Ok(Self::Wheel),
            "rage_and_dead_clicks" => Ok(Self::RageAndDeadClicks),
            "server_tree" => Ok(Self::ServerTree),
            "text" => Ok(Self::Text),
            "agents_traces_table" => Ok(Self::AgentsTracesTable),
            "heatmap" => Ok(Self::Heatmap),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DashboardCreateRequestWidgetsItemDisplayType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardCreateRequestWidgetsItemDisplayType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DashboardCreateRequestWidgetsItemInterval`
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DashboardCreateRequestWidgetsItemInterval(::std::string::String);
impl ::std::ops::Deref for DashboardCreateRequestWidgetsItemInterval {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DashboardCreateRequestWidgetsItemInterval>
for ::std::string::String {
    fn from(value: DashboardCreateRequestWidgetsItemInterval) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DashboardCreateRequestWidgetsItemInterval {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10usize {
            return Err("longer than 10 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DashboardCreateRequestWidgetsItemInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardCreateRequestWidgetsItemInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DashboardCreateRequestWidgetsItemInterval {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Widget grid layout position and dimensions for dashboard creation.

The dashboard uses a 6-column grid. Required keys: x, y, w, h.
Constraints: x (0-5), y (>= 0), w (1-6), h (>= 1), and x + w <= 6.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateRequestWidgetsItemLayout {
    ///Height in grid rows.
    pub h: ::std::num::NonZeroU64,
    ///Width in grid columns (1-6).
    pub w: ::std::num::NonZeroU64,
    ///Column position (0-indexed).
    pub x: i64,
    ///Row position (0-indexed).
    pub y: u64,
}
#[doc = "* `default`\n* `breakdown`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum DashboardCreateRequestWidgetsItemLegendType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "breakdown")]
    Breakdown,
}
impl ::std::fmt::Display for DashboardCreateRequestWidgetsItemLegendType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("default"),
            Self::Breakdown => f.write_str("breakdown"),
        }
    }
}
impl ::std::str::FromStr for DashboardCreateRequestWidgetsItemLegendType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "breakdown" => Ok(Self::Breakdown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DashboardCreateRequestWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardCreateRequestWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateRequestWidgetsItemQueriesItem {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub aggregates: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub columns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub conditions: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub field_aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub fields: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_hidden: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub linked_dashboards: ::std::option::Option<
        ::std::vec::Vec<DashboardCreateRequestWidgetsItemQueriesItemLinkedDashboardsItem>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub on_demand_extraction: ::std::option::Option<
        DashboardCreateRequestWidgetsItemQueriesItemOnDemandExtraction,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub on_demand_extraction_disabled: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub orderby: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub selected_aggregate: ::std::option::Option<i64>,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateRequestWidgetsItemQueriesItemLinkedDashboardsItem {
    pub dashboard_id: ::std::string::String,
    pub field: ::std::string::String,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateRequestWidgetsItemQueriesItemOnDemandExtraction {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub extraction_state: ::std::option::Option<::std::string::String>,
}
///`DashboardCreateRequestWidgetsItemTitle`
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DashboardCreateRequestWidgetsItemTitle(::std::string::String);
impl ::std::ops::Deref for DashboardCreateRequestWidgetsItemTitle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DashboardCreateRequestWidgetsItemTitle>
for ::std::string::String {
    fn from(value: DashboardCreateRequestWidgetsItemTitle) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DashboardCreateRequestWidgetsItemTitle {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DashboardCreateRequestWidgetsItemTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardCreateRequestWidgetsItemTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DashboardCreateRequestWidgetsItemTitle {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "* `discover`\n* `issue`\n* `metrics`\n* `error-events`\n* `transaction-like`\n* `spans`\n* `logs`\n* `tracemetrics`\n* `preprod-app-size`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum DashboardCreateRequestWidgetsItemWidgetType {
    #[serde(rename = "discover")]
    Discover,
    #[serde(rename = "issue")]
    Issue,
    #[serde(rename = "metrics")]
    Metrics,
    #[serde(rename = "error-events")]
    ErrorEvents,
    #[serde(rename = "transaction-like")]
    TransactionLike,
    #[serde(rename = "spans")]
    Spans,
    #[serde(rename = "logs")]
    Logs,
    #[serde(rename = "tracemetrics")]
    Tracemetrics,
    #[serde(rename = "preprod-app-size")]
    PreprodAppSize,
}
impl ::std::fmt::Display for DashboardCreateRequestWidgetsItemWidgetType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Discover => f.write_str("discover"),
            Self::Issue => f.write_str("issue"),
            Self::Metrics => f.write_str("metrics"),
            Self::ErrorEvents => f.write_str("error-events"),
            Self::TransactionLike => f.write_str("transaction-like"),
            Self::Spans => f.write_str("spans"),
            Self::Logs => f.write_str("logs"),
            Self::Tracemetrics => f.write_str("tracemetrics"),
            Self::PreprodAppSize => f.write_str("preprod-app-size"),
        }
    }
}
impl ::std::str::FromStr for DashboardCreateRequestWidgetsItemWidgetType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "discover" => Ok(Self::Discover),
            "issue" => Ok(Self::Issue),
            "metrics" => Ok(Self::Metrics),
            "error-events" => Ok(Self::ErrorEvents),
            "transaction-like" => Ok(Self::TransactionLike),
            "spans" => Ok(Self::Spans),
            "logs" => Ok(Self::Logs),
            "tracemetrics" => Ok(Self::Tracemetrics),
            "preprod-app-size" => Ok(Self::PreprodAppSize),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DashboardCreateRequestWidgetsItemWidgetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardCreateRequestWidgetsItemWidgetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DashboardCreateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponse {
    #[serde(
        rename = "createdBy",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub created_by: ::std::option::Option<DashboardCreateResponseCreatedBy>,
    #[serde(rename = "dateCreated")]
    pub date_created: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub environment: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expired: ::std::option::Option<bool>,
    pub filters: DashboardCreateResponseFilters,
    pub id: ::std::string::String,
    #[serde(rename = "isFavorited")]
    pub is_favorited: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub permissions: ::std::option::Option<DashboardCreateResponsePermissions>,
    #[serde(
        rename = "prebuiltId",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub prebuilt_id: ::std::option::Option<i64>,
    pub projects: ::std::vec::Vec<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub title: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub utc: ::std::option::Option<::std::string::String>,
    pub widgets: ::std::vec::Vec<DashboardCreateResponseWidgetsItem>,
}
///`DashboardCreateResponseCreatedBy`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseCreatedBy {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub authenticators: ::std::vec::Vec<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<DashboardCreateResponseCreatedByAvatar>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: ::std::string::String,
    #[serde(
        rename = "canReset2fa",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub can_reset2fa: ::std::option::Option<bool>,
    #[serde(rename = "dateJoined")]
    pub date_joined: ::chrono::DateTime<::chrono::offset::Utc>,
    pub email: ::std::string::String,
    pub emails: ::std::vec::Vec<DashboardCreateResponseCreatedByEmailsItem>,
    pub experiments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub has2fa: bool,
    #[serde(rename = "hasPasswordAuth")]
    pub has_password_auth: bool,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub identities: ::std::vec::Vec<DashboardCreateResponseCreatedByIdentitiesItem>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    #[serde(rename = "isStaff")]
    pub is_staff: bool,
    #[serde(rename = "isSuperuser")]
    pub is_superuser: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(
        rename = "lastActive",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_active: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "lastLogin",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_login: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub name: ::std::string::String,
    pub username: ::std::string::String,
}
///`DashboardCreateResponseCreatedByAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseCreatedByAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`DashboardCreateResponseCreatedByEmailsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseCreatedByEmailsItem {
    pub email: ::std::string::String,
    pub id: ::std::string::String,
    pub is_verified: bool,
}
///`DashboardCreateResponseCreatedByIdentitiesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseCreatedByIdentitiesItem {
    #[serde(rename = "dateSynced")]
    pub date_synced: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "dateVerified")]
    pub date_verified: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub organization: DashboardCreateResponseCreatedByIdentitiesItemOrganization,
    pub provider: DashboardCreateResponseCreatedByIdentitiesItemProvider,
}
///`DashboardCreateResponseCreatedByIdentitiesItemOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseCreatedByIdentitiesItemOrganization {
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`DashboardCreateResponseCreatedByIdentitiesItemProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseCreatedByIdentitiesItemProvider {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`DashboardCreateResponseFilters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseFilters {
    #[serde(
        rename = "globalFilter",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub global_filter: ::std::vec::Vec<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub release: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "releaseId",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub release_id: ::std::vec::Vec<::std::string::String>,
}
///`DashboardCreateResponsePermissions`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponsePermissions {
    #[serde(rename = "isEditableByEveryone")]
    pub is_editable_by_everyone: bool,
    #[serde(rename = "teamsWithEditAccess")]
    pub teams_with_edit_access: ::std::vec::Vec<i64>,
}
///`DashboardCreateResponseWidgetsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseWidgetsItem {
    #[serde(
        rename = "axisRange",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub axis_range: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "changedReason",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub changed_reason: ::std::option::Option<
        ::std::vec::Vec<DashboardCreateResponseWidgetsItemChangedReasonItem>,
    >,
    #[serde(rename = "dashboardId")]
    pub dashboard_id: ::std::string::String,
    #[serde(
        rename = "datasetSource",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub dataset_source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dateCreated")]
    pub date_created: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayType")]
    pub display_type: ::std::string::String,
    #[serde(
        rename = "exploreUrls",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub explore_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub id: ::std::string::String,
    pub interval: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub layout: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, i64>,
    >,
    #[serde(
        rename = "legendType",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub legend_type: ::std::option::Option<DashboardCreateResponseWidgetsItemLegendType>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub limit: ::std::option::Option<i64>,
    pub queries: ::std::vec::Vec<DashboardCreateResponseWidgetsItemQueriesItem>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub thresholds: ::std::option::Option<DashboardCreateResponseWidgetsItemThresholds>,
    pub title: ::std::string::String,
    #[serde(
        rename = "widgetType",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub widget_type: ::std::option::Option<::std::string::String>,
}
///`DashboardCreateResponseWidgetsItemChangedReasonItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseWidgetsItemChangedReasonItem {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub equations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::collections::HashMap<
                ::std::string::String,
                DashboardCreateResponseWidgetsItemChangedReasonItemEquationsItemValue,
            >,
        >,
    >,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub orderby: ::std::option::Option<
        ::std::vec::Vec<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
    pub selected_columns: ::std::vec::Vec<::std::string::String>,
}
///`DashboardCreateResponseWidgetsItemChangedReasonItemEquationsItemValue`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum DashboardCreateResponseWidgetsItemChangedReasonItemEquationsItemValue {
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>>
for DashboardCreateResponseWidgetsItemChangedReasonItemEquationsItemValue {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
///`DashboardCreateResponseWidgetsItemLegendType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum DashboardCreateResponseWidgetsItemLegendType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "breakdown")]
    Breakdown,
}
impl ::std::fmt::Display for DashboardCreateResponseWidgetsItemLegendType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("default"),
            Self::Breakdown => f.write_str("breakdown"),
        }
    }
}
impl ::std::str::FromStr for DashboardCreateResponseWidgetsItemLegendType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "breakdown" => Ok(Self::Breakdown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DashboardCreateResponseWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardCreateResponseWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DashboardCreateResponseWidgetsItemQueriesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseWidgetsItemQueriesItem {
    pub aggregates: ::std::vec::Vec<::std::string::String>,
    pub columns: ::std::vec::Vec<::std::string::String>,
    pub conditions: ::std::string::String,
    #[serde(rename = "fieldAliases")]
    pub field_aliases: ::std::vec::Vec<::std::string::String>,
    pub fields: ::std::vec::Vec<::std::string::String>,
    pub id: ::std::string::String,
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    #[serde(rename = "linkedDashboards")]
    pub linked_dashboards: ::std::vec::Vec<
        DashboardCreateResponseWidgetsItemQueriesItemLinkedDashboardsItem,
    >,
    pub name: ::std::string::String,
    #[serde(rename = "onDemand")]
    pub on_demand: ::std::vec::Vec<
        DashboardCreateResponseWidgetsItemQueriesItemOnDemandItem,
    >,
    pub orderby: ::std::string::String,
    #[serde(
        rename = "selectedAggregate",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub selected_aggregate: ::std::option::Option<i64>,
    #[serde(rename = "widgetId")]
    pub widget_id: ::std::string::String,
}
///`DashboardCreateResponseWidgetsItemQueriesItemLinkedDashboardsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseWidgetsItemQueriesItemLinkedDashboardsItem {
    #[serde(rename = "dashboardId")]
    pub dashboard_id: i64,
    pub field: ::std::string::String,
}
///`DashboardCreateResponseWidgetsItemQueriesItemOnDemandItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseWidgetsItemQueriesItemOnDemandItem {
    #[serde(rename = "dashboardWidgetQueryId")]
    pub dashboard_widget_query_id: i64,
    pub enabled: bool,
    #[serde(rename = "extractionState")]
    pub extraction_state: ::std::string::String,
}
///`DashboardCreateResponseWidgetsItemThresholds`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardCreateResponseWidgetsItemThresholds {
    pub max_values: ::std::collections::HashMap<::std::string::String, i64>,
    #[serde(
        rename = "preferredPolarity",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preferred_polarity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "timeWindow",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub time_window: ::std::option::Option<::std::string::String>,
    pub unit: ::std::string::String,
}
///`DashboardObserveResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponse {
    #[serde(
        rename = "createdBy",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub created_by: ::std::option::Option<DashboardObserveResponseCreatedBy>,
    #[serde(rename = "dateCreated")]
    pub date_created: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub environment: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expired: ::std::option::Option<bool>,
    pub filters: DashboardObserveResponseFilters,
    pub id: ::std::string::String,
    #[serde(rename = "isFavorited")]
    pub is_favorited: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub permissions: ::std::option::Option<DashboardObserveResponsePermissions>,
    #[serde(
        rename = "prebuiltId",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub prebuilt_id: ::std::option::Option<i64>,
    pub projects: ::std::vec::Vec<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub title: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub utc: ::std::option::Option<::std::string::String>,
    pub widgets: ::std::vec::Vec<DashboardObserveResponseWidgetsItem>,
}
///`DashboardObserveResponseCreatedBy`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseCreatedBy {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub authenticators: ::std::vec::Vec<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<DashboardObserveResponseCreatedByAvatar>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: ::std::string::String,
    #[serde(
        rename = "canReset2fa",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub can_reset2fa: ::std::option::Option<bool>,
    #[serde(rename = "dateJoined")]
    pub date_joined: ::chrono::DateTime<::chrono::offset::Utc>,
    pub email: ::std::string::String,
    pub emails: ::std::vec::Vec<DashboardObserveResponseCreatedByEmailsItem>,
    pub experiments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub has2fa: bool,
    #[serde(rename = "hasPasswordAuth")]
    pub has_password_auth: bool,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub identities: ::std::vec::Vec<DashboardObserveResponseCreatedByIdentitiesItem>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    #[serde(rename = "isStaff")]
    pub is_staff: bool,
    #[serde(rename = "isSuperuser")]
    pub is_superuser: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(
        rename = "lastActive",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_active: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "lastLogin",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_login: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub name: ::std::string::String,
    pub username: ::std::string::String,
}
///`DashboardObserveResponseCreatedByAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseCreatedByAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`DashboardObserveResponseCreatedByEmailsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseCreatedByEmailsItem {
    pub email: ::std::string::String,
    pub id: ::std::string::String,
    pub is_verified: bool,
}
///`DashboardObserveResponseCreatedByIdentitiesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseCreatedByIdentitiesItem {
    #[serde(rename = "dateSynced")]
    pub date_synced: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "dateVerified")]
    pub date_verified: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub organization: DashboardObserveResponseCreatedByIdentitiesItemOrganization,
    pub provider: DashboardObserveResponseCreatedByIdentitiesItemProvider,
}
///`DashboardObserveResponseCreatedByIdentitiesItemOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseCreatedByIdentitiesItemOrganization {
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`DashboardObserveResponseCreatedByIdentitiesItemProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseCreatedByIdentitiesItemProvider {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`DashboardObserveResponseFilters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseFilters {
    #[serde(
        rename = "globalFilter",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub global_filter: ::std::vec::Vec<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub release: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "releaseId",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub release_id: ::std::vec::Vec<::std::string::String>,
}
///`DashboardObserveResponsePermissions`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponsePermissions {
    #[serde(rename = "isEditableByEveryone")]
    pub is_editable_by_everyone: bool,
    #[serde(rename = "teamsWithEditAccess")]
    pub teams_with_edit_access: ::std::vec::Vec<i64>,
}
///`DashboardObserveResponseWidgetsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseWidgetsItem {
    #[serde(
        rename = "axisRange",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub axis_range: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "changedReason",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub changed_reason: ::std::option::Option<
        ::std::vec::Vec<DashboardObserveResponseWidgetsItemChangedReasonItem>,
    >,
    #[serde(rename = "dashboardId")]
    pub dashboard_id: ::std::string::String,
    #[serde(
        rename = "datasetSource",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub dataset_source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dateCreated")]
    pub date_created: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayType")]
    pub display_type: ::std::string::String,
    #[serde(
        rename = "exploreUrls",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub explore_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub id: ::std::string::String,
    pub interval: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub layout: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, i64>,
    >,
    #[serde(
        rename = "legendType",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub legend_type: ::std::option::Option<
        DashboardObserveResponseWidgetsItemLegendType,
    >,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub limit: ::std::option::Option<i64>,
    pub queries: ::std::vec::Vec<DashboardObserveResponseWidgetsItemQueriesItem>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub thresholds: ::std::option::Option<DashboardObserveResponseWidgetsItemThresholds>,
    pub title: ::std::string::String,
    #[serde(
        rename = "widgetType",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub widget_type: ::std::option::Option<::std::string::String>,
}
///`DashboardObserveResponseWidgetsItemChangedReasonItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseWidgetsItemChangedReasonItem {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub equations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::collections::HashMap<
                ::std::string::String,
                DashboardObserveResponseWidgetsItemChangedReasonItemEquationsItemValue,
            >,
        >,
    >,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub orderby: ::std::option::Option<
        ::std::vec::Vec<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
    pub selected_columns: ::std::vec::Vec<::std::string::String>,
}
///`DashboardObserveResponseWidgetsItemChangedReasonItemEquationsItemValue`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum DashboardObserveResponseWidgetsItemChangedReasonItemEquationsItemValue {
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>>
for DashboardObserveResponseWidgetsItemChangedReasonItemEquationsItemValue {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
///`DashboardObserveResponseWidgetsItemLegendType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum DashboardObserveResponseWidgetsItemLegendType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "breakdown")]
    Breakdown,
}
impl ::std::fmt::Display for DashboardObserveResponseWidgetsItemLegendType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("default"),
            Self::Breakdown => f.write_str("breakdown"),
        }
    }
}
impl ::std::str::FromStr for DashboardObserveResponseWidgetsItemLegendType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "breakdown" => Ok(Self::Breakdown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DashboardObserveResponseWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardObserveResponseWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DashboardObserveResponseWidgetsItemQueriesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseWidgetsItemQueriesItem {
    pub aggregates: ::std::vec::Vec<::std::string::String>,
    pub columns: ::std::vec::Vec<::std::string::String>,
    pub conditions: ::std::string::String,
    #[serde(rename = "fieldAliases")]
    pub field_aliases: ::std::vec::Vec<::std::string::String>,
    pub fields: ::std::vec::Vec<::std::string::String>,
    pub id: ::std::string::String,
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    #[serde(rename = "linkedDashboards")]
    pub linked_dashboards: ::std::vec::Vec<
        DashboardObserveResponseWidgetsItemQueriesItemLinkedDashboardsItem,
    >,
    pub name: ::std::string::String,
    #[serde(rename = "onDemand")]
    pub on_demand: ::std::vec::Vec<
        DashboardObserveResponseWidgetsItemQueriesItemOnDemandItem,
    >,
    pub orderby: ::std::string::String,
    #[serde(
        rename = "selectedAggregate",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub selected_aggregate: ::std::option::Option<i64>,
    #[serde(rename = "widgetId")]
    pub widget_id: ::std::string::String,
}
///`DashboardObserveResponseWidgetsItemQueriesItemLinkedDashboardsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseWidgetsItemQueriesItemLinkedDashboardsItem {
    #[serde(rename = "dashboardId")]
    pub dashboard_id: i64,
    pub field: ::std::string::String,
}
///`DashboardObserveResponseWidgetsItemQueriesItemOnDemandItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseWidgetsItemQueriesItemOnDemandItem {
    #[serde(rename = "dashboardWidgetQueryId")]
    pub dashboard_widget_query_id: i64,
    pub enabled: bool,
    #[serde(rename = "extractionState")]
    pub extraction_state: ::std::string::String,
}
///`DashboardObserveResponseWidgetsItemThresholds`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardObserveResponseWidgetsItemThresholds {
    pub max_values: ::std::collections::HashMap<::std::string::String, i64>,
    #[serde(
        rename = "preferredPolarity",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preferred_polarity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "timeWindow",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub time_window: ::std::option::Option<::std::string::String>,
    pub unit: ::std::string::String,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateRequest {
    ///The saved end time for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///The saved environment filter for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub environment: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ///The saved filters for this dashboard.
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub filters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///A dashboard's unique id.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    ///The saved time range period for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub permissions: ::std::option::Option<DashboardUpdateRequestPermissions>,
    ///The saved projects filter for this dashboard.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub projects: ::std::vec::Vec<i64>,
    ///The saved start time for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///The user-defined dashboard title.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<DashboardUpdateRequestTitle>,
    ///Setting that lets you display saved time range for this dashboard in UTC.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub utc: ::std::option::Option<bool>,
    ///A json list of widgets saved in this dashboard.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub widgets: ::std::vec::Vec<DashboardUpdateRequestWidgetsItem>,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateRequestPermissions {
    ///Whether the dashboard is editable by everyone.
    pub is_editable_by_everyone: bool,
    ///List of team IDs that have edit access to a dashboard.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub teams_with_edit_access: ::std::vec::Vec<i64>,
}
///The user-defined dashboard title.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DashboardUpdateRequestTitle(::std::string::String);
impl ::std::ops::Deref for DashboardUpdateRequestTitle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DashboardUpdateRequestTitle> for ::std::string::String {
    fn from(value: DashboardUpdateRequestTitle) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DashboardUpdateRequestTitle {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DashboardUpdateRequestTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DashboardUpdateRequestTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DashboardUpdateRequestTitle {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateRequestWidgetsItem {
    #[doc = "* `auto`\n* `dataMin`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub axis_range: ::std::option::Option<DashboardUpdateRequestWidgetsItemAxisRange>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "* `line`\n* `area`\n* `bar`\n* `table`\n* `big_number`\n* `details`\n* `categorical_bar`\n* `wheel`\n* `rage_and_dead_clicks`\n* `server_tree`\n* `text`\n* `agents_traces_table`\n* `heatmap`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub display_type: ::std::option::Option<
        DashboardUpdateRequestWidgetsItemDisplayType,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub interval: ::std::option::Option<DashboardUpdateRequestWidgetsItemInterval>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub layout: ::std::option::Option<DashboardUpdateRequestWidgetsItemLayout>,
    #[doc = "* `default`\n* `breakdown`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub legend_type: ::std::option::Option<DashboardUpdateRequestWidgetsItemLegendType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub queries: ::std::vec::Vec<DashboardUpdateRequestWidgetsItemQueriesItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub thresholds: ::std::option::Option<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<DashboardUpdateRequestWidgetsItemTitle>,
    #[doc = "* `discover`\n* `issue`\n* `metrics`\n* `error-events`\n* `transaction-like`\n* `spans`\n* `logs`\n* `tracemetrics`\n* `preprod-app-size`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub widget_type: ::std::option::Option<DashboardUpdateRequestWidgetsItemWidgetType>,
}
#[doc = "* `auto`\n* `dataMin`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum DashboardUpdateRequestWidgetsItemAxisRange {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "dataMin")]
    DataMin,
}
impl ::std::fmt::Display for DashboardUpdateRequestWidgetsItemAxisRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::DataMin => f.write_str("dataMin"),
        }
    }
}
impl ::std::str::FromStr for DashboardUpdateRequestWidgetsItemAxisRange {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "dataMin" => Ok(Self::DataMin),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DashboardUpdateRequestWidgetsItemAxisRange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardUpdateRequestWidgetsItemAxisRange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "* `line`\n* `area`\n* `bar`\n* `table`\n* `big_number`\n* `details`\n* `categorical_bar`\n* `wheel`\n* `rage_and_dead_clicks`\n* `server_tree`\n* `text`\n* `agents_traces_table`\n* `heatmap`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum DashboardUpdateRequestWidgetsItemDisplayType {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "area")]
    Area,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "big_number")]
    BigNumber,
    #[serde(rename = "details")]
    Details,
    #[serde(rename = "categorical_bar")]
    CategoricalBar,
    #[serde(rename = "wheel")]
    Wheel,
    #[serde(rename = "rage_and_dead_clicks")]
    RageAndDeadClicks,
    #[serde(rename = "server_tree")]
    ServerTree,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "agents_traces_table")]
    AgentsTracesTable,
    #[serde(rename = "heatmap")]
    Heatmap,
}
impl ::std::fmt::Display for DashboardUpdateRequestWidgetsItemDisplayType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Line => f.write_str("line"),
            Self::Area => f.write_str("area"),
            Self::Bar => f.write_str("bar"),
            Self::Table => f.write_str("table"),
            Self::BigNumber => f.write_str("big_number"),
            Self::Details => f.write_str("details"),
            Self::CategoricalBar => f.write_str("categorical_bar"),
            Self::Wheel => f.write_str("wheel"),
            Self::RageAndDeadClicks => f.write_str("rage_and_dead_clicks"),
            Self::ServerTree => f.write_str("server_tree"),
            Self::Text => f.write_str("text"),
            Self::AgentsTracesTable => f.write_str("agents_traces_table"),
            Self::Heatmap => f.write_str("heatmap"),
        }
    }
}
impl ::std::str::FromStr for DashboardUpdateRequestWidgetsItemDisplayType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "line" => Ok(Self::Line),
            "area" => Ok(Self::Area),
            "bar" => Ok(Self::Bar),
            "table" => Ok(Self::Table),
            "big_number" => Ok(Self::BigNumber),
            "details" => Ok(Self::Details),
            "categorical_bar" => Ok(Self::CategoricalBar),
            "wheel" => Ok(Self::Wheel),
            "rage_and_dead_clicks" => Ok(Self::RageAndDeadClicks),
            "server_tree" => Ok(Self::ServerTree),
            "text" => Ok(Self::Text),
            "agents_traces_table" => Ok(Self::AgentsTracesTable),
            "heatmap" => Ok(Self::Heatmap),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DashboardUpdateRequestWidgetsItemDisplayType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardUpdateRequestWidgetsItemDisplayType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DashboardUpdateRequestWidgetsItemInterval`
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DashboardUpdateRequestWidgetsItemInterval(::std::string::String);
impl ::std::ops::Deref for DashboardUpdateRequestWidgetsItemInterval {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DashboardUpdateRequestWidgetsItemInterval>
for ::std::string::String {
    fn from(value: DashboardUpdateRequestWidgetsItemInterval) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DashboardUpdateRequestWidgetsItemInterval {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10usize {
            return Err("longer than 10 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DashboardUpdateRequestWidgetsItemInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardUpdateRequestWidgetsItemInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DashboardUpdateRequestWidgetsItemInterval {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Widget grid layout position and dimensions.

The dashboard uses a 6-column grid. Required keys: x, y, w, h, minH.
Constraints: x (0-5), y (>= 0), w (1-6), h (>= 1), minH (>= 1), and x + w <= 6.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateRequestWidgetsItemLayout {
    ///Height in grid rows.
    pub h: ::std::num::NonZeroU64,
    ///Minimum height in grid rows.
    pub min_h: ::std::num::NonZeroU64,
    ///Width in grid columns (1-6).
    pub w: ::std::num::NonZeroU64,
    ///Column position (0-indexed).
    pub x: i64,
    ///Row position (0-indexed).
    pub y: u64,
}
#[doc = "* `default`\n* `breakdown`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum DashboardUpdateRequestWidgetsItemLegendType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "breakdown")]
    Breakdown,
}
impl ::std::fmt::Display for DashboardUpdateRequestWidgetsItemLegendType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("default"),
            Self::Breakdown => f.write_str("breakdown"),
        }
    }
}
impl ::std::str::FromStr for DashboardUpdateRequestWidgetsItemLegendType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "breakdown" => Ok(Self::Breakdown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DashboardUpdateRequestWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardUpdateRequestWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateRequestWidgetsItemQueriesItem {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub aggregates: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub columns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub conditions: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub field_aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub fields: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_hidden: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub linked_dashboards: ::std::option::Option<
        ::std::vec::Vec<DashboardUpdateRequestWidgetsItemQueriesItemLinkedDashboardsItem>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub on_demand_extraction: ::std::option::Option<
        DashboardUpdateRequestWidgetsItemQueriesItemOnDemandExtraction,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub on_demand_extraction_disabled: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub orderby: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub selected_aggregate: ::std::option::Option<i64>,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateRequestWidgetsItemQueriesItemLinkedDashboardsItem {
    pub dashboard_id: ::std::string::String,
    pub field: ::std::string::String,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateRequestWidgetsItemQueriesItemOnDemandExtraction {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub extraction_state: ::std::option::Option<::std::string::String>,
}
///`DashboardUpdateRequestWidgetsItemTitle`
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DashboardUpdateRequestWidgetsItemTitle(::std::string::String);
impl ::std::ops::Deref for DashboardUpdateRequestWidgetsItemTitle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DashboardUpdateRequestWidgetsItemTitle>
for ::std::string::String {
    fn from(value: DashboardUpdateRequestWidgetsItemTitle) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DashboardUpdateRequestWidgetsItemTitle {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DashboardUpdateRequestWidgetsItemTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardUpdateRequestWidgetsItemTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DashboardUpdateRequestWidgetsItemTitle {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "* `discover`\n* `issue`\n* `metrics`\n* `error-events`\n* `transaction-like`\n* `spans`\n* `logs`\n* `tracemetrics`\n* `preprod-app-size`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum DashboardUpdateRequestWidgetsItemWidgetType {
    #[serde(rename = "discover")]
    Discover,
    #[serde(rename = "issue")]
    Issue,
    #[serde(rename = "metrics")]
    Metrics,
    #[serde(rename = "error-events")]
    ErrorEvents,
    #[serde(rename = "transaction-like")]
    TransactionLike,
    #[serde(rename = "spans")]
    Spans,
    #[serde(rename = "logs")]
    Logs,
    #[serde(rename = "tracemetrics")]
    Tracemetrics,
    #[serde(rename = "preprod-app-size")]
    PreprodAppSize,
}
impl ::std::fmt::Display for DashboardUpdateRequestWidgetsItemWidgetType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Discover => f.write_str("discover"),
            Self::Issue => f.write_str("issue"),
            Self::Metrics => f.write_str("metrics"),
            Self::ErrorEvents => f.write_str("error-events"),
            Self::TransactionLike => f.write_str("transaction-like"),
            Self::Spans => f.write_str("spans"),
            Self::Logs => f.write_str("logs"),
            Self::Tracemetrics => f.write_str("tracemetrics"),
            Self::PreprodAppSize => f.write_str("preprod-app-size"),
        }
    }
}
impl ::std::str::FromStr for DashboardUpdateRequestWidgetsItemWidgetType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "discover" => Ok(Self::Discover),
            "issue" => Ok(Self::Issue),
            "metrics" => Ok(Self::Metrics),
            "error-events" => Ok(Self::ErrorEvents),
            "transaction-like" => Ok(Self::TransactionLike),
            "spans" => Ok(Self::Spans),
            "logs" => Ok(Self::Logs),
            "tracemetrics" => Ok(Self::Tracemetrics),
            "preprod-app-size" => Ok(Self::PreprodAppSize),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DashboardUpdateRequestWidgetsItemWidgetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardUpdateRequestWidgetsItemWidgetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DashboardUpdateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponse {
    #[serde(
        rename = "createdBy",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub created_by: ::std::option::Option<DashboardUpdateResponseCreatedBy>,
    #[serde(rename = "dateCreated")]
    pub date_created: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub environment: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expired: ::std::option::Option<bool>,
    pub filters: DashboardUpdateResponseFilters,
    pub id: ::std::string::String,
    #[serde(rename = "isFavorited")]
    pub is_favorited: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub permissions: ::std::option::Option<DashboardUpdateResponsePermissions>,
    #[serde(
        rename = "prebuiltId",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub prebuilt_id: ::std::option::Option<i64>,
    pub projects: ::std::vec::Vec<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub title: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub utc: ::std::option::Option<::std::string::String>,
    pub widgets: ::std::vec::Vec<DashboardUpdateResponseWidgetsItem>,
}
///`DashboardUpdateResponseCreatedBy`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseCreatedBy {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub authenticators: ::std::vec::Vec<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<DashboardUpdateResponseCreatedByAvatar>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: ::std::string::String,
    #[serde(
        rename = "canReset2fa",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub can_reset2fa: ::std::option::Option<bool>,
    #[serde(rename = "dateJoined")]
    pub date_joined: ::chrono::DateTime<::chrono::offset::Utc>,
    pub email: ::std::string::String,
    pub emails: ::std::vec::Vec<DashboardUpdateResponseCreatedByEmailsItem>,
    pub experiments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub has2fa: bool,
    #[serde(rename = "hasPasswordAuth")]
    pub has_password_auth: bool,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub identities: ::std::vec::Vec<DashboardUpdateResponseCreatedByIdentitiesItem>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    #[serde(rename = "isStaff")]
    pub is_staff: bool,
    #[serde(rename = "isSuperuser")]
    pub is_superuser: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(
        rename = "lastActive",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_active: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "lastLogin",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_login: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub name: ::std::string::String,
    pub username: ::std::string::String,
}
///`DashboardUpdateResponseCreatedByAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseCreatedByAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`DashboardUpdateResponseCreatedByEmailsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseCreatedByEmailsItem {
    pub email: ::std::string::String,
    pub id: ::std::string::String,
    pub is_verified: bool,
}
///`DashboardUpdateResponseCreatedByIdentitiesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseCreatedByIdentitiesItem {
    #[serde(rename = "dateSynced")]
    pub date_synced: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "dateVerified")]
    pub date_verified: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub organization: DashboardUpdateResponseCreatedByIdentitiesItemOrganization,
    pub provider: DashboardUpdateResponseCreatedByIdentitiesItemProvider,
}
///`DashboardUpdateResponseCreatedByIdentitiesItemOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseCreatedByIdentitiesItemOrganization {
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`DashboardUpdateResponseCreatedByIdentitiesItemProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseCreatedByIdentitiesItemProvider {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`DashboardUpdateResponseFilters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseFilters {
    #[serde(
        rename = "globalFilter",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub global_filter: ::std::vec::Vec<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub release: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "releaseId",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub release_id: ::std::vec::Vec<::std::string::String>,
}
///`DashboardUpdateResponsePermissions`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponsePermissions {
    #[serde(rename = "isEditableByEveryone")]
    pub is_editable_by_everyone: bool,
    #[serde(rename = "teamsWithEditAccess")]
    pub teams_with_edit_access: ::std::vec::Vec<i64>,
}
///`DashboardUpdateResponseWidgetsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseWidgetsItem {
    #[serde(
        rename = "axisRange",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub axis_range: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "changedReason",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub changed_reason: ::std::option::Option<
        ::std::vec::Vec<DashboardUpdateResponseWidgetsItemChangedReasonItem>,
    >,
    #[serde(rename = "dashboardId")]
    pub dashboard_id: ::std::string::String,
    #[serde(
        rename = "datasetSource",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub dataset_source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dateCreated")]
    pub date_created: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayType")]
    pub display_type: ::std::string::String,
    #[serde(
        rename = "exploreUrls",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub explore_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub id: ::std::string::String,
    pub interval: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub layout: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, i64>,
    >,
    #[serde(
        rename = "legendType",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub legend_type: ::std::option::Option<DashboardUpdateResponseWidgetsItemLegendType>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub limit: ::std::option::Option<i64>,
    pub queries: ::std::vec::Vec<DashboardUpdateResponseWidgetsItemQueriesItem>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub thresholds: ::std::option::Option<DashboardUpdateResponseWidgetsItemThresholds>,
    pub title: ::std::string::String,
    #[serde(
        rename = "widgetType",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub widget_type: ::std::option::Option<::std::string::String>,
}
///`DashboardUpdateResponseWidgetsItemChangedReasonItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseWidgetsItemChangedReasonItem {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub equations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::collections::HashMap<
                ::std::string::String,
                DashboardUpdateResponseWidgetsItemChangedReasonItemEquationsItemValue,
            >,
        >,
    >,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub orderby: ::std::option::Option<
        ::std::vec::Vec<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
    pub selected_columns: ::std::vec::Vec<::std::string::String>,
}
///`DashboardUpdateResponseWidgetsItemChangedReasonItemEquationsItemValue`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum DashboardUpdateResponseWidgetsItemChangedReasonItemEquationsItemValue {
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>>
for DashboardUpdateResponseWidgetsItemChangedReasonItemEquationsItemValue {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
///`DashboardUpdateResponseWidgetsItemLegendType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum DashboardUpdateResponseWidgetsItemLegendType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "breakdown")]
    Breakdown,
}
impl ::std::fmt::Display for DashboardUpdateResponseWidgetsItemLegendType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("default"),
            Self::Breakdown => f.write_str("breakdown"),
        }
    }
}
impl ::std::str::FromStr for DashboardUpdateResponseWidgetsItemLegendType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "breakdown" => Ok(Self::Breakdown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DashboardUpdateResponseWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DashboardUpdateResponseWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DashboardUpdateResponseWidgetsItemQueriesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseWidgetsItemQueriesItem {
    pub aggregates: ::std::vec::Vec<::std::string::String>,
    pub columns: ::std::vec::Vec<::std::string::String>,
    pub conditions: ::std::string::String,
    #[serde(rename = "fieldAliases")]
    pub field_aliases: ::std::vec::Vec<::std::string::String>,
    pub fields: ::std::vec::Vec<::std::string::String>,
    pub id: ::std::string::String,
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    #[serde(rename = "linkedDashboards")]
    pub linked_dashboards: ::std::vec::Vec<
        DashboardUpdateResponseWidgetsItemQueriesItemLinkedDashboardsItem,
    >,
    pub name: ::std::string::String,
    #[serde(rename = "onDemand")]
    pub on_demand: ::std::vec::Vec<
        DashboardUpdateResponseWidgetsItemQueriesItemOnDemandItem,
    >,
    pub orderby: ::std::string::String,
    #[serde(
        rename = "selectedAggregate",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub selected_aggregate: ::std::option::Option<i64>,
    #[serde(rename = "widgetId")]
    pub widget_id: ::std::string::String,
}
///`DashboardUpdateResponseWidgetsItemQueriesItemLinkedDashboardsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseWidgetsItemQueriesItemLinkedDashboardsItem {
    #[serde(rename = "dashboardId")]
    pub dashboard_id: i64,
    pub field: ::std::string::String,
}
///`DashboardUpdateResponseWidgetsItemQueriesItemOnDemandItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseWidgetsItemQueriesItemOnDemandItem {
    #[serde(rename = "dashboardWidgetQueryId")]
    pub dashboard_widget_query_id: i64,
    pub enabled: bool,
    #[serde(rename = "extractionState")]
    pub extraction_state: ::std::string::String,
}
///`DashboardUpdateResponseWidgetsItemThresholds`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DashboardUpdateResponseWidgetsItemThresholds {
    pub max_values: ::std::collections::HashMap<::std::string::String, i64>,
    #[serde(
        rename = "preferredPolarity",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preferred_polarity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "timeWindow",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub time_window: ::std::option::Option<::std::string::String>,
    pub unit: ::std::string::String,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateRequest {
    ///Alert rule configuration created alongside the monitor.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub alert_rule: ::std::option::Option<MonitorCreateRequestAlertRule>,
    ///The configuration for the monitor.
    pub config: MonitorCreateRequestConfig,
    ///Disable creation of monitor incidents
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_muted: ::std::option::Option<bool>,
    ///Name of the monitor. Used for notifications. If not set the slug will be derived from your monitor name.
    pub name: MonitorCreateRequestName,
    ///The ID of the team or user that owns the monitor. (eg. user:51 or team:6)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<::std::string::String>,
    ///The project ID or slug to associate the monitor to.
    pub project: ::std::string::String,
    ///Uniquely identifies your monitor within your organization. Changing this slug will require updates to any instrumented check-in calls.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<MonitorCreateRequestSlug>,
    /**Status of the monitor. Disabled monitors will not accept events and will not count towards the monitor quota.

* `active`
* `disabled`*/
    #[serde(default = "defaults::monitor_create_request_status")]
    pub status: MonitorCreateRequestStatus,
}
///`MonitorCreateRequestAlertRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateRequestAlertRule {
    ///Name of the environment
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub environment: ::std::option::Option<MonitorCreateRequestAlertRuleEnvironment>,
    ///Array of dictionaries with information of the user or team to be notified
    pub targets: ::std::vec::Vec<MonitorCreateRequestAlertRuleTargetsItem>,
}
///Name of the environment
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct MonitorCreateRequestAlertRuleEnvironment(::std::string::String);
impl ::std::ops::Deref for MonitorCreateRequestAlertRuleEnvironment {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MonitorCreateRequestAlertRuleEnvironment>
for ::std::string::String {
    fn from(value: MonitorCreateRequestAlertRuleEnvironment) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MonitorCreateRequestAlertRuleEnvironment {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MonitorCreateRequestAlertRuleEnvironment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for MonitorCreateRequestAlertRuleEnvironment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MonitorCreateRequestAlertRuleEnvironment {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`MonitorCreateRequestAlertRuleTargetsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateRequestAlertRuleTargetsItem {
    ///ID of target object
    pub target_identifier: i64,
    ///One of [Member, Team]
    pub target_type: ::std::string::String,
}
///`MonitorCreateRequestConfig`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateRequestConfig {
    ///How long (in minutes) after the expected checkin time will we wait until we consider the checkin to have been missed.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub checkin_margin: ::std::option::Option<::std::num::NonZeroU64>,
    ///How many consecutive missed or failed check-ins in a row before creating a new issue.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub failure_issue_threshold: ::std::option::Option<::std::num::NonZeroU64>,
    ///How long (in minutes) is the checkin allowed to run for in CheckInStatus.IN_PROGRESS before it is considered failed.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_runtime: ::std::option::Option<::std::num::NonZeroU64>,
    ///How many successful check-ins in a row before resolving an issue.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub recovery_threshold: ::std::option::Option<::std::num::NonZeroU64>,
    ///Varies depending on the schedule_type. Is either a crontab string, or a 2 element tuple for intervals (e.g. [1, 'day'])
    pub schedule: ::serde_json::Value,
    /**Currently supports "crontab" or "interval"

* `crontab`
* `interval`*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub schedule_type: ::std::option::Option<MonitorCreateRequestConfigScheduleType>,
    /**tz database style timezone string

* `Africa/Abidjan`
* `Africa/Accra`
* `Africa/Addis_Ababa`
* `Africa/Algiers`
* `Africa/Asmara`
* `Africa/Asmera`
* `Africa/Bamako`
* `Africa/Bangui`
* `Africa/Banjul`
* `Africa/Bissau`
* `Africa/Blantyre`
* `Africa/Brazzaville`
* `Africa/Bujumbura`
* `Africa/Cairo`
* `Africa/Casablanca`
* `Africa/Ceuta`
* `Africa/Conakry`
* `Africa/Dakar`
* `Africa/Dar_es_Salaam`
* `Africa/Djibouti`
* `Africa/Douala`
* `Africa/El_Aaiun`
* `Africa/Freetown`
* `Africa/Gaborone`
* `Africa/Harare`
* `Africa/Johannesburg`
* `Africa/Juba`
* `Africa/Kampala`
* `Africa/Khartoum`
* `Africa/Kigali`
* `Africa/Kinshasa`
* `Africa/Lagos`
* `Africa/Libreville`
* `Africa/Lome`
* `Africa/Luanda`
* `Africa/Lubumbashi`
* `Africa/Lusaka`
* `Africa/Malabo`
* `Africa/Maputo`
* `Africa/Maseru`
* `Africa/Mbabane`
* `Africa/Mogadishu`
* `Africa/Monrovia`
* `Africa/Nairobi`
* `Africa/Ndjamena`
* `Africa/Niamey`
* `Africa/Nouakchott`
* `Africa/Ouagadougou`
* `Africa/Porto-Novo`
* `Africa/Sao_Tome`
* `Africa/Timbuktu`
* `Africa/Tripoli`
* `Africa/Tunis`
* `Africa/Windhoek`
* `America/Adak`
* `America/Anchorage`
* `America/Anguilla`
* `America/Antigua`
* `America/Araguaina`
* `America/Argentina/Buenos_Aires`
* `America/Argentina/Catamarca`
* `America/Argentina/ComodRivadavia`
* `America/Argentina/Cordoba`
* `America/Argentina/Jujuy`
* `America/Argentina/La_Rioja`
* `America/Argentina/Mendoza`
* `America/Argentina/Rio_Gallegos`
* `America/Argentina/Salta`
* `America/Argentina/San_Juan`
* `America/Argentina/San_Luis`
* `America/Argentina/Tucuman`
* `America/Argentina/Ushuaia`
* `America/Aruba`
* `America/Asuncion`
* `America/Atikokan`
* `America/Atka`
* `America/Bahia`
* `America/Bahia_Banderas`
* `America/Barbados`
* `America/Belem`
* `America/Belize`
* `America/Blanc-Sablon`
* `America/Boa_Vista`
* `America/Bogota`
* `America/Boise`
* `America/Buenos_Aires`
* `America/Cambridge_Bay`
* `America/Campo_Grande`
* `America/Cancun`
* `America/Caracas`
* `America/Catamarca`
* `America/Cayenne`
* `America/Cayman`
* `America/Chicago`
* `America/Chihuahua`
* `America/Ciudad_Juarez`
* `America/Coral_Harbour`
* `America/Cordoba`
* `America/Costa_Rica`
* `America/Coyhaique`
* `America/Creston`
* `America/Cuiaba`
* `America/Curacao`
* `America/Danmarkshavn`
* `America/Dawson`
* `America/Dawson_Creek`
* `America/Denver`
* `America/Detroit`
* `America/Dominica`
* `America/Edmonton`
* `America/Eirunepe`
* `America/El_Salvador`
* `America/Ensenada`
* `America/Fort_Nelson`
* `America/Fort_Wayne`
* `America/Fortaleza`
* `America/Glace_Bay`
* `America/Godthab`
* `America/Goose_Bay`
* `America/Grand_Turk`
* `America/Grenada`
* `America/Guadeloupe`
* `America/Guatemala`
* `America/Guayaquil`
* `America/Guyana`
* `America/Halifax`
* `America/Havana`
* `America/Hermosillo`
* `America/Indiana/Indianapolis`
* `America/Indiana/Knox`
* `America/Indiana/Marengo`
* `America/Indiana/Petersburg`
* `America/Indiana/Tell_City`
* `America/Indiana/Vevay`
* `America/Indiana/Vincennes`
* `America/Indiana/Winamac`
* `America/Indianapolis`
* `America/Inuvik`
* `America/Iqaluit`
* `America/Jamaica`
* `America/Jujuy`
* `America/Juneau`
* `America/Kentucky/Louisville`
* `America/Kentucky/Monticello`
* `America/Knox_IN`
* `America/Kralendijk`
* `America/La_Paz`
* `America/Lima`
* `America/Los_Angeles`
* `America/Louisville`
* `America/Lower_Princes`
* `America/Maceio`
* `America/Managua`
* `America/Manaus`
* `America/Marigot`
* `America/Martinique`
* `America/Matamoros`
* `America/Mazatlan`
* `America/Mendoza`
* `America/Menominee`
* `America/Merida`
* `America/Metlakatla`
* `America/Mexico_City`
* `America/Miquelon`
* `America/Moncton`
* `America/Monterrey`
* `America/Montevideo`
* `America/Montreal`
* `America/Montserrat`
* `America/Nassau`
* `America/New_York`
* `America/Nipigon`
* `America/Nome`
* `America/Noronha`
* `America/North_Dakota/Beulah`
* `America/North_Dakota/Center`
* `America/North_Dakota/New_Salem`
* `America/Nuuk`
* `America/Ojinaga`
* `America/Panama`
* `America/Pangnirtung`
* `America/Paramaribo`
* `America/Phoenix`
* `America/Port-au-Prince`
* `America/Port_of_Spain`
* `America/Porto_Acre`
* `America/Porto_Velho`
* `America/Puerto_Rico`
* `America/Punta_Arenas`
* `America/Rainy_River`
* `America/Rankin_Inlet`
* `America/Recife`
* `America/Regina`
* `America/Resolute`
* `America/Rio_Branco`
* `America/Rosario`
* `America/Santa_Isabel`
* `America/Santarem`
* `America/Santiago`
* `America/Santo_Domingo`
* `America/Sao_Paulo`
* `America/Scoresbysund`
* `America/Shiprock`
* `America/Sitka`
* `America/St_Barthelemy`
* `America/St_Johns`
* `America/St_Kitts`
* `America/St_Lucia`
* `America/St_Thomas`
* `America/St_Vincent`
* `America/Swift_Current`
* `America/Tegucigalpa`
* `America/Thule`
* `America/Thunder_Bay`
* `America/Tijuana`
* `America/Toronto`
* `America/Tortola`
* `America/Vancouver`
* `America/Virgin`
* `America/Whitehorse`
* `America/Winnipeg`
* `America/Yakutat`
* `America/Yellowknife`
* `Antarctica/Casey`
* `Antarctica/Davis`
* `Antarctica/DumontDUrville`
* `Antarctica/Macquarie`
* `Antarctica/Mawson`
* `Antarctica/McMurdo`
* `Antarctica/Palmer`
* `Antarctica/Rothera`
* `Antarctica/South_Pole`
* `Antarctica/Syowa`
* `Antarctica/Troll`
* `Antarctica/Vostok`
* `Arctic/Longyearbyen`
* `Asia/Aden`
* `Asia/Almaty`
* `Asia/Amman`
* `Asia/Anadyr`
* `Asia/Aqtau`
* `Asia/Aqtobe`
* `Asia/Ashgabat`
* `Asia/Ashkhabad`
* `Asia/Atyrau`
* `Asia/Baghdad`
* `Asia/Bahrain`
* `Asia/Baku`
* `Asia/Bangkok`
* `Asia/Barnaul`
* `Asia/Beirut`
* `Asia/Bishkek`
* `Asia/Brunei`
* `Asia/Calcutta`
* `Asia/Chita`
* `Asia/Choibalsan`
* `Asia/Chongqing`
* `Asia/Chungking`
* `Asia/Colombo`
* `Asia/Dacca`
* `Asia/Damascus`
* `Asia/Dhaka`
* `Asia/Dili`
* `Asia/Dubai`
* `Asia/Dushanbe`
* `Asia/Famagusta`
* `Asia/Gaza`
* `Asia/Harbin`
* `Asia/Hebron`
* `Asia/Ho_Chi_Minh`
* `Asia/Hong_Kong`
* `Asia/Hovd`
* `Asia/Irkutsk`
* `Asia/Istanbul`
* `Asia/Jakarta`
* `Asia/Jayapura`
* `Asia/Jerusalem`
* `Asia/Kabul`
* `Asia/Kamchatka`
* `Asia/Karachi`
* `Asia/Kashgar`
* `Asia/Kathmandu`
* `Asia/Katmandu`
* `Asia/Khandyga`
* `Asia/Kolkata`
* `Asia/Krasnoyarsk`
* `Asia/Kuala_Lumpur`
* `Asia/Kuching`
* `Asia/Kuwait`
* `Asia/Macao`
* `Asia/Macau`
* `Asia/Magadan`
* `Asia/Makassar`
* `Asia/Manila`
* `Asia/Muscat`
* `Asia/Nicosia`
* `Asia/Novokuznetsk`
* `Asia/Novosibirsk`
* `Asia/Omsk`
* `Asia/Oral`
* `Asia/Phnom_Penh`
* `Asia/Pontianak`
* `Asia/Pyongyang`
* `Asia/Qatar`
* `Asia/Qostanay`
* `Asia/Qyzylorda`
* `Asia/Rangoon`
* `Asia/Riyadh`
* `Asia/Saigon`
* `Asia/Sakhalin`
* `Asia/Samarkand`
* `Asia/Seoul`
* `Asia/Shanghai`
* `Asia/Singapore`
* `Asia/Srednekolymsk`
* `Asia/Taipei`
* `Asia/Tashkent`
* `Asia/Tbilisi`
* `Asia/Tehran`
* `Asia/Tel_Aviv`
* `Asia/Thimbu`
* `Asia/Thimphu`
* `Asia/Tokyo`
* `Asia/Tomsk`
* `Asia/Ujung_Pandang`
* `Asia/Ulaanbaatar`
* `Asia/Ulan_Bator`
* `Asia/Urumqi`
* `Asia/Ust-Nera`
* `Asia/Vientiane`
* `Asia/Vladivostok`
* `Asia/Yakutsk`
* `Asia/Yangon`
* `Asia/Yekaterinburg`
* `Asia/Yerevan`
* `Atlantic/Azores`
* `Atlantic/Bermuda`
* `Atlantic/Canary`
* `Atlantic/Cape_Verde`
* `Atlantic/Faeroe`
* `Atlantic/Faroe`
* `Atlantic/Jan_Mayen`
* `Atlantic/Madeira`
* `Atlantic/Reykjavik`
* `Atlantic/South_Georgia`
* `Atlantic/St_Helena`
* `Atlantic/Stanley`
* `Australia/ACT`
* `Australia/Adelaide`
* `Australia/Brisbane`
* `Australia/Broken_Hill`
* `Australia/Canberra`
* `Australia/Currie`
* `Australia/Darwin`
* `Australia/Eucla`
* `Australia/Hobart`
* `Australia/LHI`
* `Australia/Lindeman`
* `Australia/Lord_Howe`
* `Australia/Melbourne`
* `Australia/NSW`
* `Australia/North`
* `Australia/Perth`
* `Australia/Queensland`
* `Australia/South`
* `Australia/Sydney`
* `Australia/Tasmania`
* `Australia/Victoria`
* `Australia/West`
* `Australia/Yancowinna`
* `Brazil/Acre`
* `Brazil/DeNoronha`
* `Brazil/East`
* `Brazil/West`
* `CET`
* `CST6CDT`
* `Canada/Atlantic`
* `Canada/Central`
* `Canada/Eastern`
* `Canada/Mountain`
* `Canada/Newfoundland`
* `Canada/Pacific`
* `Canada/Saskatchewan`
* `Canada/Yukon`
* `Chile/Continental`
* `Chile/EasterIsland`
* `Cuba`
* `EET`
* `EST`
* `EST5EDT`
* `Egypt`
* `Eire`
* `Etc/GMT`
* `Etc/GMT+0`
* `Etc/GMT+1`
* `Etc/GMT+10`
* `Etc/GMT+11`
* `Etc/GMT+12`
* `Etc/GMT+2`
* `Etc/GMT+3`
* `Etc/GMT+4`
* `Etc/GMT+5`
* `Etc/GMT+6`
* `Etc/GMT+7`
* `Etc/GMT+8`
* `Etc/GMT+9`
* `Etc/GMT-0`
* `Etc/GMT-1`
* `Etc/GMT-10`
* `Etc/GMT-11`
* `Etc/GMT-12`
* `Etc/GMT-13`
* `Etc/GMT-14`
* `Etc/GMT-2`
* `Etc/GMT-3`
* `Etc/GMT-4`
* `Etc/GMT-5`
* `Etc/GMT-6`
* `Etc/GMT-7`
* `Etc/GMT-8`
* `Etc/GMT-9`
* `Etc/GMT0`
* `Etc/Greenwich`
* `Etc/UCT`
* `Etc/UTC`
* `Etc/Universal`
* `Etc/Zulu`
* `Europe/Amsterdam`
* `Europe/Andorra`
* `Europe/Astrakhan`
* `Europe/Athens`
* `Europe/Belfast`
* `Europe/Belgrade`
* `Europe/Berlin`
* `Europe/Bratislava`
* `Europe/Brussels`
* `Europe/Bucharest`
* `Europe/Budapest`
* `Europe/Busingen`
* `Europe/Chisinau`
* `Europe/Copenhagen`
* `Europe/Dublin`
* `Europe/Gibraltar`
* `Europe/Guernsey`
* `Europe/Helsinki`
* `Europe/Isle_of_Man`
* `Europe/Istanbul`
* `Europe/Jersey`
* `Europe/Kaliningrad`
* `Europe/Kiev`
* `Europe/Kirov`
* `Europe/Kyiv`
* `Europe/Lisbon`
* `Europe/Ljubljana`
* `Europe/London`
* `Europe/Luxembourg`
* `Europe/Madrid`
* `Europe/Malta`
* `Europe/Mariehamn`
* `Europe/Minsk`
* `Europe/Monaco`
* `Europe/Moscow`
* `Europe/Nicosia`
* `Europe/Oslo`
* `Europe/Paris`
* `Europe/Podgorica`
* `Europe/Prague`
* `Europe/Riga`
* `Europe/Rome`
* `Europe/Samara`
* `Europe/San_Marino`
* `Europe/Sarajevo`
* `Europe/Saratov`
* `Europe/Simferopol`
* `Europe/Skopje`
* `Europe/Sofia`
* `Europe/Stockholm`
* `Europe/Tallinn`
* `Europe/Tirane`
* `Europe/Tiraspol`
* `Europe/Ulyanovsk`
* `Europe/Uzhgorod`
* `Europe/Vaduz`
* `Europe/Vatican`
* `Europe/Vienna`
* `Europe/Vilnius`
* `Europe/Volgograd`
* `Europe/Warsaw`
* `Europe/Zagreb`
* `Europe/Zaporozhye`
* `Europe/Zurich`
* `GB`
* `GB-Eire`
* `GMT`
* `GMT+0`
* `GMT-0`
* `GMT0`
* `Greenwich`
* `HST`
* `Hongkong`
* `Iceland`
* `Indian/Antananarivo`
* `Indian/Chagos`
* `Indian/Christmas`
* `Indian/Cocos`
* `Indian/Comoro`
* `Indian/Kerguelen`
* `Indian/Mahe`
* `Indian/Maldives`
* `Indian/Mauritius`
* `Indian/Mayotte`
* `Indian/Reunion`
* `Iran`
* `Israel`
* `Jamaica`
* `Japan`
* `Kwajalein`
* `Libya`
* `MET`
* `MST`
* `MST7MDT`
* `Mexico/BajaNorte`
* `Mexico/BajaSur`
* `Mexico/General`
* `NZ`
* `NZ-CHAT`
* `Navajo`
* `PRC`
* `PST8PDT`
* `Pacific/Apia`
* `Pacific/Auckland`
* `Pacific/Bougainville`
* `Pacific/Chatham`
* `Pacific/Chuuk`
* `Pacific/Easter`
* `Pacific/Efate`
* `Pacific/Enderbury`
* `Pacific/Fakaofo`
* `Pacific/Fiji`
* `Pacific/Funafuti`
* `Pacific/Galapagos`
* `Pacific/Gambier`
* `Pacific/Guadalcanal`
* `Pacific/Guam`
* `Pacific/Honolulu`
* `Pacific/Johnston`
* `Pacific/Kanton`
* `Pacific/Kiritimati`
* `Pacific/Kosrae`
* `Pacific/Kwajalein`
* `Pacific/Majuro`
* `Pacific/Marquesas`
* `Pacific/Midway`
* `Pacific/Nauru`
* `Pacific/Niue`
* `Pacific/Norfolk`
* `Pacific/Noumea`
* `Pacific/Pago_Pago`
* `Pacific/Palau`
* `Pacific/Pitcairn`
* `Pacific/Pohnpei`
* `Pacific/Ponape`
* `Pacific/Port_Moresby`
* `Pacific/Rarotonga`
* `Pacific/Saipan`
* `Pacific/Samoa`
* `Pacific/Tahiti`
* `Pacific/Tarawa`
* `Pacific/Tongatapu`
* `Pacific/Truk`
* `Pacific/Wake`
* `Pacific/Wallis`
* `Pacific/Yap`
* `Poland`
* `Portugal`
* `ROC`
* `ROK`
* `Singapore`
* `Turkey`
* `UCT`
* `US/Alaska`
* `US/Aleutian`
* `US/Arizona`
* `US/Central`
* `US/East-Indiana`
* `US/Eastern`
* `US/Hawaii`
* `US/Indiana-Starke`
* `US/Michigan`
* `US/Mountain`
* `US/Pacific`
* `US/Samoa`
* `UTC`
* `Universal`
* `W-SU`
* `WET`
* `Zulu`
* `localtime`*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timezone: ::std::option::Option<::std::string::String>,
}
/**Currently supports "crontab" or "interval"

* `crontab`
* `interval`*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum MonitorCreateRequestConfigScheduleType {
    #[serde(rename = "crontab")]
    Crontab,
    #[serde(rename = "interval")]
    Interval,
}
impl ::std::fmt::Display for MonitorCreateRequestConfigScheduleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Crontab => f.write_str("crontab"),
            Self::Interval => f.write_str("interval"),
        }
    }
}
impl ::std::str::FromStr for MonitorCreateRequestConfigScheduleType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "crontab" => Ok(Self::Crontab),
            "interval" => Ok(Self::Interval),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MonitorCreateRequestConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for MonitorCreateRequestConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Name of the monitor. Used for notifications. If not set the slug will be derived from your monitor name.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct MonitorCreateRequestName(::std::string::String);
impl ::std::ops::Deref for MonitorCreateRequestName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MonitorCreateRequestName> for ::std::string::String {
    fn from(value: MonitorCreateRequestName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MonitorCreateRequestName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MonitorCreateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MonitorCreateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MonitorCreateRequestName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Uniquely identifies your monitor within your organization. Changing this slug will require updates to any instrumented check-in calls.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct MonitorCreateRequestSlug(::std::string::String);
impl ::std::ops::Deref for MonitorCreateRequestSlug {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MonitorCreateRequestSlug> for ::std::string::String {
    fn from(value: MonitorCreateRequestSlug) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MonitorCreateRequestSlug {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 50usize {
            return Err("longer than 50 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^(?![0-9]+$)[a-z0-9_\\-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(?![0-9]+$)[a-z0-9_\\-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MonitorCreateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MonitorCreateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MonitorCreateRequestSlug {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Status of the monitor. Disabled monitors will not accept events and will not count towards the monitor quota.

* `active`
* `disabled`*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum MonitorCreateRequestStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for MonitorCreateRequestStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for MonitorCreateRequestStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MonitorCreateRequestStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MonitorCreateRequestStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for MonitorCreateRequestStatus {
    fn default() -> Self {
        MonitorCreateRequestStatus::Active
    }
}
///`MonitorCreateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateResponse {
    #[serde(
        rename = "alertRule",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub alert_rule: ::std::option::Option<MonitorCreateResponseAlertRule>,
    pub config: MonitorCreateResponseConfig,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub environments: MonitorCreateResponseEnvironments,
    pub id: ::std::string::String,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    #[serde(rename = "isUpserting")]
    pub is_upserting: bool,
    pub name: ::std::string::String,
    pub owner: MonitorCreateResponseOwner,
    pub project: MonitorCreateResponseProject,
    pub slug: ::std::string::String,
    pub status: ::std::string::String,
}
///`MonitorCreateResponseAlertRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateResponseAlertRule {
    pub environment: ::std::string::String,
    pub targets: ::std::vec::Vec<MonitorCreateResponseAlertRuleTargetsItem>,
}
///`MonitorCreateResponseAlertRuleTargetsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateResponseAlertRuleTargetsItem {
    #[serde(rename = "targetIdentifier")]
    pub target_identifier: i64,
    #[serde(rename = "targetType")]
    pub target_type: ::std::string::String,
}
///`MonitorCreateResponseConfig`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateResponseConfig {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub alert_rule_id: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub checkin_margin: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub failure_issue_threshold: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max_runtime: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub recovery_threshold: ::std::option::Option<i64>,
    pub schedule: MonitorCreateResponseConfigSchedule,
    pub schedule_type: MonitorCreateResponseConfigScheduleType,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub timezone: ::std::option::Option<::std::string::String>,
}
///`MonitorCreateResponseConfigSchedule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum MonitorCreateResponseConfigSchedule {
    String(::std::string::String),
    Array(::std::vec::Vec<i64>),
}
impl ::std::convert::From<::std::vec::Vec<i64>> for MonitorCreateResponseConfigSchedule {
    fn from(value: ::std::vec::Vec<i64>) -> Self {
        Self::Array(value)
    }
}
///`MonitorCreateResponseConfigScheduleType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum MonitorCreateResponseConfigScheduleType {
    #[serde(rename = "crontab")]
    Crontab,
    #[serde(rename = "interval")]
    Interval,
}
impl ::std::fmt::Display for MonitorCreateResponseConfigScheduleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Crontab => f.write_str("crontab"),
            Self::Interval => f.write_str("interval"),
        }
    }
}
impl ::std::str::FromStr for MonitorCreateResponseConfigScheduleType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "crontab" => Ok(Self::Crontab),
            "interval" => Ok(Self::Interval),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MonitorCreateResponseConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for MonitorCreateResponseConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MonitorCreateResponseEnvironments`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateResponseEnvironments {
    #[serde(
        rename = "activeIncident",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub active_incident: ::std::option::Option<
        MonitorCreateResponseEnvironmentsActiveIncident,
    >,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    #[serde(rename = "lastCheckIn")]
    pub last_check_in: ::chrono::DateTime<::chrono::offset::Utc>,
    pub name: ::std::string::String,
    #[serde(rename = "nextCheckIn")]
    pub next_check_in: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "nextCheckInLatest")]
    pub next_check_in_latest: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
}
///`MonitorCreateResponseEnvironmentsActiveIncident`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateResponseEnvironmentsActiveIncident {
    #[serde(
        rename = "brokenNotice",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub broken_notice: ::std::option::Option<
        MonitorCreateResponseEnvironmentsActiveIncidentBrokenNotice,
    >,
    #[serde(rename = "resolvingTimestamp")]
    pub resolving_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "startingTimestamp")]
    pub starting_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
///`MonitorCreateResponseEnvironmentsActiveIncidentBrokenNotice`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateResponseEnvironmentsActiveIncidentBrokenNotice {
    #[serde(rename = "environmentMutedTimestamp")]
    pub environment_muted_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "userNotifiedTimestamp")]
    pub user_notified_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
///`MonitorCreateResponseOwner`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateResponseOwner {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: MonitorCreateResponseOwnerType,
}
///`MonitorCreateResponseOwnerType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum MonitorCreateResponseOwnerType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "team")]
    Team,
}
impl ::std::fmt::Display for MonitorCreateResponseOwnerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Team => f.write_str("team"),
        }
    }
}
impl ::std::str::FromStr for MonitorCreateResponseOwnerType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "team" => Ok(Self::Team),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MonitorCreateResponseOwnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MonitorCreateResponseOwnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MonitorCreateResponseProject`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateResponseProject {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: MonitorCreateResponseProjectAvatar,
    pub color: ::std::string::String,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub name: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    pub status: ::std::string::String,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
}
///`MonitorCreateResponseProjectAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorCreateResponseProjectAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`MonitorObserveResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorObserveResponse {
    #[serde(
        rename = "alertRule",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub alert_rule: ::std::option::Option<MonitorObserveResponseAlertRule>,
    pub config: MonitorObserveResponseConfig,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub environments: MonitorObserveResponseEnvironments,
    pub id: ::std::string::String,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    #[serde(rename = "isUpserting")]
    pub is_upserting: bool,
    pub name: ::std::string::String,
    pub owner: MonitorObserveResponseOwner,
    pub project: MonitorObserveResponseProject,
    pub slug: ::std::string::String,
    pub status: ::std::string::String,
}
///`MonitorObserveResponseAlertRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorObserveResponseAlertRule {
    pub environment: ::std::string::String,
    pub targets: ::std::vec::Vec<MonitorObserveResponseAlertRuleTargetsItem>,
}
///`MonitorObserveResponseAlertRuleTargetsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorObserveResponseAlertRuleTargetsItem {
    #[serde(rename = "targetIdentifier")]
    pub target_identifier: i64,
    #[serde(rename = "targetType")]
    pub target_type: ::std::string::String,
}
///`MonitorObserveResponseConfig`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorObserveResponseConfig {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub alert_rule_id: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub checkin_margin: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub failure_issue_threshold: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max_runtime: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub recovery_threshold: ::std::option::Option<i64>,
    pub schedule: MonitorObserveResponseConfigSchedule,
    pub schedule_type: MonitorObserveResponseConfigScheduleType,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub timezone: ::std::option::Option<::std::string::String>,
}
///`MonitorObserveResponseConfigSchedule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum MonitorObserveResponseConfigSchedule {
    String(::std::string::String),
    Array(::std::vec::Vec<i64>),
}
impl ::std::convert::From<::std::vec::Vec<i64>>
for MonitorObserveResponseConfigSchedule {
    fn from(value: ::std::vec::Vec<i64>) -> Self {
        Self::Array(value)
    }
}
///`MonitorObserveResponseConfigScheduleType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum MonitorObserveResponseConfigScheduleType {
    #[serde(rename = "crontab")]
    Crontab,
    #[serde(rename = "interval")]
    Interval,
}
impl ::std::fmt::Display for MonitorObserveResponseConfigScheduleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Crontab => f.write_str("crontab"),
            Self::Interval => f.write_str("interval"),
        }
    }
}
impl ::std::str::FromStr for MonitorObserveResponseConfigScheduleType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "crontab" => Ok(Self::Crontab),
            "interval" => Ok(Self::Interval),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MonitorObserveResponseConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for MonitorObserveResponseConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MonitorObserveResponseEnvironments`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorObserveResponseEnvironments {
    #[serde(
        rename = "activeIncident",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub active_incident: ::std::option::Option<
        MonitorObserveResponseEnvironmentsActiveIncident,
    >,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    #[serde(rename = "lastCheckIn")]
    pub last_check_in: ::chrono::DateTime<::chrono::offset::Utc>,
    pub name: ::std::string::String,
    #[serde(rename = "nextCheckIn")]
    pub next_check_in: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "nextCheckInLatest")]
    pub next_check_in_latest: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
}
///`MonitorObserveResponseEnvironmentsActiveIncident`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorObserveResponseEnvironmentsActiveIncident {
    #[serde(
        rename = "brokenNotice",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub broken_notice: ::std::option::Option<
        MonitorObserveResponseEnvironmentsActiveIncidentBrokenNotice,
    >,
    #[serde(rename = "resolvingTimestamp")]
    pub resolving_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "startingTimestamp")]
    pub starting_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
///`MonitorObserveResponseEnvironmentsActiveIncidentBrokenNotice`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorObserveResponseEnvironmentsActiveIncidentBrokenNotice {
    #[serde(rename = "environmentMutedTimestamp")]
    pub environment_muted_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "userNotifiedTimestamp")]
    pub user_notified_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
///`MonitorObserveResponseOwner`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorObserveResponseOwner {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: MonitorObserveResponseOwnerType,
}
///`MonitorObserveResponseOwnerType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum MonitorObserveResponseOwnerType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "team")]
    Team,
}
impl ::std::fmt::Display for MonitorObserveResponseOwnerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Team => f.write_str("team"),
        }
    }
}
impl ::std::str::FromStr for MonitorObserveResponseOwnerType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "team" => Ok(Self::Team),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MonitorObserveResponseOwnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MonitorObserveResponseOwnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MonitorObserveResponseProject`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorObserveResponseProject {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: MonitorObserveResponseProjectAvatar,
    pub color: ::std::string::String,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub name: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    pub status: ::std::string::String,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
}
///`MonitorObserveResponseProjectAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorObserveResponseProjectAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateRequest {
    ///Alert rule configuration created alongside the monitor.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub alert_rule: ::std::option::Option<MonitorUpdateRequestAlertRule>,
    ///The configuration for the monitor.
    pub config: MonitorUpdateRequestConfig,
    ///Disable creation of monitor incidents
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_muted: ::std::option::Option<bool>,
    ///Name of the monitor. Used for notifications. If not set the slug will be derived from your monitor name.
    pub name: MonitorUpdateRequestName,
    ///The ID of the team or user that owns the monitor. (eg. user:51 or team:6)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<::std::string::String>,
    ///The project ID or slug to associate the monitor to.
    pub project: ::std::string::String,
    ///Uniquely identifies your monitor within your organization. Changing this slug will require updates to any instrumented check-in calls.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<MonitorUpdateRequestSlug>,
    /**Status of the monitor. Disabled monitors will not accept events and will not count towards the monitor quota.

* `active`
* `disabled`*/
    #[serde(default = "defaults::monitor_update_request_status")]
    pub status: MonitorUpdateRequestStatus,
}
///`MonitorUpdateRequestAlertRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateRequestAlertRule {
    ///Name of the environment
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub environment: ::std::option::Option<MonitorUpdateRequestAlertRuleEnvironment>,
    ///Array of dictionaries with information of the user or team to be notified
    pub targets: ::std::vec::Vec<MonitorUpdateRequestAlertRuleTargetsItem>,
}
///Name of the environment
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct MonitorUpdateRequestAlertRuleEnvironment(::std::string::String);
impl ::std::ops::Deref for MonitorUpdateRequestAlertRuleEnvironment {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MonitorUpdateRequestAlertRuleEnvironment>
for ::std::string::String {
    fn from(value: MonitorUpdateRequestAlertRuleEnvironment) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MonitorUpdateRequestAlertRuleEnvironment {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MonitorUpdateRequestAlertRuleEnvironment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for MonitorUpdateRequestAlertRuleEnvironment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MonitorUpdateRequestAlertRuleEnvironment {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`MonitorUpdateRequestAlertRuleTargetsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateRequestAlertRuleTargetsItem {
    ///ID of target object
    pub target_identifier: i64,
    ///One of [Member, Team]
    pub target_type: ::std::string::String,
}
///`MonitorUpdateRequestConfig`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateRequestConfig {
    ///How long (in minutes) after the expected checkin time will we wait until we consider the checkin to have been missed.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub checkin_margin: ::std::option::Option<::std::num::NonZeroU64>,
    ///How many consecutive missed or failed check-ins in a row before creating a new issue.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub failure_issue_threshold: ::std::option::Option<::std::num::NonZeroU64>,
    ///How long (in minutes) is the checkin allowed to run for in CheckInStatus.IN_PROGRESS before it is considered failed.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_runtime: ::std::option::Option<::std::num::NonZeroU64>,
    ///How many successful check-ins in a row before resolving an issue.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub recovery_threshold: ::std::option::Option<::std::num::NonZeroU64>,
    ///Varies depending on the schedule_type. Is either a crontab string, or a 2 element tuple for intervals (e.g. [1, 'day'])
    pub schedule: ::serde_json::Value,
    /**Currently supports "crontab" or "interval"

* `crontab`
* `interval`*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub schedule_type: ::std::option::Option<MonitorUpdateRequestConfigScheduleType>,
    /**tz database style timezone string

* `Africa/Abidjan`
* `Africa/Accra`
* `Africa/Addis_Ababa`
* `Africa/Algiers`
* `Africa/Asmara`
* `Africa/Asmera`
* `Africa/Bamako`
* `Africa/Bangui`
* `Africa/Banjul`
* `Africa/Bissau`
* `Africa/Blantyre`
* `Africa/Brazzaville`
* `Africa/Bujumbura`
* `Africa/Cairo`
* `Africa/Casablanca`
* `Africa/Ceuta`
* `Africa/Conakry`
* `Africa/Dakar`
* `Africa/Dar_es_Salaam`
* `Africa/Djibouti`
* `Africa/Douala`
* `Africa/El_Aaiun`
* `Africa/Freetown`
* `Africa/Gaborone`
* `Africa/Harare`
* `Africa/Johannesburg`
* `Africa/Juba`
* `Africa/Kampala`
* `Africa/Khartoum`
* `Africa/Kigali`
* `Africa/Kinshasa`
* `Africa/Lagos`
* `Africa/Libreville`
* `Africa/Lome`
* `Africa/Luanda`
* `Africa/Lubumbashi`
* `Africa/Lusaka`
* `Africa/Malabo`
* `Africa/Maputo`
* `Africa/Maseru`
* `Africa/Mbabane`
* `Africa/Mogadishu`
* `Africa/Monrovia`
* `Africa/Nairobi`
* `Africa/Ndjamena`
* `Africa/Niamey`
* `Africa/Nouakchott`
* `Africa/Ouagadougou`
* `Africa/Porto-Novo`
* `Africa/Sao_Tome`
* `Africa/Timbuktu`
* `Africa/Tripoli`
* `Africa/Tunis`
* `Africa/Windhoek`
* `America/Adak`
* `America/Anchorage`
* `America/Anguilla`
* `America/Antigua`
* `America/Araguaina`
* `America/Argentina/Buenos_Aires`
* `America/Argentina/Catamarca`
* `America/Argentina/ComodRivadavia`
* `America/Argentina/Cordoba`
* `America/Argentina/Jujuy`
* `America/Argentina/La_Rioja`
* `America/Argentina/Mendoza`
* `America/Argentina/Rio_Gallegos`
* `America/Argentina/Salta`
* `America/Argentina/San_Juan`
* `America/Argentina/San_Luis`
* `America/Argentina/Tucuman`
* `America/Argentina/Ushuaia`
* `America/Aruba`
* `America/Asuncion`
* `America/Atikokan`
* `America/Atka`
* `America/Bahia`
* `America/Bahia_Banderas`
* `America/Barbados`
* `America/Belem`
* `America/Belize`
* `America/Blanc-Sablon`
* `America/Boa_Vista`
* `America/Bogota`
* `America/Boise`
* `America/Buenos_Aires`
* `America/Cambridge_Bay`
* `America/Campo_Grande`
* `America/Cancun`
* `America/Caracas`
* `America/Catamarca`
* `America/Cayenne`
* `America/Cayman`
* `America/Chicago`
* `America/Chihuahua`
* `America/Ciudad_Juarez`
* `America/Coral_Harbour`
* `America/Cordoba`
* `America/Costa_Rica`
* `America/Coyhaique`
* `America/Creston`
* `America/Cuiaba`
* `America/Curacao`
* `America/Danmarkshavn`
* `America/Dawson`
* `America/Dawson_Creek`
* `America/Denver`
* `America/Detroit`
* `America/Dominica`
* `America/Edmonton`
* `America/Eirunepe`
* `America/El_Salvador`
* `America/Ensenada`
* `America/Fort_Nelson`
* `America/Fort_Wayne`
* `America/Fortaleza`
* `America/Glace_Bay`
* `America/Godthab`
* `America/Goose_Bay`
* `America/Grand_Turk`
* `America/Grenada`
* `America/Guadeloupe`
* `America/Guatemala`
* `America/Guayaquil`
* `America/Guyana`
* `America/Halifax`
* `America/Havana`
* `America/Hermosillo`
* `America/Indiana/Indianapolis`
* `America/Indiana/Knox`
* `America/Indiana/Marengo`
* `America/Indiana/Petersburg`
* `America/Indiana/Tell_City`
* `America/Indiana/Vevay`
* `America/Indiana/Vincennes`
* `America/Indiana/Winamac`
* `America/Indianapolis`
* `America/Inuvik`
* `America/Iqaluit`
* `America/Jamaica`
* `America/Jujuy`
* `America/Juneau`
* `America/Kentucky/Louisville`
* `America/Kentucky/Monticello`
* `America/Knox_IN`
* `America/Kralendijk`
* `America/La_Paz`
* `America/Lima`
* `America/Los_Angeles`
* `America/Louisville`
* `America/Lower_Princes`
* `America/Maceio`
* `America/Managua`
* `America/Manaus`
* `America/Marigot`
* `America/Martinique`
* `America/Matamoros`
* `America/Mazatlan`
* `America/Mendoza`
* `America/Menominee`
* `America/Merida`
* `America/Metlakatla`
* `America/Mexico_City`
* `America/Miquelon`
* `America/Moncton`
* `America/Monterrey`
* `America/Montevideo`
* `America/Montreal`
* `America/Montserrat`
* `America/Nassau`
* `America/New_York`
* `America/Nipigon`
* `America/Nome`
* `America/Noronha`
* `America/North_Dakota/Beulah`
* `America/North_Dakota/Center`
* `America/North_Dakota/New_Salem`
* `America/Nuuk`
* `America/Ojinaga`
* `America/Panama`
* `America/Pangnirtung`
* `America/Paramaribo`
* `America/Phoenix`
* `America/Port-au-Prince`
* `America/Port_of_Spain`
* `America/Porto_Acre`
* `America/Porto_Velho`
* `America/Puerto_Rico`
* `America/Punta_Arenas`
* `America/Rainy_River`
* `America/Rankin_Inlet`
* `America/Recife`
* `America/Regina`
* `America/Resolute`
* `America/Rio_Branco`
* `America/Rosario`
* `America/Santa_Isabel`
* `America/Santarem`
* `America/Santiago`
* `America/Santo_Domingo`
* `America/Sao_Paulo`
* `America/Scoresbysund`
* `America/Shiprock`
* `America/Sitka`
* `America/St_Barthelemy`
* `America/St_Johns`
* `America/St_Kitts`
* `America/St_Lucia`
* `America/St_Thomas`
* `America/St_Vincent`
* `America/Swift_Current`
* `America/Tegucigalpa`
* `America/Thule`
* `America/Thunder_Bay`
* `America/Tijuana`
* `America/Toronto`
* `America/Tortola`
* `America/Vancouver`
* `America/Virgin`
* `America/Whitehorse`
* `America/Winnipeg`
* `America/Yakutat`
* `America/Yellowknife`
* `Antarctica/Casey`
* `Antarctica/Davis`
* `Antarctica/DumontDUrville`
* `Antarctica/Macquarie`
* `Antarctica/Mawson`
* `Antarctica/McMurdo`
* `Antarctica/Palmer`
* `Antarctica/Rothera`
* `Antarctica/South_Pole`
* `Antarctica/Syowa`
* `Antarctica/Troll`
* `Antarctica/Vostok`
* `Arctic/Longyearbyen`
* `Asia/Aden`
* `Asia/Almaty`
* `Asia/Amman`
* `Asia/Anadyr`
* `Asia/Aqtau`
* `Asia/Aqtobe`
* `Asia/Ashgabat`
* `Asia/Ashkhabad`
* `Asia/Atyrau`
* `Asia/Baghdad`
* `Asia/Bahrain`
* `Asia/Baku`
* `Asia/Bangkok`
* `Asia/Barnaul`
* `Asia/Beirut`
* `Asia/Bishkek`
* `Asia/Brunei`
* `Asia/Calcutta`
* `Asia/Chita`
* `Asia/Choibalsan`
* `Asia/Chongqing`
* `Asia/Chungking`
* `Asia/Colombo`
* `Asia/Dacca`
* `Asia/Damascus`
* `Asia/Dhaka`
* `Asia/Dili`
* `Asia/Dubai`
* `Asia/Dushanbe`
* `Asia/Famagusta`
* `Asia/Gaza`
* `Asia/Harbin`
* `Asia/Hebron`
* `Asia/Ho_Chi_Minh`
* `Asia/Hong_Kong`
* `Asia/Hovd`
* `Asia/Irkutsk`
* `Asia/Istanbul`
* `Asia/Jakarta`
* `Asia/Jayapura`
* `Asia/Jerusalem`
* `Asia/Kabul`
* `Asia/Kamchatka`
* `Asia/Karachi`
* `Asia/Kashgar`
* `Asia/Kathmandu`
* `Asia/Katmandu`
* `Asia/Khandyga`
* `Asia/Kolkata`
* `Asia/Krasnoyarsk`
* `Asia/Kuala_Lumpur`
* `Asia/Kuching`
* `Asia/Kuwait`
* `Asia/Macao`
* `Asia/Macau`
* `Asia/Magadan`
* `Asia/Makassar`
* `Asia/Manila`
* `Asia/Muscat`
* `Asia/Nicosia`
* `Asia/Novokuznetsk`
* `Asia/Novosibirsk`
* `Asia/Omsk`
* `Asia/Oral`
* `Asia/Phnom_Penh`
* `Asia/Pontianak`
* `Asia/Pyongyang`
* `Asia/Qatar`
* `Asia/Qostanay`
* `Asia/Qyzylorda`
* `Asia/Rangoon`
* `Asia/Riyadh`
* `Asia/Saigon`
* `Asia/Sakhalin`
* `Asia/Samarkand`
* `Asia/Seoul`
* `Asia/Shanghai`
* `Asia/Singapore`
* `Asia/Srednekolymsk`
* `Asia/Taipei`
* `Asia/Tashkent`
* `Asia/Tbilisi`
* `Asia/Tehran`
* `Asia/Tel_Aviv`
* `Asia/Thimbu`
* `Asia/Thimphu`
* `Asia/Tokyo`
* `Asia/Tomsk`
* `Asia/Ujung_Pandang`
* `Asia/Ulaanbaatar`
* `Asia/Ulan_Bator`
* `Asia/Urumqi`
* `Asia/Ust-Nera`
* `Asia/Vientiane`
* `Asia/Vladivostok`
* `Asia/Yakutsk`
* `Asia/Yangon`
* `Asia/Yekaterinburg`
* `Asia/Yerevan`
* `Atlantic/Azores`
* `Atlantic/Bermuda`
* `Atlantic/Canary`
* `Atlantic/Cape_Verde`
* `Atlantic/Faeroe`
* `Atlantic/Faroe`
* `Atlantic/Jan_Mayen`
* `Atlantic/Madeira`
* `Atlantic/Reykjavik`
* `Atlantic/South_Georgia`
* `Atlantic/St_Helena`
* `Atlantic/Stanley`
* `Australia/ACT`
* `Australia/Adelaide`
* `Australia/Brisbane`
* `Australia/Broken_Hill`
* `Australia/Canberra`
* `Australia/Currie`
* `Australia/Darwin`
* `Australia/Eucla`
* `Australia/Hobart`
* `Australia/LHI`
* `Australia/Lindeman`
* `Australia/Lord_Howe`
* `Australia/Melbourne`
* `Australia/NSW`
* `Australia/North`
* `Australia/Perth`
* `Australia/Queensland`
* `Australia/South`
* `Australia/Sydney`
* `Australia/Tasmania`
* `Australia/Victoria`
* `Australia/West`
* `Australia/Yancowinna`
* `Brazil/Acre`
* `Brazil/DeNoronha`
* `Brazil/East`
* `Brazil/West`
* `CET`
* `CST6CDT`
* `Canada/Atlantic`
* `Canada/Central`
* `Canada/Eastern`
* `Canada/Mountain`
* `Canada/Newfoundland`
* `Canada/Pacific`
* `Canada/Saskatchewan`
* `Canada/Yukon`
* `Chile/Continental`
* `Chile/EasterIsland`
* `Cuba`
* `EET`
* `EST`
* `EST5EDT`
* `Egypt`
* `Eire`
* `Etc/GMT`
* `Etc/GMT+0`
* `Etc/GMT+1`
* `Etc/GMT+10`
* `Etc/GMT+11`
* `Etc/GMT+12`
* `Etc/GMT+2`
* `Etc/GMT+3`
* `Etc/GMT+4`
* `Etc/GMT+5`
* `Etc/GMT+6`
* `Etc/GMT+7`
* `Etc/GMT+8`
* `Etc/GMT+9`
* `Etc/GMT-0`
* `Etc/GMT-1`
* `Etc/GMT-10`
* `Etc/GMT-11`
* `Etc/GMT-12`
* `Etc/GMT-13`
* `Etc/GMT-14`
* `Etc/GMT-2`
* `Etc/GMT-3`
* `Etc/GMT-4`
* `Etc/GMT-5`
* `Etc/GMT-6`
* `Etc/GMT-7`
* `Etc/GMT-8`
* `Etc/GMT-9`
* `Etc/GMT0`
* `Etc/Greenwich`
* `Etc/UCT`
* `Etc/UTC`
* `Etc/Universal`
* `Etc/Zulu`
* `Europe/Amsterdam`
* `Europe/Andorra`
* `Europe/Astrakhan`
* `Europe/Athens`
* `Europe/Belfast`
* `Europe/Belgrade`
* `Europe/Berlin`
* `Europe/Bratislava`
* `Europe/Brussels`
* `Europe/Bucharest`
* `Europe/Budapest`
* `Europe/Busingen`
* `Europe/Chisinau`
* `Europe/Copenhagen`
* `Europe/Dublin`
* `Europe/Gibraltar`
* `Europe/Guernsey`
* `Europe/Helsinki`
* `Europe/Isle_of_Man`
* `Europe/Istanbul`
* `Europe/Jersey`
* `Europe/Kaliningrad`
* `Europe/Kiev`
* `Europe/Kirov`
* `Europe/Kyiv`
* `Europe/Lisbon`
* `Europe/Ljubljana`
* `Europe/London`
* `Europe/Luxembourg`
* `Europe/Madrid`
* `Europe/Malta`
* `Europe/Mariehamn`
* `Europe/Minsk`
* `Europe/Monaco`
* `Europe/Moscow`
* `Europe/Nicosia`
* `Europe/Oslo`
* `Europe/Paris`
* `Europe/Podgorica`
* `Europe/Prague`
* `Europe/Riga`
* `Europe/Rome`
* `Europe/Samara`
* `Europe/San_Marino`
* `Europe/Sarajevo`
* `Europe/Saratov`
* `Europe/Simferopol`
* `Europe/Skopje`
* `Europe/Sofia`
* `Europe/Stockholm`
* `Europe/Tallinn`
* `Europe/Tirane`
* `Europe/Tiraspol`
* `Europe/Ulyanovsk`
* `Europe/Uzhgorod`
* `Europe/Vaduz`
* `Europe/Vatican`
* `Europe/Vienna`
* `Europe/Vilnius`
* `Europe/Volgograd`
* `Europe/Warsaw`
* `Europe/Zagreb`
* `Europe/Zaporozhye`
* `Europe/Zurich`
* `GB`
* `GB-Eire`
* `GMT`
* `GMT+0`
* `GMT-0`
* `GMT0`
* `Greenwich`
* `HST`
* `Hongkong`
* `Iceland`
* `Indian/Antananarivo`
* `Indian/Chagos`
* `Indian/Christmas`
* `Indian/Cocos`
* `Indian/Comoro`
* `Indian/Kerguelen`
* `Indian/Mahe`
* `Indian/Maldives`
* `Indian/Mauritius`
* `Indian/Mayotte`
* `Indian/Reunion`
* `Iran`
* `Israel`
* `Jamaica`
* `Japan`
* `Kwajalein`
* `Libya`
* `MET`
* `MST`
* `MST7MDT`
* `Mexico/BajaNorte`
* `Mexico/BajaSur`
* `Mexico/General`
* `NZ`
* `NZ-CHAT`
* `Navajo`
* `PRC`
* `PST8PDT`
* `Pacific/Apia`
* `Pacific/Auckland`
* `Pacific/Bougainville`
* `Pacific/Chatham`
* `Pacific/Chuuk`
* `Pacific/Easter`
* `Pacific/Efate`
* `Pacific/Enderbury`
* `Pacific/Fakaofo`
* `Pacific/Fiji`
* `Pacific/Funafuti`
* `Pacific/Galapagos`
* `Pacific/Gambier`
* `Pacific/Guadalcanal`
* `Pacific/Guam`
* `Pacific/Honolulu`
* `Pacific/Johnston`
* `Pacific/Kanton`
* `Pacific/Kiritimati`
* `Pacific/Kosrae`
* `Pacific/Kwajalein`
* `Pacific/Majuro`
* `Pacific/Marquesas`
* `Pacific/Midway`
* `Pacific/Nauru`
* `Pacific/Niue`
* `Pacific/Norfolk`
* `Pacific/Noumea`
* `Pacific/Pago_Pago`
* `Pacific/Palau`
* `Pacific/Pitcairn`
* `Pacific/Pohnpei`
* `Pacific/Ponape`
* `Pacific/Port_Moresby`
* `Pacific/Rarotonga`
* `Pacific/Saipan`
* `Pacific/Samoa`
* `Pacific/Tahiti`
* `Pacific/Tarawa`
* `Pacific/Tongatapu`
* `Pacific/Truk`
* `Pacific/Wake`
* `Pacific/Wallis`
* `Pacific/Yap`
* `Poland`
* `Portugal`
* `ROC`
* `ROK`
* `Singapore`
* `Turkey`
* `UCT`
* `US/Alaska`
* `US/Aleutian`
* `US/Arizona`
* `US/Central`
* `US/East-Indiana`
* `US/Eastern`
* `US/Hawaii`
* `US/Indiana-Starke`
* `US/Michigan`
* `US/Mountain`
* `US/Pacific`
* `US/Samoa`
* `UTC`
* `Universal`
* `W-SU`
* `WET`
* `Zulu`
* `localtime`*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timezone: ::std::option::Option<::std::string::String>,
}
/**Currently supports "crontab" or "interval"

* `crontab`
* `interval`*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum MonitorUpdateRequestConfigScheduleType {
    #[serde(rename = "crontab")]
    Crontab,
    #[serde(rename = "interval")]
    Interval,
}
impl ::std::fmt::Display for MonitorUpdateRequestConfigScheduleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Crontab => f.write_str("crontab"),
            Self::Interval => f.write_str("interval"),
        }
    }
}
impl ::std::str::FromStr for MonitorUpdateRequestConfigScheduleType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "crontab" => Ok(Self::Crontab),
            "interval" => Ok(Self::Interval),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MonitorUpdateRequestConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for MonitorUpdateRequestConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Name of the monitor. Used for notifications. If not set the slug will be derived from your monitor name.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct MonitorUpdateRequestName(::std::string::String);
impl ::std::ops::Deref for MonitorUpdateRequestName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MonitorUpdateRequestName> for ::std::string::String {
    fn from(value: MonitorUpdateRequestName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MonitorUpdateRequestName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MonitorUpdateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MonitorUpdateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MonitorUpdateRequestName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Uniquely identifies your monitor within your organization. Changing this slug will require updates to any instrumented check-in calls.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct MonitorUpdateRequestSlug(::std::string::String);
impl ::std::ops::Deref for MonitorUpdateRequestSlug {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MonitorUpdateRequestSlug> for ::std::string::String {
    fn from(value: MonitorUpdateRequestSlug) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MonitorUpdateRequestSlug {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 50usize {
            return Err("longer than 50 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^(?![0-9]+$)[a-z0-9_\\-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(?![0-9]+$)[a-z0-9_\\-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MonitorUpdateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MonitorUpdateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MonitorUpdateRequestSlug {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Status of the monitor. Disabled monitors will not accept events and will not count towards the monitor quota.

* `active`
* `disabled`*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum MonitorUpdateRequestStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for MonitorUpdateRequestStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for MonitorUpdateRequestStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MonitorUpdateRequestStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MonitorUpdateRequestStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for MonitorUpdateRequestStatus {
    fn default() -> Self {
        MonitorUpdateRequestStatus::Active
    }
}
///`MonitorUpdateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateResponse {
    #[serde(
        rename = "alertRule",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub alert_rule: ::std::option::Option<MonitorUpdateResponseAlertRule>,
    pub config: MonitorUpdateResponseConfig,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub environments: MonitorUpdateResponseEnvironments,
    pub id: ::std::string::String,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    #[serde(rename = "isUpserting")]
    pub is_upserting: bool,
    pub name: ::std::string::String,
    pub owner: MonitorUpdateResponseOwner,
    pub project: MonitorUpdateResponseProject,
    pub slug: ::std::string::String,
    pub status: ::std::string::String,
}
///`MonitorUpdateResponseAlertRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateResponseAlertRule {
    pub environment: ::std::string::String,
    pub targets: ::std::vec::Vec<MonitorUpdateResponseAlertRuleTargetsItem>,
}
///`MonitorUpdateResponseAlertRuleTargetsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateResponseAlertRuleTargetsItem {
    #[serde(rename = "targetIdentifier")]
    pub target_identifier: i64,
    #[serde(rename = "targetType")]
    pub target_type: ::std::string::String,
}
///`MonitorUpdateResponseConfig`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateResponseConfig {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub alert_rule_id: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub checkin_margin: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub failure_issue_threshold: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max_runtime: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub recovery_threshold: ::std::option::Option<i64>,
    pub schedule: MonitorUpdateResponseConfigSchedule,
    pub schedule_type: MonitorUpdateResponseConfigScheduleType,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub timezone: ::std::option::Option<::std::string::String>,
}
///`MonitorUpdateResponseConfigSchedule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum MonitorUpdateResponseConfigSchedule {
    String(::std::string::String),
    Array(::std::vec::Vec<i64>),
}
impl ::std::convert::From<::std::vec::Vec<i64>> for MonitorUpdateResponseConfigSchedule {
    fn from(value: ::std::vec::Vec<i64>) -> Self {
        Self::Array(value)
    }
}
///`MonitorUpdateResponseConfigScheduleType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum MonitorUpdateResponseConfigScheduleType {
    #[serde(rename = "crontab")]
    Crontab,
    #[serde(rename = "interval")]
    Interval,
}
impl ::std::fmt::Display for MonitorUpdateResponseConfigScheduleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Crontab => f.write_str("crontab"),
            Self::Interval => f.write_str("interval"),
        }
    }
}
impl ::std::str::FromStr for MonitorUpdateResponseConfigScheduleType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "crontab" => Ok(Self::Crontab),
            "interval" => Ok(Self::Interval),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MonitorUpdateResponseConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for MonitorUpdateResponseConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MonitorUpdateResponseEnvironments`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateResponseEnvironments {
    #[serde(
        rename = "activeIncident",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub active_incident: ::std::option::Option<
        MonitorUpdateResponseEnvironmentsActiveIncident,
    >,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    #[serde(rename = "lastCheckIn")]
    pub last_check_in: ::chrono::DateTime<::chrono::offset::Utc>,
    pub name: ::std::string::String,
    #[serde(rename = "nextCheckIn")]
    pub next_check_in: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "nextCheckInLatest")]
    pub next_check_in_latest: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
}
///`MonitorUpdateResponseEnvironmentsActiveIncident`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateResponseEnvironmentsActiveIncident {
    #[serde(
        rename = "brokenNotice",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub broken_notice: ::std::option::Option<
        MonitorUpdateResponseEnvironmentsActiveIncidentBrokenNotice,
    >,
    #[serde(rename = "resolvingTimestamp")]
    pub resolving_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "startingTimestamp")]
    pub starting_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
///`MonitorUpdateResponseEnvironmentsActiveIncidentBrokenNotice`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateResponseEnvironmentsActiveIncidentBrokenNotice {
    #[serde(rename = "environmentMutedTimestamp")]
    pub environment_muted_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "userNotifiedTimestamp")]
    pub user_notified_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
///`MonitorUpdateResponseOwner`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateResponseOwner {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: MonitorUpdateResponseOwnerType,
}
///`MonitorUpdateResponseOwnerType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum MonitorUpdateResponseOwnerType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "team")]
    Team,
}
impl ::std::fmt::Display for MonitorUpdateResponseOwnerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Team => f.write_str("team"),
        }
    }
}
impl ::std::str::FromStr for MonitorUpdateResponseOwnerType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "team" => Ok(Self::Team),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MonitorUpdateResponseOwnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MonitorUpdateResponseOwnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MonitorUpdateResponseProject`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateResponseProject {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: MonitorUpdateResponseProjectAvatar,
    pub color: ::std::string::String,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub name: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    pub status: ::std::string::String,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
}
///`MonitorUpdateResponseProjectAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct MonitorUpdateResponseProjectAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`OrganizationMemberCreateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberCreateRequest {
    ///The email address to send the invitation to.
    pub email: OrganizationMemberCreateRequestEmail,
    /**The organization-level role of the new member. Roles include:

* `billing` - Can manage payment and compliance details.
* `member` - Can view and act on events, as well as view most other data within the organization.
* `manager` - Has full management access to all teams and projects. Can also manage
        the organization's membership.
* `owner` - Has unrestricted access to the organization, its data, and its
        settings. Can add, modify, and delete projects and members, as well as
        make billing and plan changes.
* `admin` - Can edit global integrations, manage projects, and add/remove teams.
        They automatically assume the Team Admin role for teams they join.
        Note: This role can no longer be assigned in Business and Enterprise plans. Use `TeamRoles` instead.
        */
    #[serde(
        rename = "orgRole",
        default = "defaults::organization_member_create_request_org_role"
    )]
    pub org_role: OrganizationMemberCreateRequestOrgRole,
    ///Reissue the invite token, invalidating any previously sent invite link.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub regenerate: ::std::option::Option<bool>,
    ///Whether or not to re-invite a user who has already been invited to the organization. Defaults to True.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reinvite: ::std::option::Option<bool>,
    /**Deprecated, use `orgRole`. The organization-level role to assign.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
    #[serde(default = "defaults::organization_member_create_request_role")]
    pub role: OrganizationMemberCreateRequestRole,
    ///Whether or not to send an invite notification through email. Defaults to True.
    #[serde(rename = "sendInvite", default = "defaults::default_bool::<true>")]
    pub send_invite: bool,
    /**The team and team-roles assigned to the member. Team roles can be either:
        - `contributor` - Can view and act on issues. Depending on organization settings, they can also add team members.
        - `admin` - Has full management access to their team's membership and projects.*/
    #[serde(
        rename = "teamRoles",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub team_roles: ::std::option::Option<
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    >,
    ///Deprecated, use `teamRoles`. Slugs of teams to add the member to.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub teams: ::std::vec::Vec<::serde_json::Value>,
}
///The email address to send the invitation to.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct OrganizationMemberCreateRequestEmail(::std::string::String);
impl ::std::ops::Deref for OrganizationMemberCreateRequestEmail {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<OrganizationMemberCreateRequestEmail>
for ::std::string::String {
    fn from(value: OrganizationMemberCreateRequestEmail) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for OrganizationMemberCreateRequestEmail {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 75usize {
            return Err("longer than 75 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationMemberCreateRequestEmail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for OrganizationMemberCreateRequestEmail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for OrganizationMemberCreateRequestEmail {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**The organization-level role of the new member. Roles include:

* `billing` - Can manage payment and compliance details.
* `member` - Can view and act on events, as well as view most other data within the organization.
* `manager` - Has full management access to all teams and projects. Can also manage
        the organization's membership.
* `owner` - Has unrestricted access to the organization, its data, and its
        settings. Can add, modify, and delete projects and members, as well as
        make billing and plan changes.
* `admin` - Can edit global integrations, manage projects, and add/remove teams.
        They automatically assume the Team Admin role for teams they join.
        Note: This role can no longer be assigned in Business and Enterprise plans. Use `TeamRoles` instead.
        */
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum OrganizationMemberCreateRequestOrgRole {
    #[serde(rename = "billing")]
    Billing,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "admin")]
    Admin,
}
impl ::std::fmt::Display for OrganizationMemberCreateRequestOrgRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Billing => f.write_str("billing"),
            Self::Member => f.write_str("member"),
            Self::Manager => f.write_str("manager"),
            Self::Owner => f.write_str("owner"),
            Self::Admin => f.write_str("admin"),
        }
    }
}
impl ::std::str::FromStr for OrganizationMemberCreateRequestOrgRole {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "billing" => Ok(Self::Billing),
            "member" => Ok(Self::Member),
            "manager" => Ok(Self::Manager),
            "owner" => Ok(Self::Owner),
            "admin" => Ok(Self::Admin),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationMemberCreateRequestOrgRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for OrganizationMemberCreateRequestOrgRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for OrganizationMemberCreateRequestOrgRole {
    fn default() -> Self {
        OrganizationMemberCreateRequestOrgRole::Member
    }
}
/**Deprecated, use `orgRole`. The organization-level role to assign.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum OrganizationMemberCreateRequestRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "owner")]
    Owner,
}
impl ::std::fmt::Display for OrganizationMemberCreateRequestRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Member => f.write_str("member"),
            Self::Admin => f.write_str("admin"),
            Self::Manager => f.write_str("manager"),
            Self::Owner => f.write_str("owner"),
        }
    }
}
impl ::std::str::FromStr for OrganizationMemberCreateRequestRole {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "member" => Ok(Self::Member),
            "admin" => Ok(Self::Admin),
            "manager" => Ok(Self::Manager),
            "owner" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationMemberCreateRequestRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for OrganizationMemberCreateRequestRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for OrganizationMemberCreateRequestRole {
    fn default() -> Self {
        OrganizationMemberCreateRequestRole::Member
    }
}
///`OrganizationMemberCreateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberCreateResponse {
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub email: ::std::string::String,
    pub expired: bool,
    #[serde(
        rename = "externalUsers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_users: ::std::vec::Vec<
        OrganizationMemberCreateResponseExternalUsersItem,
    >,
    pub flags: OrganizationMemberCreateResponseFlags,
    pub id: ::std::string::String,
    #[serde(rename = "inviteStatus")]
    pub invite_status: ::std::string::String,
    #[serde(
        rename = "inviterName",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub inviter_name: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(rename = "orgRole")]
    pub org_role: ::std::string::String,
    pub pending: bool,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub user: ::std::option::Option<OrganizationMemberCreateResponseUser>,
}
///`OrganizationMemberCreateResponseExternalUsersItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberCreateResponseExternalUsersItem {
    #[serde(
        rename = "externalId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalName")]
    pub external_name: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "integrationId")]
    pub integration_id: ::std::string::String,
    pub provider: ::std::string::String,
    #[serde(rename = "teamId", skip_serializing_if = "::std::option::Option::is_none")]
    pub team_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId", skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
///`OrganizationMemberCreateResponseFlags`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberCreateResponseFlags {
    #[serde(rename = "idp:provisioned")]
    pub idp_provisioned: bool,
    #[serde(rename = "idp:role-restricted")]
    pub idp_role_restricted: bool,
    #[serde(rename = "member-limit:restricted")]
    pub member_limit_restricted: bool,
    #[serde(rename = "partnership:restricted")]
    pub partnership_restricted: bool,
    #[serde(rename = "sso:invalid")]
    pub sso_invalid: bool,
    #[serde(rename = "sso:linked")]
    pub sso_linked: bool,
}
///`OrganizationMemberCreateResponseUser`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberCreateResponseUser {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub authenticators: ::std::vec::Vec<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<OrganizationMemberCreateResponseUserAvatar>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: ::std::string::String,
    #[serde(
        rename = "canReset2fa",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub can_reset2fa: ::std::option::Option<bool>,
    #[serde(rename = "dateJoined")]
    pub date_joined: ::chrono::DateTime<::chrono::offset::Utc>,
    pub email: ::std::string::String,
    pub emails: ::std::vec::Vec<OrganizationMemberCreateResponseUserEmailsItem>,
    pub experiments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub has2fa: bool,
    #[serde(rename = "hasPasswordAuth")]
    pub has_password_auth: bool,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub identities: ::std::vec::Vec<OrganizationMemberCreateResponseUserIdentitiesItem>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    #[serde(rename = "isStaff")]
    pub is_staff: bool,
    #[serde(rename = "isSuperuser")]
    pub is_superuser: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(
        rename = "lastActive",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_active: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "lastLogin",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_login: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub name: ::std::string::String,
    pub username: ::std::string::String,
}
///`OrganizationMemberCreateResponseUserAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberCreateResponseUserAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`OrganizationMemberCreateResponseUserEmailsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberCreateResponseUserEmailsItem {
    pub email: ::std::string::String,
    pub id: ::std::string::String,
    pub is_verified: bool,
}
///`OrganizationMemberCreateResponseUserIdentitiesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberCreateResponseUserIdentitiesItem {
    #[serde(rename = "dateSynced")]
    pub date_synced: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "dateVerified")]
    pub date_verified: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub organization: OrganizationMemberCreateResponseUserIdentitiesItemOrganization,
    pub provider: OrganizationMemberCreateResponseUserIdentitiesItemProvider,
}
///`OrganizationMemberCreateResponseUserIdentitiesItemOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberCreateResponseUserIdentitiesItemOrganization {
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`OrganizationMemberCreateResponseUserIdentitiesItemProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberCreateResponseUserIdentitiesItemProvider {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`OrganizationMemberObserveResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponse {
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub email: ::std::string::String,
    pub expired: bool,
    #[serde(
        rename = "externalUsers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_users: ::std::vec::Vec<
        OrganizationMemberObserveResponseExternalUsersItem,
    >,
    pub flags: OrganizationMemberObserveResponseFlags,
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub invite_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inviteStatus")]
    pub invite_status: ::std::string::String,
    #[serde(
        rename = "inviterName",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub inviter_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isOnlyOwner")]
    pub is_only_owner: bool,
    pub name: ::std::string::String,
    #[serde(rename = "orgRole")]
    pub org_role: ::std::string::String,
    #[serde(rename = "orgRoleList")]
    pub org_role_list: ::std::vec::Vec<OrganizationMemberObserveResponseOrgRoleListItem>,
    pub pending: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "roleName", skip_serializing_if = "::std::option::Option::is_none")]
    pub role_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamRoleList")]
    pub team_role_list: ::std::vec::Vec<
        OrganizationMemberObserveResponseTeamRoleListItem,
    >,
    #[serde(rename = "teamRoles")]
    pub team_roles: ::std::vec::Vec<OrganizationMemberObserveResponseTeamRolesItem>,
    pub teams: ::std::vec::Vec<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub user: ::std::option::Option<OrganizationMemberObserveResponseUser>,
}
///`OrganizationMemberObserveResponseExternalUsersItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponseExternalUsersItem {
    #[serde(
        rename = "externalId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalName")]
    pub external_name: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "integrationId")]
    pub integration_id: ::std::string::String,
    pub provider: ::std::string::String,
    #[serde(rename = "teamId", skip_serializing_if = "::std::option::Option::is_none")]
    pub team_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId", skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
///`OrganizationMemberObserveResponseFlags`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponseFlags {
    #[serde(rename = "idp:provisioned")]
    pub idp_provisioned: bool,
    #[serde(rename = "idp:role-restricted")]
    pub idp_role_restricted: bool,
    #[serde(rename = "member-limit:restricted")]
    pub member_limit_restricted: bool,
    #[serde(rename = "partnership:restricted")]
    pub partnership_restricted: bool,
    #[serde(rename = "sso:invalid")]
    pub sso_invalid: bool,
    #[serde(rename = "sso:linked")]
    pub sso_linked: bool,
}
///`OrganizationMemberObserveResponseOrgRoleListItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponseOrgRoleListItem {
    pub allowed: bool,
    pub desc: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "isAllowed")]
    pub is_allowed: bool,
    #[serde(rename = "isGlobal")]
    pub is_global: bool,
    #[serde(rename = "isRetired")]
    pub is_retired: bool,
    #[serde(rename = "isTeamRolesAllowed")]
    pub is_team_roles_allowed: bool,
    #[serde(rename = "minimumTeamRole")]
    pub minimum_team_role: ::std::string::String,
    pub name: ::std::string::String,
    pub scopes: ::std::vec::Vec<::std::string::String>,
}
///`OrganizationMemberObserveResponseTeamRoleListItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponseTeamRoleListItem {
    pub allowed: bool,
    pub desc: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "isAllowed")]
    pub is_allowed: bool,
    #[serde(
        rename = "isMinimumRoleFor",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub is_minimum_role_for: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isRetired")]
    pub is_retired: bool,
    #[serde(rename = "isTeamRolesAllowed")]
    pub is_team_roles_allowed: bool,
    pub name: ::std::string::String,
    pub scopes: ::std::vec::Vec<::std::string::String>,
}
///`OrganizationMemberObserveResponseTeamRolesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponseTeamRolesItem {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamSlug")]
    pub team_slug: ::std::string::String,
}
///`OrganizationMemberObserveResponseUser`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponseUser {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub authenticators: ::std::vec::Vec<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<OrganizationMemberObserveResponseUserAvatar>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: ::std::string::String,
    #[serde(
        rename = "canReset2fa",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub can_reset2fa: ::std::option::Option<bool>,
    #[serde(rename = "dateJoined")]
    pub date_joined: ::chrono::DateTime<::chrono::offset::Utc>,
    pub email: ::std::string::String,
    pub emails: ::std::vec::Vec<OrganizationMemberObserveResponseUserEmailsItem>,
    pub experiments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub has2fa: bool,
    #[serde(rename = "hasPasswordAuth")]
    pub has_password_auth: bool,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub identities: ::std::vec::Vec<OrganizationMemberObserveResponseUserIdentitiesItem>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    #[serde(rename = "isStaff")]
    pub is_staff: bool,
    #[serde(rename = "isSuperuser")]
    pub is_superuser: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(
        rename = "lastActive",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_active: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "lastLogin",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_login: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub name: ::std::string::String,
    pub username: ::std::string::String,
}
///`OrganizationMemberObserveResponseUserAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponseUserAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`OrganizationMemberObserveResponseUserEmailsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponseUserEmailsItem {
    pub email: ::std::string::String,
    pub id: ::std::string::String,
    pub is_verified: bool,
}
///`OrganizationMemberObserveResponseUserIdentitiesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponseUserIdentitiesItem {
    #[serde(rename = "dateSynced")]
    pub date_synced: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "dateVerified")]
    pub date_verified: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub organization: OrganizationMemberObserveResponseUserIdentitiesItemOrganization,
    pub provider: OrganizationMemberObserveResponseUserIdentitiesItemProvider,
}
///`OrganizationMemberObserveResponseUserIdentitiesItemOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponseUserIdentitiesItemOrganization {
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`OrganizationMemberObserveResponseUserIdentitiesItemProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberObserveResponseUserIdentitiesItemProvider {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`OrganizationMemberUpdateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateRequest {
    /**The organization role of the member. The options are:

* `billing` - Can manage payment and compliance details.
* `member` - Can view and act on events, as well as view most other data within the organization.
* `manager` - Has full management access to all teams and projects. Can also manage
        the organization's membership.
* `owner` - Has unrestricted access to the organization, its data, and its
        settings. Can add, modify, and delete projects and members, as well as
        make billing and plan changes.
* `admin` - Can edit global integrations, manage projects, and add/remove teams.
        They automatically assume the Team Admin role for teams they join.
        Note: This role can no longer be assigned in Business and Enterprise plans. Use `TeamRoles` instead.
        */
    #[serde(rename = "orgRole", skip_serializing_if = "::std::option::Option::is_none")]
    pub org_role: ::std::option::Option<OrganizationMemberUpdateRequestOrgRole>,
    /**
Configures the team role of the member. The two roles are:
- `contributor` - Can view and act on issues. Depending on organization settings, they can also add team members.
- `admin` - Has full management access to their team's membership and projects.
```json
{
    "teamRoles": [
        {
            "teamSlug": "ancient-gabelers",
            "role": "admin"
        },
        {
            "teamSlug": "powerful-abolitionist",
            "role": "contributor"
        }
    ]
}
```
*/
    #[serde(
        rename = "teamRoles",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub team_roles: ::std::option::Option<
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    >,
}
/**The organization role of the member. The options are:

* `billing` - Can manage payment and compliance details.
* `member` - Can view and act on events, as well as view most other data within the organization.
* `manager` - Has full management access to all teams and projects. Can also manage
        the organization's membership.
* `owner` - Has unrestricted access to the organization, its data, and its
        settings. Can add, modify, and delete projects and members, as well as
        make billing and plan changes.
* `admin` - Can edit global integrations, manage projects, and add/remove teams.
        They automatically assume the Team Admin role for teams they join.
        Note: This role can no longer be assigned in Business and Enterprise plans. Use `TeamRoles` instead.
        */
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum OrganizationMemberUpdateRequestOrgRole {
    #[serde(rename = "billing")]
    Billing,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "admin")]
    Admin,
}
impl ::std::fmt::Display for OrganizationMemberUpdateRequestOrgRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Billing => f.write_str("billing"),
            Self::Member => f.write_str("member"),
            Self::Manager => f.write_str("manager"),
            Self::Owner => f.write_str("owner"),
            Self::Admin => f.write_str("admin"),
        }
    }
}
impl ::std::str::FromStr for OrganizationMemberUpdateRequestOrgRole {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "billing" => Ok(Self::Billing),
            "member" => Ok(Self::Member),
            "manager" => Ok(Self::Manager),
            "owner" => Ok(Self::Owner),
            "admin" => Ok(Self::Admin),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationMemberUpdateRequestOrgRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for OrganizationMemberUpdateRequestOrgRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`OrganizationMemberUpdateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponse {
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub email: ::std::string::String,
    pub expired: bool,
    #[serde(
        rename = "externalUsers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_users: ::std::vec::Vec<
        OrganizationMemberUpdateResponseExternalUsersItem,
    >,
    pub flags: OrganizationMemberUpdateResponseFlags,
    pub id: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub invite_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inviteStatus")]
    pub invite_status: ::std::string::String,
    #[serde(
        rename = "inviterName",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub inviter_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isOnlyOwner")]
    pub is_only_owner: bool,
    pub name: ::std::string::String,
    #[serde(rename = "orgRole")]
    pub org_role: ::std::string::String,
    #[serde(rename = "orgRoleList")]
    pub org_role_list: ::std::vec::Vec<OrganizationMemberUpdateResponseOrgRoleListItem>,
    pub pending: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "roleName", skip_serializing_if = "::std::option::Option::is_none")]
    pub role_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamRoleList")]
    pub team_role_list: ::std::vec::Vec<
        OrganizationMemberUpdateResponseTeamRoleListItem,
    >,
    #[serde(rename = "teamRoles")]
    pub team_roles: ::std::vec::Vec<OrganizationMemberUpdateResponseTeamRolesItem>,
    pub teams: ::std::vec::Vec<::std::string::String>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub user: ::std::option::Option<OrganizationMemberUpdateResponseUser>,
}
///`OrganizationMemberUpdateResponseExternalUsersItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponseExternalUsersItem {
    #[serde(
        rename = "externalId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalName")]
    pub external_name: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "integrationId")]
    pub integration_id: ::std::string::String,
    pub provider: ::std::string::String,
    #[serde(rename = "teamId", skip_serializing_if = "::std::option::Option::is_none")]
    pub team_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId", skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
///`OrganizationMemberUpdateResponseFlags`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponseFlags {
    #[serde(rename = "idp:provisioned")]
    pub idp_provisioned: bool,
    #[serde(rename = "idp:role-restricted")]
    pub idp_role_restricted: bool,
    #[serde(rename = "member-limit:restricted")]
    pub member_limit_restricted: bool,
    #[serde(rename = "partnership:restricted")]
    pub partnership_restricted: bool,
    #[serde(rename = "sso:invalid")]
    pub sso_invalid: bool,
    #[serde(rename = "sso:linked")]
    pub sso_linked: bool,
}
///`OrganizationMemberUpdateResponseOrgRoleListItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponseOrgRoleListItem {
    pub allowed: bool,
    pub desc: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "isAllowed")]
    pub is_allowed: bool,
    #[serde(rename = "isGlobal")]
    pub is_global: bool,
    #[serde(rename = "isRetired")]
    pub is_retired: bool,
    #[serde(rename = "isTeamRolesAllowed")]
    pub is_team_roles_allowed: bool,
    #[serde(rename = "minimumTeamRole")]
    pub minimum_team_role: ::std::string::String,
    pub name: ::std::string::String,
    pub scopes: ::std::vec::Vec<::std::string::String>,
}
///`OrganizationMemberUpdateResponseTeamRoleListItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponseTeamRoleListItem {
    pub allowed: bool,
    pub desc: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "isAllowed")]
    pub is_allowed: bool,
    #[serde(
        rename = "isMinimumRoleFor",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub is_minimum_role_for: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isRetired")]
    pub is_retired: bool,
    #[serde(rename = "isTeamRolesAllowed")]
    pub is_team_roles_allowed: bool,
    pub name: ::std::string::String,
    pub scopes: ::std::vec::Vec<::std::string::String>,
}
///`OrganizationMemberUpdateResponseTeamRolesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponseTeamRolesItem {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamSlug")]
    pub team_slug: ::std::string::String,
}
///`OrganizationMemberUpdateResponseUser`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponseUser {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub authenticators: ::std::vec::Vec<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<OrganizationMemberUpdateResponseUserAvatar>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: ::std::string::String,
    #[serde(
        rename = "canReset2fa",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub can_reset2fa: ::std::option::Option<bool>,
    #[serde(rename = "dateJoined")]
    pub date_joined: ::chrono::DateTime<::chrono::offset::Utc>,
    pub email: ::std::string::String,
    pub emails: ::std::vec::Vec<OrganizationMemberUpdateResponseUserEmailsItem>,
    pub experiments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub has2fa: bool,
    #[serde(rename = "hasPasswordAuth")]
    pub has_password_auth: bool,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub identities: ::std::vec::Vec<OrganizationMemberUpdateResponseUserIdentitiesItem>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    #[serde(rename = "isStaff")]
    pub is_staff: bool,
    #[serde(rename = "isSuperuser")]
    pub is_superuser: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(
        rename = "lastActive",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_active: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "lastLogin",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_login: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub name: ::std::string::String,
    pub username: ::std::string::String,
}
///`OrganizationMemberUpdateResponseUserAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponseUserAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`OrganizationMemberUpdateResponseUserEmailsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponseUserEmailsItem {
    pub email: ::std::string::String,
    pub id: ::std::string::String,
    pub is_verified: bool,
}
///`OrganizationMemberUpdateResponseUserIdentitiesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponseUserIdentitiesItem {
    #[serde(rename = "dateSynced")]
    pub date_synced: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "dateVerified")]
    pub date_verified: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub organization: OrganizationMemberUpdateResponseUserIdentitiesItemOrganization,
    pub provider: OrganizationMemberUpdateResponseUserIdentitiesItemProvider,
}
///`OrganizationMemberUpdateResponseUserIdentitiesItemOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponseUserIdentitiesItemOrganization {
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`OrganizationMemberUpdateResponseUserIdentitiesItemProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationMemberUpdateResponseUserIdentitiesItemProvider {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`OrganizationObserveResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationObserveResponse {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowMemberInvite")]
    pub allow_member_invite: bool,
    #[serde(rename = "allowMemberProjectCreation")]
    pub allow_member_project_creation: bool,
    #[serde(rename = "allowSuperuserAccess")]
    pub allow_superuser_access: bool,
    pub avatar: OrganizationObserveResponseAvatar,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "extraOptions",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extra_options: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "hasAuthProvider")]
    pub has_auth_provider: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isEarlyAdopter")]
    pub is_early_adopter: bool,
    pub links: OrganizationObserveResponseLinks,
    pub name: ::std::string::String,
    #[serde(
        rename = "onboardingTasks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub onboarding_tasks: ::std::vec::Vec<
        OrganizationObserveResponseOnboardingTasksItem,
    >,
    #[serde(rename = "require2FA")]
    pub require2_fa: bool,
    pub slug: ::std::string::String,
    pub status: OrganizationObserveResponseStatus,
}
///`OrganizationObserveResponseAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationObserveResponseAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`OrganizationObserveResponseLinks`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationObserveResponseLinks {
    #[serde(rename = "organizationUrl")]
    pub organization_url: ::std::string::String,
    #[serde(rename = "regionUrl")]
    pub region_url: ::std::string::String,
}
///`OrganizationObserveResponseOnboardingTasksItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationObserveResponseOnboardingTasksItem {
    #[serde(
        rename = "completionSeen",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub completion_seen: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    pub data: ::serde_json::Value,
    #[serde(rename = "dateCompleted")]
    pub date_completed: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub task: ::std::option::Option<::std::string::String>,
}
///`OrganizationObserveResponseStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationObserveResponseStatus {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`OrganizationUpdateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateRequest {
    ///Specify `true` to allow members to create, edit, and delete alert rules by granting them the `alerts:write` scope.
    #[serde(
        rename = "alertsMemberWrite",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub alerts_member_write: ::std::option::Option<bool>,
    ///Specify `true` to allow users to request to join your organization.
    #[serde(
        rename = "allowJoinRequests",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_join_requests: ::std::option::Option<bool>,
    ///Specify `true` to allow sharing of limited details on issues to anonymous users.
    #[serde(
        rename = "allowSharedIssues",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_shared_issues: ::std::option::Option<bool>,
    ///Deprecated. Response-time threshold in milliseconds previously used to compute Apdex.
    #[serde(
        rename = "apdexThreshold",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub apdex_threshold: ::std::option::Option<i64>,
    /**The role required to download event attachments, such as native crash reports or log files.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
    #[serde(
        rename = "attachmentsRole",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub attachments_role: ::std::option::Option<
        OrganizationUpdateRequestAttachmentsRole,
    >,
    ///The image to upload as the organization avatar, in base64. Required if `avatarType` is `upload`.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<::std::string::String>,
    /**The type of display picture for the organization.

* `letter_avatar` - Use initials
* `upload` - Upload an image*/
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<OrganizationUpdateRequestAvatarType>,
    ///Specify `true` to restore an organization that is pending deletion.
    #[serde(
        rename = "cancelDeletion",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cancel_deletion: ::std::option::Option<bool>,
    ///Specify `true` to require server-side data scrubbing for all projects.
    #[serde(
        rename = "dataScrubber",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub data_scrubber: ::std::option::Option<bool>,
    ///Specify `true` to apply the default scrubbers to prevent things like passwords and credit cards from being stored for all projects.
    #[serde(
        rename = "dataScrubberDefaults",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub data_scrubber_defaults: ::std::option::Option<bool>,
    /**The role required to download debug information files, ProGuard mappings and source maps.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
    #[serde(
        rename = "debugFilesRole",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub debug_files_role: ::std::option::Option<OrganizationUpdateRequestDebugFilesRole>,
    /**The default role new members will receive.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
    #[serde(
        rename = "defaultRole",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_role: ::std::option::Option<OrganizationUpdateRequestDefaultRole>,
    ///Specify `true` to enable enhanced privacy controls to limit personally identifiable information (PII) as well as source code in things like notifications.
    #[serde(
        rename = "enhancedPrivacy",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub enhanced_privacy: ::std::option::Option<bool>,
    ///Specify `true` to allow members to delete events (including the delete & discard action) by granting them the `event:admin` scope.
    #[serde(
        rename = "eventsMemberAdmin",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub events_member_admin: ::std::option::Option<bool>,
    ///Specify `true` to enable granular replay permissions, allowing per-member access control for replay data.
    #[serde(
        rename = "hasGranularReplayPermissions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_granular_replay_permissions: ::std::option::Option<bool>,
    ///Specify `true` to hide AI features from the organization.
    #[serde(
        rename = "hideAiFeatures",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hide_ai_features: ::std::option::Option<bool>,
    ///Specify `true` to opt-in to new features before they're released to the public.
    #[serde(
        rename = "isEarlyAdopter",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_early_adopter: ::std::option::Option<bool>,
    ///Specify `true` to allow the Sentry Slack integration to post replies in threads for an Issue Alert notification. Requires a Slack integration.
    #[serde(
        rename = "issueAlertsThreadFlag",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub issue_alerts_thread_flag: ::std::option::Option<bool>,
    ///Specify `true` to allow the Sentry Slack integration to post replies in threads for a Metric Alert notification. Requires a Slack integration.
    #[serde(
        rename = "metricAlertsThreadFlag",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub metric_alerts_thread_flag: ::std::option::Option<bool>,
    ///The new name for the organization.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<OrganizationUpdateRequestName>,
    ///Specify `true` to allow organization members to freely join any team.
    #[serde(
        rename = "openMembership",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_membership: ::std::option::Option<bool>,
    ///A Relay base URL to use when displaying Client Key DSNs for this organization.
    #[serde(
        rename = "relayDsnEndpoint",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub relay_dsn_endpoint: ::std::option::Option<::std::string::String>,
    /**Advanced data scrubbing rules that can be configured for each project as a JSON string. The new rules will only apply to new incoming events. For more details on advanced data scrubbing, see our [full documentation](/security-legal-pii/scrubbing/advanced-datascrubbing/).

> Warning: Calling this endpoint with this field fully overwrites the advanced data scrubbing rules.

Below is an example of a payload for a set of advanced data scrubbing rules for masking credit card numbers from the log message (equivalent to `[Mask] [Credit card numbers] from [$message]` in the Sentry app) and removing a specific key called `foo` (equivalent to `[Remove] [Anything] from [extra.foo]` in the Sentry app):
```json
{
    relayPiiConfig: "{\"rules":{\"0\":{\"type\":\"creditcard\",\"redaction\":{\"method\":\"mask\"}},\"1\":{\"type\":\"anything\",\"redaction\":{\"method\":\"remove\"}}},\"applications\":{\"$message\":[\"0\"],\"extra.foo\":[\"1\"]}}"
}
```
        */
    #[serde(
        rename = "relayPiiConfig",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub relay_pii_config: ::std::option::Option<::std::string::String>,
    ///A list of user IDs who have permission to access replay data. Requires the hasGranularReplayPermissions flag to be true to be enforced.
    #[serde(
        rename = "replayAccessMembers",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub replay_access_members: ::std::option::Option<::std::vec::Vec<i64>>,
    ///Specify `true` to require and enforce two-factor authentication for all members.
    #[serde(
        rename = "require2FA",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub require2_fa: ::std::option::Option<bool>,
    ///A list of global field names which data scrubbers should ignore.
    #[serde(
        rename = "safeFields",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub safe_fields: ::std::vec::Vec<::std::string::String>,
    ///Specify `true` to allow Sentry to scrape missing JavaScript source context when possible.
    #[serde(
        rename = "scrapeJavaScript",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrape_java_script: ::std::option::Option<bool>,
    ///Specify `true` to prevent IP addresses from being stored for new events on all projects.
    #[serde(
        rename = "scrubIPAddresses",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrub_ip_addresses: ::std::option::Option<bool>,
    ///A list of additional global field names to match against when scrubbing data for all projects.
    #[serde(
        rename = "sensitiveFields",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub sensitive_fields: ::std::vec::Vec<::std::string::String>,
    ///The new slug for the organization, which needs to be unique.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<OrganizationUpdateRequestSlug>,
    /**How many native crash reports (such as Minidumps for improved processing and download in issue details) to store per issue.

* `0` - Disabled
* `1` - 1 per issue
* `5` - 5 per issue
* `10` - 10 per issue
* `20` - 20 per issue
* `50` - 50 per issue
* `100` - 100 per issue
* `-1` - Unlimited Allowed values: 0, 1, 5, 10, 20, 50, 100, -1.*/
    #[serde(
        rename = "storeCrashReports",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub store_crash_reports: ::std::option::Option<i64>,
    /**A list of local Relays (the name, public key, and description as a JSON) registered for the organization. This feature is only available for organizations on the Business and Enterprise plans. Read more about Relay [here](/product/relay/).

                                          Below is an example of a list containing a single local Relay registered for the organization:
                                          ```json
                                          {
                                            trustedRelays: [
                                                {
                                                    name: "my-relay",
                                                    publicKey: "eiwr9fdruw4erfh892qy4493reyf89ur34wefd90h",
                                                    description: "Configuration for my-relay."
                                                }
                                            ]
                                          }
                                          ```
                                          */
    #[serde(
        rename = "trustedRelays",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub trusted_relays: ::std::vec::Vec<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
}
/**The role required to download event attachments, such as native crash reports or log files.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum OrganizationUpdateRequestAttachmentsRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "owner")]
    Owner,
}
impl ::std::fmt::Display for OrganizationUpdateRequestAttachmentsRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Member => f.write_str("member"),
            Self::Admin => f.write_str("admin"),
            Self::Manager => f.write_str("manager"),
            Self::Owner => f.write_str("owner"),
        }
    }
}
impl ::std::str::FromStr for OrganizationUpdateRequestAttachmentsRole {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "member" => Ok(Self::Member),
            "admin" => Ok(Self::Admin),
            "manager" => Ok(Self::Manager),
            "owner" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationUpdateRequestAttachmentsRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for OrganizationUpdateRequestAttachmentsRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**The type of display picture for the organization.

* `letter_avatar` - Use initials
* `upload` - Upload an image*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum OrganizationUpdateRequestAvatarType {
    #[serde(rename = "letter_avatar")]
    LetterAvatar,
    #[serde(rename = "upload")]
    Upload,
}
impl ::std::fmt::Display for OrganizationUpdateRequestAvatarType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LetterAvatar => f.write_str("letter_avatar"),
            Self::Upload => f.write_str("upload"),
        }
    }
}
impl ::std::str::FromStr for OrganizationUpdateRequestAvatarType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "letter_avatar" => Ok(Self::LetterAvatar),
            "upload" => Ok(Self::Upload),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationUpdateRequestAvatarType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for OrganizationUpdateRequestAvatarType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**The role required to download debug information files, ProGuard mappings and source maps.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum OrganizationUpdateRequestDebugFilesRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "owner")]
    Owner,
}
impl ::std::fmt::Display for OrganizationUpdateRequestDebugFilesRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Member => f.write_str("member"),
            Self::Admin => f.write_str("admin"),
            Self::Manager => f.write_str("manager"),
            Self::Owner => f.write_str("owner"),
        }
    }
}
impl ::std::str::FromStr for OrganizationUpdateRequestDebugFilesRole {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "member" => Ok(Self::Member),
            "admin" => Ok(Self::Admin),
            "manager" => Ok(Self::Manager),
            "owner" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationUpdateRequestDebugFilesRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for OrganizationUpdateRequestDebugFilesRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**The default role new members will receive.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum OrganizationUpdateRequestDefaultRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "owner")]
    Owner,
}
impl ::std::fmt::Display for OrganizationUpdateRequestDefaultRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Member => f.write_str("member"),
            Self::Admin => f.write_str("admin"),
            Self::Manager => f.write_str("manager"),
            Self::Owner => f.write_str("owner"),
        }
    }
}
impl ::std::str::FromStr for OrganizationUpdateRequestDefaultRole {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "member" => Ok(Self::Member),
            "admin" => Ok(Self::Admin),
            "manager" => Ok(Self::Manager),
            "owner" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationUpdateRequestDefaultRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for OrganizationUpdateRequestDefaultRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The new name for the organization.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct OrganizationUpdateRequestName(::std::string::String);
impl ::std::ops::Deref for OrganizationUpdateRequestName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<OrganizationUpdateRequestName> for ::std::string::String {
    fn from(value: OrganizationUpdateRequestName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for OrganizationUpdateRequestName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationUpdateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganizationUpdateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for OrganizationUpdateRequestName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///The new slug for the organization, which needs to be unique.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct OrganizationUpdateRequestSlug(::std::string::String);
impl ::std::ops::Deref for OrganizationUpdateRequestSlug {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<OrganizationUpdateRequestSlug> for ::std::string::String {
    fn from(value: OrganizationUpdateRequestSlug) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for OrganizationUpdateRequestSlug {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 50usize {
            return Err("longer than 50 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for OrganizationUpdateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganizationUpdateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for OrganizationUpdateRequestSlug {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`OrganizationUpdateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponse {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "aggregatedDataConsent")]
    pub aggregated_data_consent: bool,
    #[serde(rename = "alertsMemberWrite")]
    pub alerts_member_write: bool,
    #[serde(rename = "allowJoinRequests")]
    pub allow_join_requests: bool,
    #[serde(rename = "allowMemberInvite")]
    pub allow_member_invite: bool,
    #[serde(rename = "allowMemberProjectCreation")]
    pub allow_member_project_creation: bool,
    #[serde(rename = "allowSharedIssues")]
    pub allow_shared_issues: bool,
    #[serde(rename = "allowSuperuserAccess")]
    pub allow_superuser_access: bool,
    #[serde(rename = "attachmentsRole")]
    pub attachments_role: ::std::string::String,
    #[serde(rename = "autoEnableCodeReview")]
    pub auto_enable_code_review: bool,
    #[serde(rename = "autoOpenPrs")]
    pub auto_open_prs: bool,
    pub avatar: OrganizationUpdateResponseAvatar,
    #[serde(rename = "dataScrubber")]
    pub data_scrubber: bool,
    #[serde(rename = "dataScrubberDefaults")]
    pub data_scrubber_defaults: bool,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "debugFilesRole")]
    pub debug_files_role: ::std::string::String,
    #[serde(rename = "defaultAutofixAutomationTuning")]
    pub default_autofix_automation_tuning: ::std::string::String,
    #[serde(rename = "defaultAutomatedRunStoppingPoint")]
    pub default_automated_run_stopping_point: ::std::string::String,
    #[serde(rename = "defaultCodeReviewTriggers")]
    pub default_code_review_triggers: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "defaultCodingAgent")]
    pub default_coding_agent: ::std::string::String,
    #[serde(
        rename = "defaultCodingAgentIntegrationId",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub default_coding_agent_integration_id: ::std::option::Option<
        ::std::string::String,
    >,
    #[serde(rename = "defaultRole")]
    pub default_role: ::std::string::String,
    #[serde(rename = "defaultSeerScannerAutomation")]
    pub default_seer_scanner_automation: bool,
    #[serde(
        rename = "desiredSampleRate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub desired_sample_rate: ::std::option::Option<f64>,
    #[serde(rename = "enableSeerCoding")]
    pub enable_seer_coding: bool,
    #[serde(rename = "enhancedPrivacy")]
    pub enhanced_privacy: bool,
    #[serde(rename = "eventsMemberAdmin")]
    pub events_member_admin: bool,
    pub experiments: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(
        rename = "extraOptions",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extra_options: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "hasAuthProvider")]
    pub has_auth_provider: bool,
    #[serde(rename = "hideAiFeatures")]
    pub hide_ai_features: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "isDynamicallySampled")]
    pub is_dynamically_sampled: bool,
    #[serde(rename = "isEarlyAdopter")]
    pub is_early_adopter: bool,
    #[serde(rename = "issueAlertsThreadFlag")]
    pub issue_alerts_thread_flag: bool,
    pub links: OrganizationUpdateResponseLinks,
    #[serde(rename = "metricAlertsThreadFlag")]
    pub metric_alerts_thread_flag: bool,
    pub name: ::std::string::String,
    #[serde(
        rename = "onboardingTasks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub onboarding_tasks: ::std::vec::Vec<OrganizationUpdateResponseOnboardingTasksItem>,
    #[serde(rename = "openMembership")]
    pub open_membership: bool,
    #[serde(rename = "orgRole", skip_serializing_if = "::std::option::Option::is_none")]
    pub org_role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orgRoleList")]
    pub org_role_list: ::std::vec::Vec<OrganizationUpdateResponseOrgRoleListItem>,
    #[serde(rename = "pendingAccessRequests")]
    pub pending_access_requests: i64,
    #[serde(
        rename = "planSampleRate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub plan_sample_rate: ::std::option::Option<f64>,
    pub projects: ::std::vec::Vec<OrganizationUpdateResponseProjectsItem>,
    #[serde(
        rename = "relayDsnEndpoint",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub relay_dsn_endpoint: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "relayPiiConfig",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub relay_pii_config: ::std::option::Option<::std::string::String>,
    #[serde(rename = "require2FA")]
    pub require2_fa: bool,
    #[serde(rename = "requiresSso")]
    pub requires_sso: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "safeFields")]
    pub safe_fields: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "samplingMode",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sampling_mode: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scrapeJavaScript")]
    pub scrape_java_script: bool,
    #[serde(rename = "scrubIPAddresses")]
    pub scrub_ip_addresses: bool,
    #[serde(rename = "sensitiveFields")]
    pub sensitive_fields: ::std::vec::Vec<::std::string::String>,
    pub slug: ::std::string::String,
    pub status: OrganizationUpdateResponseStatus,
    #[serde(rename = "storeCrashReports")]
    pub store_crash_reports: i64,
    #[serde(
        rename = "targetSampleRate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_sample_rate: ::std::option::Option<f64>,
    #[serde(rename = "teamRoleList")]
    pub team_role_list: ::std::vec::Vec<OrganizationUpdateResponseTeamRoleListItem>,
    pub teams: ::std::vec::Vec<OrganizationUpdateResponseTeamsItem>,
    #[serde(rename = "trustedRelays")]
    pub trusted_relays: ::std::vec::Vec<OrganizationUpdateResponseTrustedRelaysItem>,
}
///`OrganizationUpdateResponseAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`OrganizationUpdateResponseLinks`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseLinks {
    #[serde(rename = "organizationUrl")]
    pub organization_url: ::std::string::String,
    #[serde(rename = "regionUrl")]
    pub region_url: ::std::string::String,
}
///`OrganizationUpdateResponseOnboardingTasksItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseOnboardingTasksItem {
    #[serde(
        rename = "completionSeen",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub completion_seen: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    pub data: ::serde_json::Value,
    #[serde(rename = "dateCompleted")]
    pub date_completed: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub task: ::std::option::Option<::std::string::String>,
}
///`OrganizationUpdateResponseOrgRoleListItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseOrgRoleListItem {
    pub allowed: bool,
    pub desc: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "isAllowed")]
    pub is_allowed: bool,
    #[serde(rename = "isGlobal")]
    pub is_global: bool,
    #[serde(rename = "isRetired")]
    pub is_retired: bool,
    #[serde(rename = "isTeamRolesAllowed")]
    pub is_team_roles_allowed: bool,
    #[serde(rename = "minimumTeamRole")]
    pub minimum_team_role: ::std::string::String,
    pub name: ::std::string::String,
    pub scopes: ::std::vec::Vec<::std::string::String>,
}
///`OrganizationUpdateResponseProjectsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseProjectsItem {
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub environments: ::std::vec::Vec<::std::string::String>,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    #[serde(rename = "hasUserReports")]
    pub has_user_reports: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(
        rename = "latestDeploys",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub latest_deploys: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
    #[serde(
        rename = "latestRelease",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub latest_release: ::std::option::Option<
        OrganizationUpdateResponseProjectsItemLatestRelease,
    >,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub options: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    pub platforms: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub team: ::std::option::Option<OrganizationUpdateResponseProjectsItemTeam>,
    pub teams: ::std::vec::Vec<OrganizationUpdateResponseProjectsItemTeamsItem>,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
}
///`OrganizationUpdateResponseProjectsItemLatestRelease`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseProjectsItemLatestRelease {
    pub version: ::std::string::String,
}
///`OrganizationUpdateResponseProjectsItemTeam`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseProjectsItemTeam {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`OrganizationUpdateResponseProjectsItemTeamsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseProjectsItemTeamsItem {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`OrganizationUpdateResponseStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseStatus {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`OrganizationUpdateResponseTeamRoleListItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTeamRoleListItem {
    pub allowed: bool,
    pub desc: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "isAllowed")]
    pub is_allowed: bool,
    #[serde(
        rename = "isMinimumRoleFor",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub is_minimum_role_for: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isRetired")]
    pub is_retired: bool,
    #[serde(rename = "isTeamRolesAllowed")]
    pub is_team_roles_allowed: bool,
    pub name: ::std::string::String,
    pub scopes: ::std::vec::Vec<::std::string::String>,
}
///`OrganizationUpdateResponseTeamsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTeamsItem {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: OrganizationUpdateResponseTeamsItemAvatar,
    #[serde(
        rename = "dateCreated",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "externalTeams",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_teams: ::std::vec::Vec<
        OrganizationUpdateResponseTeamsItemExternalTeamsItem,
    >,
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "memberCount")]
    pub member_count: i64,
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub organization: ::std::option::Option<
        OrganizationUpdateResponseTeamsItemOrganization,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub projects: ::std::vec::Vec<OrganizationUpdateResponseTeamsItemProjectsItem>,
    pub slug: ::std::string::String,
    #[serde(
        rename = "teamRole",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub team_role: ::std::option::Option<::std::string::String>,
}
///`OrganizationUpdateResponseTeamsItemAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTeamsItemAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`OrganizationUpdateResponseTeamsItemExternalTeamsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTeamsItemExternalTeamsItem {
    #[serde(
        rename = "externalId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalName")]
    pub external_name: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "integrationId")]
    pub integration_id: ::std::string::String,
    pub provider: ::std::string::String,
    #[serde(rename = "teamId", skip_serializing_if = "::std::option::Option::is_none")]
    pub team_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId", skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
///`OrganizationUpdateResponseTeamsItemOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTeamsItemOrganization {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowMemberInvite")]
    pub allow_member_invite: bool,
    #[serde(rename = "allowMemberProjectCreation")]
    pub allow_member_project_creation: bool,
    #[serde(rename = "allowSuperuserAccess")]
    pub allow_superuser_access: bool,
    pub avatar: OrganizationUpdateResponseTeamsItemOrganizationAvatar,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "extraOptions",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extra_options: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "hasAuthProvider")]
    pub has_auth_provider: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isEarlyAdopter")]
    pub is_early_adopter: bool,
    pub links: OrganizationUpdateResponseTeamsItemOrganizationLinks,
    pub name: ::std::string::String,
    #[serde(
        rename = "onboardingTasks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub onboarding_tasks: ::std::vec::Vec<
        OrganizationUpdateResponseTeamsItemOrganizationOnboardingTasksItem,
    >,
    #[serde(rename = "require2FA")]
    pub require2_fa: bool,
    pub slug: ::std::string::String,
    pub status: OrganizationUpdateResponseTeamsItemOrganizationStatus,
}
///`OrganizationUpdateResponseTeamsItemOrganizationAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTeamsItemOrganizationAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`OrganizationUpdateResponseTeamsItemOrganizationLinks`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTeamsItemOrganizationLinks {
    #[serde(rename = "organizationUrl")]
    pub organization_url: ::std::string::String,
    #[serde(rename = "regionUrl")]
    pub region_url: ::std::string::String,
}
///`OrganizationUpdateResponseTeamsItemOrganizationOnboardingTasksItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTeamsItemOrganizationOnboardingTasksItem {
    #[serde(
        rename = "completionSeen",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub completion_seen: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    pub data: ::serde_json::Value,
    #[serde(rename = "dateCompleted")]
    pub date_completed: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub task: ::std::option::Option<::std::string::String>,
}
///`OrganizationUpdateResponseTeamsItemOrganizationStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTeamsItemOrganizationStatus {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`OrganizationUpdateResponseTeamsItemProjectsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTeamsItemProjectsItem {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: OrganizationUpdateResponseTeamsItemProjectsItemAvatar,
    pub color: ::std::string::String,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub name: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    pub status: ::std::string::String,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
}
///`OrganizationUpdateResponseTeamsItemProjectsItemAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTeamsItemProjectsItemAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`OrganizationUpdateResponseTrustedRelaysItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct OrganizationUpdateResponseTrustedRelaysItem {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "lastModified",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_modified: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "publicKey",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_key: ::std::option::Option<::std::string::String>,
}
///`ProjectCreateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectCreateRequest {
    /**
Defaults to true where the behavior is to alert the user on every new
issue. Setting this to false will turn this off and the user must create
their own alerts to be notified of new issues.
        */
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default_rules: ::std::option::Option<bool>,
    ///The name for the project.
    pub name: ProjectCreateRequestName,
    ///The platform for the project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub platform: ::std::option::Option<::std::string::String>,
    /**Uniquely identifies a project and is used for the interface.
        If not provided, it is automatically generated from the name.*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<ProjectCreateRequestSlug>,
}
///The name for the project.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ProjectCreateRequestName(::std::string::String);
impl ::std::ops::Deref for ProjectCreateRequestName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProjectCreateRequestName> for ::std::string::String {
    fn from(value: ProjectCreateRequestName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProjectCreateRequestName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 50usize {
            return Err("longer than 50 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProjectCreateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectCreateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProjectCreateRequestName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Uniquely identifies a project and is used for the interface.
        If not provided, it is automatically generated from the name.*/
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ProjectCreateRequestSlug(::std::string::String);
impl ::std::ops::Deref for ProjectCreateRequestSlug {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProjectCreateRequestSlug> for ::std::string::String {
    fn from(value: ProjectCreateRequestSlug) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProjectCreateRequestSlug {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^(?![0-9]+$)[a-z0-9_\\-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(?![0-9]+$)[a-z0-9_\\-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProjectCreateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectCreateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProjectCreateRequestSlug {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`ProjectCreateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectCreateResponse {
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub environments: ::std::vec::Vec<::std::string::String>,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    #[serde(rename = "hasUserReports")]
    pub has_user_reports: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(
        rename = "latestDeploys",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub latest_deploys: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
    #[serde(
        rename = "latestRelease",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub latest_release: ::std::option::Option<ProjectCreateResponseLatestRelease>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub options: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    pub platforms: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub team: ::std::option::Option<ProjectCreateResponseTeam>,
    pub teams: ::std::vec::Vec<ProjectCreateResponseTeamsItem>,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
}
///`ProjectCreateResponseLatestRelease`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectCreateResponseLatestRelease {
    pub version: ::std::string::String,
}
///`ProjectCreateResponseTeam`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectCreateResponseTeam {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`ProjectCreateResponseTeamsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectCreateResponseTeamsItem {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`ProjectKeyCreateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyCreateRequest {
    ///The optional name of the key. If not provided it will be automatically generated.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<ProjectKeyCreateRequestName>,
    #[serde(
        rename = "rateLimit",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rate_limit: ::std::option::Option<ProjectKeyCreateRequestRateLimit>,
    #[doc = "* `user`\n* `profiling`\n* `tempest`\n* `demo`"]
    #[serde(
        rename = "useCase",
        default = "defaults::project_key_create_request_use_case"
    )]
    pub use_case: ProjectKeyCreateRequestUseCase,
}
impl ::std::default::Default for ProjectKeyCreateRequest {
    fn default() -> Self {
        Self {
            name: Default::default(),
            rate_limit: Default::default(),
            use_case: defaults::project_key_create_request_use_case(),
        }
    }
}
///The optional name of the key. If not provided it will be automatically generated.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ProjectKeyCreateRequestName(::std::string::String);
impl ::std::ops::Deref for ProjectKeyCreateRequestName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProjectKeyCreateRequestName> for ::std::string::String {
    fn from(value: ProjectKeyCreateRequestName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProjectKeyCreateRequestName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProjectKeyCreateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectKeyCreateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProjectKeyCreateRequestName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Applies a rate limit to cap the number of errors accepted during a given time window. To
disable entirely set `rateLimit` to null.
```json
{
    "rateLimit": {
        "window": 7200, // time in seconds
        "count": 1000 // error cap
    }
}
```*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyCreateRequestRateLimit {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub window: ::std::option::Option<i64>,
}
#[doc = "* `user`\n* `profiling`\n* `tempest`\n* `demo`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum ProjectKeyCreateRequestUseCase {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "profiling")]
    Profiling,
    #[serde(rename = "tempest")]
    Tempest,
    #[serde(rename = "demo")]
    Demo,
}
impl ::std::fmt::Display for ProjectKeyCreateRequestUseCase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Profiling => f.write_str("profiling"),
            Self::Tempest => f.write_str("tempest"),
            Self::Demo => f.write_str("demo"),
        }
    }
}
impl ::std::str::FromStr for ProjectKeyCreateRequestUseCase {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "profiling" => Ok(Self::Profiling),
            "tempest" => Ok(Self::Tempest),
            "demo" => Ok(Self::Demo),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectKeyCreateRequestUseCase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectKeyCreateRequestUseCase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for ProjectKeyCreateRequestUseCase {
    fn default() -> Self {
        ProjectKeyCreateRequestUseCase::User
    }
}
///This represents a Sentry Project Client Key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyCreateResponse {
    #[serde(rename = "browserSdk")]
    pub browser_sdk: ProjectKeyCreateResponseBrowserSdk,
    #[serde(rename = "browserSdkVersion")]
    pub browser_sdk_version: ::std::string::String,
    #[serde(
        rename = "dateCreated",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub dsn: ProjectKeyCreateResponseDsn,
    #[serde(rename = "dynamicSdkLoaderOptions")]
    pub dynamic_sdk_loader_options: ProjectKeyCreateResponseDynamicSdkLoaderOptions,
    pub id: ::std::string::String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub label: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(rename = "projectId")]
    pub project_id: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub public: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "rateLimit",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub rate_limit: ::std::option::Option<ProjectKeyCreateResponseRateLimit>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub secret: ::std::option::Option<::std::string::String>,
    #[serde(rename = "useCase", skip_serializing_if = "::std::option::Option::is_none")]
    pub use_case: ::std::option::Option<::std::string::String>,
}
///`ProjectKeyCreateResponseBrowserSdk`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyCreateResponseBrowserSdk {
    pub choices: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
}
///`ProjectKeyCreateResponseDsn`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyCreateResponseDsn {
    pub cdn: ::std::string::String,
    pub crons: ::std::string::String,
    pub csp: ::std::string::String,
    pub integration: ::std::string::String,
    pub minidump: ::std::string::String,
    pub nel: ::std::string::String,
    pub otlp_logs: ::std::string::String,
    pub otlp_traces: ::std::string::String,
    pub playstation: ::std::string::String,
    pub public: ::std::string::String,
    pub secret: ::std::string::String,
    pub security: ::std::string::String,
    pub unreal: ::std::string::String,
}
///`ProjectKeyCreateResponseDynamicSdkLoaderOptions`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyCreateResponseDynamicSdkLoaderOptions {
    #[serde(rename = "hasDebug")]
    pub has_debug: bool,
    #[serde(rename = "hasFeedback")]
    pub has_feedback: bool,
    #[serde(rename = "hasLogsAndMetrics")]
    pub has_logs_and_metrics: bool,
    #[serde(rename = "hasPerformance")]
    pub has_performance: bool,
    #[serde(rename = "hasReplay")]
    pub has_replay: bool,
}
///`ProjectKeyCreateResponseRateLimit`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyCreateResponseRateLimit {
    pub count: i64,
    pub window: i64,
}
///This represents a Sentry Project Client Key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyObserveResponse {
    #[serde(rename = "browserSdk")]
    pub browser_sdk: ProjectKeyObserveResponseBrowserSdk,
    #[serde(rename = "browserSdkVersion")]
    pub browser_sdk_version: ::std::string::String,
    #[serde(
        rename = "dateCreated",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub dsn: ProjectKeyObserveResponseDsn,
    #[serde(rename = "dynamicSdkLoaderOptions")]
    pub dynamic_sdk_loader_options: ProjectKeyObserveResponseDynamicSdkLoaderOptions,
    pub id: ::std::string::String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub label: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(rename = "projectId")]
    pub project_id: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub public: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "rateLimit",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub rate_limit: ::std::option::Option<ProjectKeyObserveResponseRateLimit>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub secret: ::std::option::Option<::std::string::String>,
    #[serde(rename = "useCase", skip_serializing_if = "::std::option::Option::is_none")]
    pub use_case: ::std::option::Option<::std::string::String>,
}
///`ProjectKeyObserveResponseBrowserSdk`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyObserveResponseBrowserSdk {
    pub choices: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
}
///`ProjectKeyObserveResponseDsn`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyObserveResponseDsn {
    pub cdn: ::std::string::String,
    pub crons: ::std::string::String,
    pub csp: ::std::string::String,
    pub integration: ::std::string::String,
    pub minidump: ::std::string::String,
    pub nel: ::std::string::String,
    pub otlp_logs: ::std::string::String,
    pub otlp_traces: ::std::string::String,
    pub playstation: ::std::string::String,
    pub public: ::std::string::String,
    pub secret: ::std::string::String,
    pub security: ::std::string::String,
    pub unreal: ::std::string::String,
}
///`ProjectKeyObserveResponseDynamicSdkLoaderOptions`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyObserveResponseDynamicSdkLoaderOptions {
    #[serde(rename = "hasDebug")]
    pub has_debug: bool,
    #[serde(rename = "hasFeedback")]
    pub has_feedback: bool,
    #[serde(rename = "hasLogsAndMetrics")]
    pub has_logs_and_metrics: bool,
    #[serde(rename = "hasPerformance")]
    pub has_performance: bool,
    #[serde(rename = "hasReplay")]
    pub has_replay: bool,
}
///`ProjectKeyObserveResponseRateLimit`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyObserveResponseRateLimit {
    pub count: i64,
    pub window: i64,
}
///`ProjectKeyUpdateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyUpdateRequest {
    /**The Sentry Javascript SDK version to use. The currently supported options are:

* `latest` - Most recent version
* `7.x` - Version 7 releases*/
    #[serde(
        rename = "browserSdkVersion",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub browser_sdk_version: ::std::option::Option<
        ProjectKeyUpdateRequestBrowserSdkVersion,
    >,
    #[serde(
        rename = "dynamicSdkLoaderOptions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub dynamic_sdk_loader_options: ::std::option::Option<
        ProjectKeyUpdateRequestDynamicSdkLoaderOptions,
    >,
    ///Activate or deactivate the client key.
    #[serde(rename = "isActive", skip_serializing_if = "::std::option::Option::is_none")]
    pub is_active: ::std::option::Option<bool>,
    ///The name for the client key
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "rateLimit",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rate_limit: ::std::option::Option<ProjectKeyUpdateRequestRateLimit>,
}
/**The Sentry Javascript SDK version to use. The currently supported options are:

* `latest` - Most recent version
* `7.x` - Version 7 releases*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum ProjectKeyUpdateRequestBrowserSdkVersion {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "7.x")]
    X7X,
}
impl ::std::fmt::Display for ProjectKeyUpdateRequestBrowserSdkVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Latest => f.write_str("latest"),
            Self::X7X => f.write_str("7.x"),
        }
    }
}
impl ::std::str::FromStr for ProjectKeyUpdateRequestBrowserSdkVersion {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "latest" => Ok(Self::Latest),
            "7.x" => Ok(Self::X7X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectKeyUpdateRequestBrowserSdkVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ProjectKeyUpdateRequestBrowserSdkVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**Configures multiple options for the Javascript Loader Script.
- `Performance Monitoring`
- `Debug Bundles & Logging`
- `Session Replay` - Note that the loader will load the ES6 bundle instead of the ES5 bundle.
- `User Feedback` - Note that the loader will load the ES6 bundle instead of the ES5 bundle.
- `Logs and Metrics` - Note that the loader will load the ES6 bundle instead of the ES5 bundle. Requires SDK >= 10.0.0.
```json
{
    "dynamicSdkLoaderOptions": {
        "hasReplay": true,
        "hasPerformance": true,
        "hasDebug": true,
        "hasFeedback": true,
        "hasLogsAndMetrics": true
    }
}
```*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyUpdateRequestDynamicSdkLoaderOptions {
    #[serde(rename = "hasDebug", skip_serializing_if = "::std::option::Option::is_none")]
    pub has_debug: ::std::option::Option<bool>,
    #[serde(
        rename = "hasFeedback",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_feedback: ::std::option::Option<bool>,
    #[serde(
        rename = "hasLogsAndMetrics",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_logs_and_metrics: ::std::option::Option<bool>,
    #[serde(
        rename = "hasPerformance",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_performance: ::std::option::Option<bool>,
    #[serde(
        rename = "hasReplay",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_replay: ::std::option::Option<bool>,
}
/**Applies a rate limit to cap the number of errors accepted during a given time window. To
disable entirely set `rateLimit` to null.
```json
{
    "rateLimit": {
        "window": 7200, // time in seconds
        "count": 1000 // error cap
    }
}
```*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyUpdateRequestRateLimit {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub window: ::std::option::Option<i64>,
}
///This represents a Sentry Project Client Key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyUpdateResponse {
    #[serde(rename = "browserSdk")]
    pub browser_sdk: ProjectKeyUpdateResponseBrowserSdk,
    #[serde(rename = "browserSdkVersion")]
    pub browser_sdk_version: ::std::string::String,
    #[serde(
        rename = "dateCreated",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub dsn: ProjectKeyUpdateResponseDsn,
    #[serde(rename = "dynamicSdkLoaderOptions")]
    pub dynamic_sdk_loader_options: ProjectKeyUpdateResponseDynamicSdkLoaderOptions,
    pub id: ::std::string::String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub label: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(rename = "projectId")]
    pub project_id: i64,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub public: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "rateLimit",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub rate_limit: ::std::option::Option<ProjectKeyUpdateResponseRateLimit>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub secret: ::std::option::Option<::std::string::String>,
    #[serde(rename = "useCase", skip_serializing_if = "::std::option::Option::is_none")]
    pub use_case: ::std::option::Option<::std::string::String>,
}
///`ProjectKeyUpdateResponseBrowserSdk`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyUpdateResponseBrowserSdk {
    pub choices: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
}
///`ProjectKeyUpdateResponseDsn`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyUpdateResponseDsn {
    pub cdn: ::std::string::String,
    pub crons: ::std::string::String,
    pub csp: ::std::string::String,
    pub integration: ::std::string::String,
    pub minidump: ::std::string::String,
    pub nel: ::std::string::String,
    pub otlp_logs: ::std::string::String,
    pub otlp_traces: ::std::string::String,
    pub playstation: ::std::string::String,
    pub public: ::std::string::String,
    pub secret: ::std::string::String,
    pub security: ::std::string::String,
    pub unreal: ::std::string::String,
}
///`ProjectKeyUpdateResponseDynamicSdkLoaderOptions`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyUpdateResponseDynamicSdkLoaderOptions {
    #[serde(rename = "hasDebug")]
    pub has_debug: bool,
    #[serde(rename = "hasFeedback")]
    pub has_feedback: bool,
    #[serde(rename = "hasLogsAndMetrics")]
    pub has_logs_and_metrics: bool,
    #[serde(rename = "hasPerformance")]
    pub has_performance: bool,
    #[serde(rename = "hasReplay")]
    pub has_replay: bool,
}
///`ProjectKeyUpdateResponseRateLimit`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectKeyUpdateResponseRateLimit {
    pub count: i64,
    pub window: i64,
}
///`ProjectObserveResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectObserveResponse {
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowedDomains")]
    pub allowed_domains: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "autofixAutomationTuning")]
    pub autofix_automation_tuning: ::std::string::String,
    pub avatar: ProjectObserveResponseAvatar,
    #[serde(rename = "builtinSymbolSources")]
    pub builtin_symbol_sources: ::std::vec::Vec<::std::string::String>,
    pub color: ::std::string::String,
    #[serde(rename = "dataScrubber")]
    pub data_scrubber: bool,
    #[serde(rename = "dataScrubberDefaults")]
    pub data_scrubber_defaults: bool,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "debugFilesRole",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub debug_files_role: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "defaultEnvironment",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub default_environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "derivedGroupingEnhancements")]
    pub derived_grouping_enhancements: ::std::string::String,
    #[serde(rename = "digestsMaxDelay")]
    pub digests_max_delay: i64,
    #[serde(rename = "digestsMinDelay")]
    pub digests_min_delay: i64,
    #[serde(
        rename = "dynamicSamplingBiases",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub dynamic_sampling_biases: ::std::option::Option<
        ::std::vec::Vec<
            ::std::collections::HashMap<
                ::std::string::String,
                ProjectObserveResponseDynamicSamplingBiasesItemValue,
            >,
        >,
    >,
    #[serde(rename = "enableAutoReleaseCreation")]
    pub enable_auto_release_creation: bool,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "fingerprintingRules")]
    pub fingerprinting_rules: ::std::string::String,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "groupingConfig")]
    pub grouping_config: ::std::string::String,
    #[serde(rename = "groupingEnhancements")]
    pub grouping_enhancements: ::std::string::String,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    #[serde(rename = "highlightContext")]
    pub highlight_context: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "highlightPreset")]
    pub highlight_preset: ProjectObserveResponseHighlightPreset,
    #[serde(rename = "highlightTags")]
    pub highlight_tags: ::std::vec::Vec<::std::string::String>,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isDynamicallySampled")]
    pub is_dynamically_sampled: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(
        rename = "latestRelease",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub latest_release: ::std::option::Option<ProjectObserveResponseLatestRelease>,
    pub name: ::std::string::String,
    pub options: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub organization: ProjectObserveResponseOrganization,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    pub platforms: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "processingIssues")]
    pub processing_issues: i64,
    #[serde(
        rename = "relayPiiConfig",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub relay_pii_config: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resolveAge")]
    pub resolve_age: i64,
    #[serde(rename = "safeFields")]
    pub safe_fields: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "scmSourceContextEnabled")]
    pub scm_source_context_enabled: bool,
    #[serde(rename = "scrapeJavaScript")]
    pub scrape_java_script: bool,
    #[serde(rename = "scrubIPAddresses")]
    pub scrub_ip_addresses: bool,
    #[serde(
        rename = "secondaryGroupingConfig",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub secondary_grouping_config: ::std::option::Option<::std::string::String>,
    #[serde(rename = "secondaryGroupingExpiry")]
    pub secondary_grouping_expiry: i64,
    #[serde(rename = "securityToken")]
    pub security_token: ::std::string::String,
    #[serde(
        rename = "securityTokenHeader",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub security_token_header: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seerNightshiftTweaks")]
    pub seer_nightshift_tweaks: ::serde_json::Value,
    #[serde(rename = "seerScannerAutomation")]
    pub seer_scanner_automation: bool,
    #[serde(rename = "sensitiveFields")]
    pub sensitive_fields: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    pub status: ::std::string::String,
    #[serde(
        rename = "storeCrashReports",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub store_crash_reports: ::std::option::Option<i64>,
    #[serde(rename = "subjectPrefix")]
    pub subject_prefix: ::std::string::String,
    #[serde(rename = "subjectTemplate")]
    pub subject_template: ::std::string::String,
    #[serde(rename = "symbolSources")]
    pub symbol_sources: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub team: ::std::option::Option<ProjectObserveResponseTeam>,
    pub teams: ::std::vec::Vec<ProjectObserveResponseTeamsItem>,
    #[serde(rename = "tempestFetchScreenshots")]
    pub tempest_fetch_screenshots: bool,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "verifySSL")]
    pub verify_ssl: bool,
}
///`ProjectObserveResponseAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectObserveResponseAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`ProjectObserveResponseDynamicSamplingBiasesItemValue`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum ProjectObserveResponseDynamicSamplingBiasesItemValue {
    String(::std::string::String),
    Boolean(bool),
}
impl ::std::fmt::Display for ProjectObserveResponseDynamicSamplingBiasesItemValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Boolean(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool>
for ProjectObserveResponseDynamicSamplingBiasesItemValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
///`ProjectObserveResponseHighlightPreset`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectObserveResponseHighlightPreset {
    pub context: ::std::collections::HashMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    pub tags: ::std::vec::Vec<::std::string::String>,
}
///`ProjectObserveResponseLatestRelease`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectObserveResponseLatestRelease {
    pub version: ::std::string::String,
}
///`ProjectObserveResponseOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectObserveResponseOrganization {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowMemberInvite")]
    pub allow_member_invite: bool,
    #[serde(rename = "allowMemberProjectCreation")]
    pub allow_member_project_creation: bool,
    #[serde(rename = "allowSuperuserAccess")]
    pub allow_superuser_access: bool,
    pub avatar: ProjectObserveResponseOrganizationAvatar,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "extraOptions",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extra_options: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "hasAuthProvider")]
    pub has_auth_provider: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isEarlyAdopter")]
    pub is_early_adopter: bool,
    pub links: ProjectObserveResponseOrganizationLinks,
    pub name: ::std::string::String,
    #[serde(
        rename = "onboardingTasks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub onboarding_tasks: ::std::vec::Vec<
        ProjectObserveResponseOrganizationOnboardingTasksItem,
    >,
    #[serde(rename = "require2FA")]
    pub require2_fa: bool,
    pub slug: ::std::string::String,
    pub status: ProjectObserveResponseOrganizationStatus,
}
///`ProjectObserveResponseOrganizationAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectObserveResponseOrganizationAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`ProjectObserveResponseOrganizationLinks`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectObserveResponseOrganizationLinks {
    #[serde(rename = "organizationUrl")]
    pub organization_url: ::std::string::String,
    #[serde(rename = "regionUrl")]
    pub region_url: ::std::string::String,
}
///`ProjectObserveResponseOrganizationOnboardingTasksItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectObserveResponseOrganizationOnboardingTasksItem {
    #[serde(
        rename = "completionSeen",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub completion_seen: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    pub data: ::serde_json::Value,
    #[serde(rename = "dateCompleted")]
    pub date_completed: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub task: ::std::option::Option<::std::string::String>,
}
///`ProjectObserveResponseOrganizationStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectObserveResponseOrganizationStatus {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`ProjectObserveResponseTeam`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectObserveResponseTeam {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`ProjectObserveResponseTeamsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectObserveResponseTeamsItem {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`ProjectUpdateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateRequest {
    ///Origins permitted to send events to this project. Use `*` to allow any.
    #[serde(
        rename = "allowedDomains",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub allowed_domains: ::std::vec::Vec<::std::string::String>,
    /**How aggressively Seer runs Autofix on new issues. Can be updated with **`project:read`** permission.

* `off`
* `super_low`
* `low`
* `medium`
* `high`
* `always`*/
    #[serde(
        rename = "autofixAutomationTuning",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub autofix_automation_tuning: ::std::option::Option<
        ProjectUpdateRequestAutofixAutomationTuning,
    >,
    ///Identifiers of Sentry-hosted symbol sources to use when symbolicating events.
    #[serde(
        rename = "builtinSymbolSources",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub builtin_symbol_sources: ::std::vec::Vec<::std::string::String>,
    ///ID of a project whose settings, teams, and keys should be copied into this one.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub copy_from_project: ::std::option::Option<i64>,
    ///Remove known sensitive values from events before storing them.
    #[serde(
        rename = "dataScrubber",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub data_scrubber: ::std::option::Option<bool>,
    ///Also scrub Sentry's built-in list of sensitive field names.
    #[serde(
        rename = "dataScrubberDefaults",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub data_scrubber_defaults: ::std::option::Option<bool>,
    /**The role required to download debug information files, ProGuard mappings and source maps. If not set, inherits from organization setting.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
    #[serde(
        rename = "debugFilesRole",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub debug_files_role: ::std::option::Option<ProjectUpdateRequestDebugFilesRole>,
    ///Environment selected by default when viewing this project.
    #[serde(
        rename = "defaultEnvironment",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_environment: ::std::option::Option<::std::string::String>,
    ///Maximum time in seconds to wait before sending an issue digest email.
    #[serde(
        rename = "digestsMaxDelay",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub digests_max_delay: ::std::option::Option<i64>,
    ///Minimum time in seconds to wait before sending an issue digest email.
    #[serde(
        rename = "digestsMinDelay",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub digests_min_delay: ::std::option::Option<i64>,
    ///Per-bias toggles adjusting which transactions dynamic sampling favors retaining.
    #[serde(
        rename = "dynamicSamplingBiases",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub dynamic_sampling_biases: ::std::vec::Vec<
        ProjectUpdateRequestDynamicSamplingBiasesItem,
    >,
    ///Automatically create releases from ingested events. When disabled, releases must be created manually (e.g. via the Sentry CLI).
    #[serde(
        rename = "enableAutoReleaseCreation",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub enable_auto_release_creation: ::std::option::Option<bool>,
    ///Fingerprinting rules, as a newline-delimited config string, controlling how events are grouped into issues.
    #[serde(
        rename = "fingerprintingRules",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fingerprinting_rules: ::std::option::Option<::std::string::String>,
    ///Identifier of the grouping algorithm used to assign events to issues.
    #[serde(
        rename = "groupingConfig",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub grouping_config: ::std::option::Option<::std::string::String>,
    ///Grouping enhancement rules, as a newline-delimited config string, adjusting which stack frames contribute to an issue.
    #[serde(
        rename = "groupingEnhancements",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub grouping_enhancements: ::std::option::Option<::std::string::String>,
    /**A JSON mapping of context types to lists of strings for their keys.
E.g. `{'user': ['id', 'email']}`*/
    #[serde(
        rename = "highlightContext",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub highlight_context: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    /**A list of strings with tag keys to highlight on this project's issues.
E.g. `['release', 'environment']`*/
    #[serde(
        rename = "highlightTags",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub highlight_tags: ::std::vec::Vec<::std::string::String>,
    ///Enables starring the project within the projects tab. Can be updated with **`project:read`** permission.
    #[serde(
        rename = "isBookmarked",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_bookmarked: ::std::option::Option<bool>,
    ///The name for the project
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<ProjectUpdateRequestName>,
    ///The platform for the project
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub platform: ::std::option::Option<::std::string::String>,
    ///Enable preprod build distribution. Can be updated with **`project:read`** permission.
    #[serde(
        rename = "preprodDistributionEnabledByCustomer",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_distribution_enabled_by_customer: ::std::option::Option<bool>,
    ///Query selecting which preprod builds are distributed. Can be updated with **`project:read`** permission.
    #[serde(
        rename = "preprodDistributionEnabledQuery",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_distribution_enabled_query: ::std::option::Option<::std::string::String>,
    ///Post preprod distribution updates as PR comments. Can be updated with **`project:read`** permission.
    #[serde(
        rename = "preprodDistributionPrCommentsEnabledByCustomer",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_distribution_pr_comments_enabled_by_customer: ::std::option::Option<
        bool,
    >,
    ///Enable preprod size analysis. Can be updated with **`project:read`** permission.
    #[serde(
        rename = "preprodSizeEnabledByCustomer",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_size_enabled_by_customer: ::std::option::Option<bool>,
    ///Query selecting which preprod builds are size-analyzed. Can be updated with **`project:read`** permission.
    #[serde(
        rename = "preprodSizeEnabledQuery",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_size_enabled_query: ::std::option::Option<::std::string::String>,
    ///Enable preprod size PR comments. Can be updated with **`project:read`** permission.
    #[serde(
        rename = "preprodSizePrCommentsEnabled",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_size_pr_comments_enabled: ::std::option::Option<bool>,
    ///Rules controlling which preprod size changes are posted as PR comments. Can be updated with **`project:read`** permission.
    #[serde(
        rename = "preprodSizePrCommentsRules",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub preprod_size_pr_comments_rules: ::serde_json::Map<
        ::std::string::String,
        ::serde_json::Value,
    >,
    ///Enable preprod size status checks. Can be updated with **`project:read`** permission.
    #[serde(
        rename = "preprodSizeStatusChecksEnabled",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_size_status_checks_enabled: ::std::option::Option<bool>,
    ///Rules controlling when preprod size status checks fail. Can be updated with **`project:read`** permission.
    #[serde(
        rename = "preprodSizeStatusChecksRules",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub preprod_size_status_checks_rules: ::serde_json::Map<
        ::std::string::String,
        ::serde_json::Value,
    >,
    ///Post preprod snapshot changes as PR comments.
    #[serde(
        rename = "preprodSnapshotPrCommentsEnabled",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_snapshot_pr_comments_enabled: ::std::option::Option<bool>,
    ///Include added snapshots in preprod snapshot PR comments.
    #[serde(
        rename = "preprodSnapshotPrCommentsPostOnAdded",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_snapshot_pr_comments_post_on_added: ::std::option::Option<bool>,
    ///Include changed snapshots in preprod snapshot PR comments.
    #[serde(
        rename = "preprodSnapshotPrCommentsPostOnChanged",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_snapshot_pr_comments_post_on_changed: ::std::option::Option<bool>,
    ///Include removed snapshots in preprod snapshot PR comments.
    #[serde(
        rename = "preprodSnapshotPrCommentsPostOnRemoved",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_snapshot_pr_comments_post_on_removed: ::std::option::Option<bool>,
    ///Include renamed snapshots in preprod snapshot PR comments.
    #[serde(
        rename = "preprodSnapshotPrCommentsPostOnRenamed",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_snapshot_pr_comments_post_on_renamed: ::std::option::Option<bool>,
    ///Enable preprod snapshot status checks.
    #[serde(
        rename = "preprodSnapshotStatusChecksEnabled",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_snapshot_status_checks_enabled: ::std::option::Option<bool>,
    ///Fail the preprod snapshot status check when snapshots are added.
    #[serde(
        rename = "preprodSnapshotStatusChecksFailOnAdded",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_snapshot_status_checks_fail_on_added: ::std::option::Option<bool>,
    ///Fail the preprod snapshot status check when snapshots change.
    #[serde(
        rename = "preprodSnapshotStatusChecksFailOnChanged",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_snapshot_status_checks_fail_on_changed: ::std::option::Option<bool>,
    ///Fail the preprod snapshot status check when snapshots are removed.
    #[serde(
        rename = "preprodSnapshotStatusChecksFailOnRemoved",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_snapshot_status_checks_fail_on_removed: ::std::option::Option<bool>,
    ///Fail the preprod snapshot status check when snapshots are renamed.
    #[serde(
        rename = "preprodSnapshotStatusChecksFailOnRenamed",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preprod_snapshot_status_checks_fail_on_renamed: ::std::option::Option<bool>,
    ///Advanced data scrubbing rules, as a JSON string, applied before events are stored.
    #[serde(
        rename = "relayPiiConfig",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub relay_pii_config: ::std::option::Option<::std::string::String>,
    ///Automatically resolve an issue if it hasn't been seen for this many hours. Set to `0` to disable auto-resolve.
    #[serde(
        rename = "resolveAge",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resolve_age: ::std::option::Option<i64>,
    ///Field names to exempt from scrubbing, including the built-in defaults.
    #[serde(
        rename = "safeFields",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub safe_fields: ::std::vec::Vec<::std::string::String>,
    ///Enable on-demand source context fetching from SCM integrations for stack traces.
    #[serde(
        rename = "scmSourceContextEnabled",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scm_source_context_enabled: ::std::option::Option<bool>,
    ///Allow Sentry to fetch source files and source maps from your servers.
    #[serde(
        rename = "scrapeJavaScript",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrape_java_script: ::std::option::Option<bool>,
    ///Discard client IP addresses rather than storing them on events.
    #[serde(
        rename = "scrubIPAddresses",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrub_ip_addresses: ::std::option::Option<bool>,
    ///Grouping algorithm run alongside the primary one during a grouping migration.
    #[serde(
        rename = "secondaryGroupingConfig",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub secondary_grouping_config: ::std::option::Option<::std::string::String>,
    ///Unix timestamp after which the secondary grouping algorithm stops running.
    #[serde(
        rename = "secondaryGroupingExpiry",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub secondary_grouping_expiry: ::std::option::Option<::std::num::NonZeroU64>,
    ///Token sent with security reports (CSP, Expect-CT, HPKP) so Sentry can verify their origin.
    #[serde(
        rename = "securityToken",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub security_token: ::std::option::Option<ProjectUpdateRequestSecurityToken>,
    ///Name of the header carrying the security token on inbound security reports.
    #[serde(
        rename = "securityTokenHeader",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub security_token_header: ::std::option::Option<
        ProjectUpdateRequestSecurityTokenHeader,
    >,
    ///Let Seer scan new issues automatically. Can be updated with **`project:read`** permission.
    #[serde(
        rename = "seerScannerAutomation",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub seer_scanner_automation: ::std::option::Option<bool>,
    ///Additional field names to scrub from events, beyond the defaults.
    #[serde(
        rename = "sensitiveFields",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub sensitive_fields: ::std::vec::Vec<::std::string::String>,
    ///Uniquely identifies a project and is used for the interface.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<ProjectUpdateRequestSlug>,
    ///Number of native crash report files to store per issue. Use `-1` for unlimited or `0` to store none.
    #[serde(
        rename = "storeCrashReports",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub store_crash_reports: ::std::option::Option<i64>,
    ///Custom prefix for emails from this project.
    #[serde(
        rename = "subjectPrefix",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subject_prefix: ::std::option::Option<ProjectUpdateRequestSubjectPrefix>,
    /**The email subject to use (excluding the prefix) for individual alerts. Here are the list of variables you can use:
- `$title`
- `$shortID`
- `$projectID`
- `$orgID`
- `${tag:key}` - such as `${tag:environment}` or `${tag:release}`.*/
    #[serde(
        rename = "subjectTemplate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subject_template: ::std::option::Option<ProjectUpdateRequestSubjectTemplate>,
    ///Custom symbol sources, as a JSON string, to use when symbolicating events.
    #[serde(
        rename = "symbolSources",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub symbol_sources: ::std::option::Option<::std::string::String>,
    ///Target proportion of transactions to retain, from `0` to `1`. Requires manual dynamic sampling mode.
    #[serde(
        rename = "targetSampleRate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_sample_rate: ::std::option::Option<f64>,
    ///Fetch screenshots attached to console crash reports. Requires the Tempest feature.
    #[serde(
        rename = "tempestFetchScreenshots",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tempest_fetch_screenshots: ::std::option::Option<bool>,
    ///Verify SSL certificates when delivering outbound webhooks and service hooks.
    #[serde(
        rename = "verifySSL",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub verify_ssl: ::std::option::Option<bool>,
}
/**How aggressively Seer runs Autofix on new issues. Can be updated with **`project:read`** permission.

* `off`
* `super_low`
* `low`
* `medium`
* `high`
* `always`*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum ProjectUpdateRequestAutofixAutomationTuning {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "super_low")]
    SuperLow,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "always")]
    Always,
}
impl ::std::fmt::Display for ProjectUpdateRequestAutofixAutomationTuning {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Off => f.write_str("off"),
            Self::SuperLow => f.write_str("super_low"),
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Always => f.write_str("always"),
        }
    }
}
impl ::std::str::FromStr for ProjectUpdateRequestAutofixAutomationTuning {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "off" => Ok(Self::Off),
            "super_low" => Ok(Self::SuperLow),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "always" => Ok(Self::Always),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectUpdateRequestAutofixAutomationTuning {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ProjectUpdateRequestAutofixAutomationTuning {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**The role required to download debug information files, ProGuard mappings and source maps. If not set, inherits from organization setting.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum ProjectUpdateRequestDebugFilesRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "owner")]
    Owner,
}
impl ::std::fmt::Display for ProjectUpdateRequestDebugFilesRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Member => f.write_str("member"),
            Self::Admin => f.write_str("admin"),
            Self::Manager => f.write_str("manager"),
            Self::Owner => f.write_str("owner"),
        }
    }
}
impl ::std::str::FromStr for ProjectUpdateRequestDebugFilesRole {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "member" => Ok(Self::Member),
            "admin" => Ok(Self::Admin),
            "manager" => Ok(Self::Manager),
            "owner" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectUpdateRequestDebugFilesRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ProjectUpdateRequestDebugFilesRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ProjectUpdateRequestDynamicSamplingBiasesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateRequestDynamicSamplingBiasesItem {
    #[serde(default)]
    pub active: bool,
    #[doc = "* `boostEnvironments`\n* `boostKeyTransactions`\n* `boostLatestRelease`\n* `boostLowVolumeTransactions`\n* `boostReplayId`\n* `ignoreHealthChecks`\n* `minimumSampleRate`\n* `recalibrationRule`"]
    pub id: ProjectUpdateRequestDynamicSamplingBiasesItemId,
}
#[doc = "* `boostEnvironments`\n* `boostKeyTransactions`\n* `boostLatestRelease`\n* `boostLowVolumeTransactions`\n* `boostReplayId`\n* `ignoreHealthChecks`\n* `minimumSampleRate`\n* `recalibrationRule`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum ProjectUpdateRequestDynamicSamplingBiasesItemId {
    #[serde(rename = "boostEnvironments")]
    BoostEnvironments,
    #[serde(rename = "boostKeyTransactions")]
    BoostKeyTransactions,
    #[serde(rename = "boostLatestRelease")]
    BoostLatestRelease,
    #[serde(rename = "boostLowVolumeTransactions")]
    BoostLowVolumeTransactions,
    #[serde(rename = "boostReplayId")]
    BoostReplayId,
    #[serde(rename = "ignoreHealthChecks")]
    IgnoreHealthChecks,
    #[serde(rename = "minimumSampleRate")]
    MinimumSampleRate,
    #[serde(rename = "recalibrationRule")]
    RecalibrationRule,
}
impl ::std::fmt::Display for ProjectUpdateRequestDynamicSamplingBiasesItemId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::BoostEnvironments => f.write_str("boostEnvironments"),
            Self::BoostKeyTransactions => f.write_str("boostKeyTransactions"),
            Self::BoostLatestRelease => f.write_str("boostLatestRelease"),
            Self::BoostLowVolumeTransactions => f.write_str("boostLowVolumeTransactions"),
            Self::BoostReplayId => f.write_str("boostReplayId"),
            Self::IgnoreHealthChecks => f.write_str("ignoreHealthChecks"),
            Self::MinimumSampleRate => f.write_str("minimumSampleRate"),
            Self::RecalibrationRule => f.write_str("recalibrationRule"),
        }
    }
}
impl ::std::str::FromStr for ProjectUpdateRequestDynamicSamplingBiasesItemId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "boostEnvironments" => Ok(Self::BoostEnvironments),
            "boostKeyTransactions" => Ok(Self::BoostKeyTransactions),
            "boostLatestRelease" => Ok(Self::BoostLatestRelease),
            "boostLowVolumeTransactions" => Ok(Self::BoostLowVolumeTransactions),
            "boostReplayId" => Ok(Self::BoostReplayId),
            "ignoreHealthChecks" => Ok(Self::IgnoreHealthChecks),
            "minimumSampleRate" => Ok(Self::MinimumSampleRate),
            "recalibrationRule" => Ok(Self::RecalibrationRule),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProjectUpdateRequestDynamicSamplingBiasesItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ProjectUpdateRequestDynamicSamplingBiasesItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The name for the project
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ProjectUpdateRequestName(::std::string::String);
impl ::std::ops::Deref for ProjectUpdateRequestName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProjectUpdateRequestName> for ::std::string::String {
    fn from(value: ProjectUpdateRequestName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProjectUpdateRequestName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 200usize {
            return Err("longer than 200 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProjectUpdateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectUpdateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProjectUpdateRequestName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Token sent with security reports (CSP, Expect-CT, HPKP) so Sentry can verify their origin.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ProjectUpdateRequestSecurityToken(::std::string::String);
impl ::std::ops::Deref for ProjectUpdateRequestSecurityToken {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProjectUpdateRequestSecurityToken> for ::std::string::String {
    fn from(value: ProjectUpdateRequestSecurityToken) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProjectUpdateRequestSecurityToken {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[-a-zA-Z0-9+/=\\s]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[-a-zA-Z0-9+/=\\s]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProjectUpdateRequestSecurityToken {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ProjectUpdateRequestSecurityToken {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProjectUpdateRequestSecurityToken {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Name of the header carrying the security token on inbound security reports.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ProjectUpdateRequestSecurityTokenHeader(::std::string::String);
impl ::std::ops::Deref for ProjectUpdateRequestSecurityTokenHeader {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProjectUpdateRequestSecurityTokenHeader>
for ::std::string::String {
    fn from(value: ProjectUpdateRequestSecurityTokenHeader) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProjectUpdateRequestSecurityTokenHeader {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-zA-Z0-9_\\-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9_\\-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProjectUpdateRequestSecurityTokenHeader {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ProjectUpdateRequestSecurityTokenHeader {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProjectUpdateRequestSecurityTokenHeader {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Uniquely identifies a project and is used for the interface.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ProjectUpdateRequestSlug(::std::string::String);
impl ::std::ops::Deref for ProjectUpdateRequestSlug {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProjectUpdateRequestSlug> for ::std::string::String {
    fn from(value: ProjectUpdateRequestSlug) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProjectUpdateRequestSlug {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^(?![0-9]+$)[a-z0-9_\\-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(?![0-9]+$)[a-z0-9_\\-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProjectUpdateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectUpdateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProjectUpdateRequestSlug {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Custom prefix for emails from this project.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ProjectUpdateRequestSubjectPrefix(::std::string::String);
impl ::std::ops::Deref for ProjectUpdateRequestSubjectPrefix {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProjectUpdateRequestSubjectPrefix> for ::std::string::String {
    fn from(value: ProjectUpdateRequestSubjectPrefix) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProjectUpdateRequestSubjectPrefix {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 200usize {
            return Err("longer than 200 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProjectUpdateRequestSubjectPrefix {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ProjectUpdateRequestSubjectPrefix {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProjectUpdateRequestSubjectPrefix {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**The email subject to use (excluding the prefix) for individual alerts. Here are the list of variables you can use:
- `$title`
- `$shortID`
- `$projectID`
- `$orgID`
- `${tag:key}` - such as `${tag:environment}` or `${tag:release}`.*/
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ProjectUpdateRequestSubjectTemplate(::std::string::String);
impl ::std::ops::Deref for ProjectUpdateRequestSubjectTemplate {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProjectUpdateRequestSubjectTemplate>
for ::std::string::String {
    fn from(value: ProjectUpdateRequestSubjectTemplate) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProjectUpdateRequestSubjectTemplate {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 200usize {
            return Err("longer than 200 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProjectUpdateRequestSubjectTemplate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ProjectUpdateRequestSubjectTemplate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProjectUpdateRequestSubjectTemplate {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`ProjectUpdateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateResponse {
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowedDomains")]
    pub allowed_domains: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "autofixAutomationTuning")]
    pub autofix_automation_tuning: ::std::string::String,
    pub avatar: ProjectUpdateResponseAvatar,
    #[serde(rename = "builtinSymbolSources")]
    pub builtin_symbol_sources: ::std::vec::Vec<::std::string::String>,
    pub color: ::std::string::String,
    #[serde(rename = "dataScrubber")]
    pub data_scrubber: bool,
    #[serde(rename = "dataScrubberDefaults")]
    pub data_scrubber_defaults: bool,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "debugFilesRole",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub debug_files_role: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "defaultEnvironment",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub default_environment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "derivedGroupingEnhancements")]
    pub derived_grouping_enhancements: ::std::string::String,
    #[serde(rename = "digestsMaxDelay")]
    pub digests_max_delay: i64,
    #[serde(rename = "digestsMinDelay")]
    pub digests_min_delay: i64,
    #[serde(
        rename = "dynamicSamplingBiases",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub dynamic_sampling_biases: ::std::option::Option<
        ::std::vec::Vec<
            ::std::collections::HashMap<
                ::std::string::String,
                ProjectUpdateResponseDynamicSamplingBiasesItemValue,
            >,
        >,
    >,
    #[serde(rename = "enableAutoReleaseCreation")]
    pub enable_auto_release_creation: bool,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "fingerprintingRules")]
    pub fingerprinting_rules: ::std::string::String,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "groupingConfig")]
    pub grouping_config: ::std::string::String,
    #[serde(rename = "groupingEnhancements")]
    pub grouping_enhancements: ::std::string::String,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    #[serde(rename = "highlightContext")]
    pub highlight_context: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "highlightPreset")]
    pub highlight_preset: ProjectUpdateResponseHighlightPreset,
    #[serde(rename = "highlightTags")]
    pub highlight_tags: ::std::vec::Vec<::std::string::String>,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isDynamicallySampled")]
    pub is_dynamically_sampled: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(
        rename = "latestRelease",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub latest_release: ::std::option::Option<ProjectUpdateResponseLatestRelease>,
    pub name: ::std::string::String,
    pub options: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub organization: ProjectUpdateResponseOrganization,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    pub platforms: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "processingIssues")]
    pub processing_issues: i64,
    #[serde(
        rename = "relayPiiConfig",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub relay_pii_config: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resolveAge")]
    pub resolve_age: i64,
    #[serde(rename = "safeFields")]
    pub safe_fields: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "scmSourceContextEnabled")]
    pub scm_source_context_enabled: bool,
    #[serde(rename = "scrapeJavaScript")]
    pub scrape_java_script: bool,
    #[serde(rename = "scrubIPAddresses")]
    pub scrub_ip_addresses: bool,
    #[serde(
        rename = "secondaryGroupingConfig",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub secondary_grouping_config: ::std::option::Option<::std::string::String>,
    #[serde(rename = "secondaryGroupingExpiry")]
    pub secondary_grouping_expiry: i64,
    #[serde(rename = "securityToken")]
    pub security_token: ::std::string::String,
    #[serde(
        rename = "securityTokenHeader",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub security_token_header: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seerNightshiftTweaks")]
    pub seer_nightshift_tweaks: ::serde_json::Value,
    #[serde(rename = "seerScannerAutomation")]
    pub seer_scanner_automation: bool,
    #[serde(rename = "sensitiveFields")]
    pub sensitive_fields: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    pub status: ::std::string::String,
    #[serde(
        rename = "storeCrashReports",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub store_crash_reports: ::std::option::Option<i64>,
    #[serde(rename = "subjectPrefix")]
    pub subject_prefix: ::std::string::String,
    #[serde(rename = "subjectTemplate")]
    pub subject_template: ::std::string::String,
    #[serde(rename = "symbolSources")]
    pub symbol_sources: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub team: ::std::option::Option<ProjectUpdateResponseTeam>,
    pub teams: ::std::vec::Vec<ProjectUpdateResponseTeamsItem>,
    #[serde(rename = "tempestFetchScreenshots")]
    pub tempest_fetch_screenshots: bool,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "verifySSL")]
    pub verify_ssl: bool,
}
///`ProjectUpdateResponseAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateResponseAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`ProjectUpdateResponseDynamicSamplingBiasesItemValue`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum ProjectUpdateResponseDynamicSamplingBiasesItemValue {
    String(::std::string::String),
    Boolean(bool),
}
impl ::std::fmt::Display for ProjectUpdateResponseDynamicSamplingBiasesItemValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Boolean(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for ProjectUpdateResponseDynamicSamplingBiasesItemValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
///`ProjectUpdateResponseHighlightPreset`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateResponseHighlightPreset {
    pub context: ::std::collections::HashMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    pub tags: ::std::vec::Vec<::std::string::String>,
}
///`ProjectUpdateResponseLatestRelease`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateResponseLatestRelease {
    pub version: ::std::string::String,
}
///`ProjectUpdateResponseOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateResponseOrganization {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowMemberInvite")]
    pub allow_member_invite: bool,
    #[serde(rename = "allowMemberProjectCreation")]
    pub allow_member_project_creation: bool,
    #[serde(rename = "allowSuperuserAccess")]
    pub allow_superuser_access: bool,
    pub avatar: ProjectUpdateResponseOrganizationAvatar,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "extraOptions",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extra_options: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "hasAuthProvider")]
    pub has_auth_provider: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isEarlyAdopter")]
    pub is_early_adopter: bool,
    pub links: ProjectUpdateResponseOrganizationLinks,
    pub name: ::std::string::String,
    #[serde(
        rename = "onboardingTasks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub onboarding_tasks: ::std::vec::Vec<
        ProjectUpdateResponseOrganizationOnboardingTasksItem,
    >,
    #[serde(rename = "require2FA")]
    pub require2_fa: bool,
    pub slug: ::std::string::String,
    pub status: ProjectUpdateResponseOrganizationStatus,
}
///`ProjectUpdateResponseOrganizationAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateResponseOrganizationAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`ProjectUpdateResponseOrganizationLinks`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateResponseOrganizationLinks {
    #[serde(rename = "organizationUrl")]
    pub organization_url: ::std::string::String,
    #[serde(rename = "regionUrl")]
    pub region_url: ::std::string::String,
}
///`ProjectUpdateResponseOrganizationOnboardingTasksItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateResponseOrganizationOnboardingTasksItem {
    #[serde(
        rename = "completionSeen",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub completion_seen: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    pub data: ::serde_json::Value,
    #[serde(rename = "dateCompleted")]
    pub date_completed: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub task: ::std::option::Option<::std::string::String>,
}
///`ProjectUpdateResponseOrganizationStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateResponseOrganizationStatus {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`ProjectUpdateResponseTeam`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateResponseTeam {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`ProjectUpdateResponseTeamsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ProjectUpdateResponseTeamsItem {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`DashboardAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProvider {
    #[serde(
        rename = "createdBy",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_by: ::std::option::Option<SentryV1alpha1DashboardAtProviderCreatedBy>,
    #[serde(
        rename = "dateCreated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_created: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub environment: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expired: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filters: ::std::option::Option<SentryV1alpha1DashboardAtProviderFilters>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isFavorited",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorited: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub permissions: ::std::option::Option<SentryV1alpha1DashboardAtProviderPermissions>,
    #[serde(
        rename = "prebuiltId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prebuilt_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub projects: ::std::vec::Vec<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub utc: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub widgets: ::std::vec::Vec<SentryV1alpha1DashboardAtProviderWidgetsItem>,
}
///`DashboardAtProviderCreatedBy`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderCreatedBy {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub authenticators: ::std::vec::Vec<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<SentryV1alpha1DashboardAtProviderCreatedByAvatar>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: ::std::string::String,
    #[serde(
        rename = "canReset2fa",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub can_reset2fa: ::std::option::Option<bool>,
    #[serde(rename = "dateJoined")]
    pub date_joined: ::chrono::DateTime<::chrono::offset::Utc>,
    pub email: ::std::string::String,
    pub emails: ::std::vec::Vec<SentryV1alpha1DashboardAtProviderCreatedByEmailsItem>,
    pub experiments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub has2fa: bool,
    #[serde(rename = "hasPasswordAuth")]
    pub has_password_auth: bool,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub identities: ::std::vec::Vec<
        SentryV1alpha1DashboardAtProviderCreatedByIdentitiesItem,
    >,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    #[serde(rename = "isStaff")]
    pub is_staff: bool,
    #[serde(rename = "isSuperuser")]
    pub is_superuser: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(
        rename = "lastActive",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_active: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "lastLogin",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_login: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub name: ::std::string::String,
    pub username: ::std::string::String,
}
///`DashboardAtProviderCreatedByAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderCreatedByAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`DashboardAtProviderCreatedByEmailsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderCreatedByEmailsItem {
    pub email: ::std::string::String,
    pub id: ::std::string::String,
    pub is_verified: bool,
}
///`DashboardAtProviderCreatedByIdentitiesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderCreatedByIdentitiesItem {
    #[serde(rename = "dateSynced")]
    pub date_synced: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "dateVerified")]
    pub date_verified: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub organization: SentryV1alpha1DashboardAtProviderCreatedByIdentitiesItemOrganization,
    pub provider: SentryV1alpha1DashboardAtProviderCreatedByIdentitiesItemProvider,
}
///`DashboardAtProviderCreatedByIdentitiesItemOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderCreatedByIdentitiesItemOrganization {
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`DashboardAtProviderCreatedByIdentitiesItemProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderCreatedByIdentitiesItemProvider {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`DashboardAtProviderFilters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderFilters {
    #[serde(
        rename = "globalFilter",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub global_filter: ::std::vec::Vec<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub release: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "releaseId",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub release_id: ::std::vec::Vec<::std::string::String>,
}
///`DashboardAtProviderPermissions`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderPermissions {
    #[serde(rename = "isEditableByEveryone")]
    pub is_editable_by_everyone: bool,
    #[serde(rename = "teamsWithEditAccess")]
    pub teams_with_edit_access: ::std::vec::Vec<i64>,
}
///`DashboardAtProviderWidgetsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderWidgetsItem {
    #[serde(
        rename = "axisRange",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub axis_range: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "changedReason",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub changed_reason: ::std::option::Option<
        ::std::vec::Vec<SentryV1alpha1DashboardAtProviderWidgetsItemChangedReasonItem>,
    >,
    #[serde(rename = "dashboardId")]
    pub dashboard_id: ::std::string::String,
    #[serde(
        rename = "datasetSource",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub dataset_source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dateCreated")]
    pub date_created: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayType")]
    pub display_type: ::std::string::String,
    #[serde(
        rename = "exploreUrls",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub explore_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub id: ::std::string::String,
    pub interval: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub layout: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, i64>,
    >,
    #[serde(
        rename = "legendType",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub legend_type: ::std::option::Option<
        SentryV1alpha1DashboardAtProviderWidgetsItemLegendType,
    >,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub limit: ::std::option::Option<i64>,
    pub queries: ::std::vec::Vec<
        SentryV1alpha1DashboardAtProviderWidgetsItemQueriesItem,
    >,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub thresholds: ::std::option::Option<
        SentryV1alpha1DashboardAtProviderWidgetsItemThresholds,
    >,
    pub title: ::std::string::String,
    #[serde(
        rename = "widgetType",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub widget_type: ::std::option::Option<::std::string::String>,
}
///`DashboardAtProviderWidgetsItemChangedReasonItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderWidgetsItemChangedReasonItem {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub equations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::collections::HashMap<
                ::std::string::String,
                SentryV1alpha1DashboardAtProviderWidgetsItemChangedReasonItemEquationsItemValue,
            >,
        >,
    >,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub orderby: ::std::option::Option<
        ::std::vec::Vec<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
    pub selected_columns: ::std::vec::Vec<::std::string::String>,
}
///`DashboardAtProviderWidgetsItemChangedReasonItemEquationsItemValue`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum SentryV1alpha1DashboardAtProviderWidgetsItemChangedReasonItemEquationsItemValue {
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>>
for SentryV1alpha1DashboardAtProviderWidgetsItemChangedReasonItemEquationsItemValue {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
///`DashboardAtProviderWidgetsItemLegendType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1DashboardAtProviderWidgetsItemLegendType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "breakdown")]
    Breakdown,
}
impl ::std::fmt::Display for SentryV1alpha1DashboardAtProviderWidgetsItemLegendType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("default"),
            Self::Breakdown => f.write_str("breakdown"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1DashboardAtProviderWidgetsItemLegendType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "breakdown" => Ok(Self::Breakdown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for SentryV1alpha1DashboardAtProviderWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1DashboardAtProviderWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DashboardAtProviderWidgetsItemQueriesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderWidgetsItemQueriesItem {
    pub aggregates: ::std::vec::Vec<::std::string::String>,
    pub columns: ::std::vec::Vec<::std::string::String>,
    pub conditions: ::std::string::String,
    #[serde(rename = "fieldAliases")]
    pub field_aliases: ::std::vec::Vec<::std::string::String>,
    pub fields: ::std::vec::Vec<::std::string::String>,
    pub id: ::std::string::String,
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    #[serde(rename = "linkedDashboards")]
    pub linked_dashboards: ::std::vec::Vec<
        SentryV1alpha1DashboardAtProviderWidgetsItemQueriesItemLinkedDashboardsItem,
    >,
    pub name: ::std::string::String,
    #[serde(rename = "onDemand")]
    pub on_demand: ::std::vec::Vec<
        SentryV1alpha1DashboardAtProviderWidgetsItemQueriesItemOnDemandItem,
    >,
    pub orderby: ::std::string::String,
    #[serde(
        rename = "selectedAggregate",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub selected_aggregate: ::std::option::Option<i64>,
    #[serde(rename = "widgetId")]
    pub widget_id: ::std::string::String,
}
///`DashboardAtProviderWidgetsItemQueriesItemLinkedDashboardsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderWidgetsItemQueriesItemLinkedDashboardsItem {
    #[serde(rename = "dashboardId")]
    pub dashboard_id: i64,
    pub field: ::std::string::String,
}
///`DashboardAtProviderWidgetsItemQueriesItemOnDemandItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderWidgetsItemQueriesItemOnDemandItem {
    #[serde(rename = "dashboardWidgetQueryId")]
    pub dashboard_widget_query_id: i64,
    pub enabled: bool,
    #[serde(rename = "extractionState")]
    pub extraction_state: ::std::string::String,
}
///`DashboardAtProviderWidgetsItemThresholds`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardAtProviderWidgetsItemThresholds {
    pub max_values: ::std::collections::HashMap<::std::string::String, i64>,
    #[serde(
        rename = "preferredPolarity",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preferred_polarity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "timeWindow",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub time_window: ::std::option::Option<::std::string::String>,
    pub unit: ::std::string::String,
}
///`DashboardForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardForProvider {
    ///The saved end time for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///The saved environment filter for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub environment: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ///The saved filters for this dashboard.
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub filters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///A dashboard's unique id.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    ///Favorite the dashboard automatically for the request user
    #[serde(default)]
    pub is_favorited: bool,
    ///The saved time range period for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub permissions: ::std::option::Option<
        SentryV1alpha1DashboardForProviderPermissions,
    >,
    ///The saved projects filter for this dashboard.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub projects: ::std::vec::Vec<i64>,
    ///The saved start time for this dashboard.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///The user defined title for this dashboard.
    pub title: SentryV1alpha1DashboardForProviderTitle,
    ///Setting that lets you display saved time range for this dashboard in UTC.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub utc: ::std::option::Option<bool>,
    ///A json list of widgets saved in this dashboard.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub widgets: ::std::vec::Vec<SentryV1alpha1DashboardForProviderWidgetsItem>,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardForProviderPermissions {
    ///Whether the dashboard is editable by everyone.
    pub is_editable_by_everyone: bool,
    ///List of team IDs that have edit access to a dashboard.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub teams_with_edit_access: ::std::vec::Vec<i64>,
}
///The user defined title for this dashboard.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1DashboardForProviderTitle(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1DashboardForProviderTitle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1DashboardForProviderTitle>
for ::std::string::String {
    fn from(value: SentryV1alpha1DashboardForProviderTitle) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1DashboardForProviderTitle {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1DashboardForProviderTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1DashboardForProviderTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SentryV1alpha1DashboardForProviderTitle {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardForProviderWidgetsItem {
    #[doc = "* `auto`\n* `dataMin`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub axis_range: ::std::option::Option<
        SentryV1alpha1DashboardForProviderWidgetsItemAxisRange,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "* `line`\n* `area`\n* `bar`\n* `table`\n* `big_number`\n* `details`\n* `categorical_bar`\n* `wheel`\n* `rage_and_dead_clicks`\n* `server_tree`\n* `text`\n* `agents_traces_table`\n* `heatmap`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub display_type: ::std::option::Option<
        SentryV1alpha1DashboardForProviderWidgetsItemDisplayType,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub interval: ::std::option::Option<
        SentryV1alpha1DashboardForProviderWidgetsItemInterval,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub layout: ::std::option::Option<
        SentryV1alpha1DashboardForProviderWidgetsItemLayout,
    >,
    #[doc = "* `default`\n* `breakdown`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub legend_type: ::std::option::Option<
        SentryV1alpha1DashboardForProviderWidgetsItemLegendType,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub queries: ::std::vec::Vec<
        SentryV1alpha1DashboardForProviderWidgetsItemQueriesItem,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub thresholds: ::std::option::Option<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<SentryV1alpha1DashboardForProviderWidgetsItemTitle>,
    #[doc = "* `discover`\n* `issue`\n* `metrics`\n* `error-events`\n* `transaction-like`\n* `spans`\n* `logs`\n* `tracemetrics`\n* `preprod-app-size`"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub widget_type: ::std::option::Option<
        SentryV1alpha1DashboardForProviderWidgetsItemWidgetType,
    >,
}
#[doc = "* `auto`\n* `dataMin`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1DashboardForProviderWidgetsItemAxisRange {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "dataMin")]
    DataMin,
}
impl ::std::fmt::Display for SentryV1alpha1DashboardForProviderWidgetsItemAxisRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::DataMin => f.write_str("dataMin"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1DashboardForProviderWidgetsItemAxisRange {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "dataMin" => Ok(Self::DataMin),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for SentryV1alpha1DashboardForProviderWidgetsItemAxisRange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1DashboardForProviderWidgetsItemAxisRange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "* `line`\n* `area`\n* `bar`\n* `table`\n* `big_number`\n* `details`\n* `categorical_bar`\n* `wheel`\n* `rage_and_dead_clicks`\n* `server_tree`\n* `text`\n* `agents_traces_table`\n* `heatmap`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1DashboardForProviderWidgetsItemDisplayType {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "area")]
    Area,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "big_number")]
    BigNumber,
    #[serde(rename = "details")]
    Details,
    #[serde(rename = "categorical_bar")]
    CategoricalBar,
    #[serde(rename = "wheel")]
    Wheel,
    #[serde(rename = "rage_and_dead_clicks")]
    RageAndDeadClicks,
    #[serde(rename = "server_tree")]
    ServerTree,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "agents_traces_table")]
    AgentsTracesTable,
    #[serde(rename = "heatmap")]
    Heatmap,
}
impl ::std::fmt::Display for SentryV1alpha1DashboardForProviderWidgetsItemDisplayType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Line => f.write_str("line"),
            Self::Area => f.write_str("area"),
            Self::Bar => f.write_str("bar"),
            Self::Table => f.write_str("table"),
            Self::BigNumber => f.write_str("big_number"),
            Self::Details => f.write_str("details"),
            Self::CategoricalBar => f.write_str("categorical_bar"),
            Self::Wheel => f.write_str("wheel"),
            Self::RageAndDeadClicks => f.write_str("rage_and_dead_clicks"),
            Self::ServerTree => f.write_str("server_tree"),
            Self::Text => f.write_str("text"),
            Self::AgentsTracesTable => f.write_str("agents_traces_table"),
            Self::Heatmap => f.write_str("heatmap"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1DashboardForProviderWidgetsItemDisplayType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "line" => Ok(Self::Line),
            "area" => Ok(Self::Area),
            "bar" => Ok(Self::Bar),
            "table" => Ok(Self::Table),
            "big_number" => Ok(Self::BigNumber),
            "details" => Ok(Self::Details),
            "categorical_bar" => Ok(Self::CategoricalBar),
            "wheel" => Ok(Self::Wheel),
            "rage_and_dead_clicks" => Ok(Self::RageAndDeadClicks),
            "server_tree" => Ok(Self::ServerTree),
            "text" => Ok(Self::Text),
            "agents_traces_table" => Ok(Self::AgentsTracesTable),
            "heatmap" => Ok(Self::Heatmap),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for SentryV1alpha1DashboardForProviderWidgetsItemDisplayType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1DashboardForProviderWidgetsItemDisplayType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DashboardForProviderWidgetsItemInterval`
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1DashboardForProviderWidgetsItemInterval(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1DashboardForProviderWidgetsItemInterval {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1DashboardForProviderWidgetsItemInterval>
for ::std::string::String {
    fn from(value: SentryV1alpha1DashboardForProviderWidgetsItemInterval) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1DashboardForProviderWidgetsItemInterval {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10usize {
            return Err("longer than 10 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
for SentryV1alpha1DashboardForProviderWidgetsItemInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1DashboardForProviderWidgetsItemInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
for SentryV1alpha1DashboardForProviderWidgetsItemInterval {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Widget grid layout position and dimensions for dashboard creation.

The dashboard uses a 6-column grid. Required keys: x, y, w, h.
Constraints: x (0-5), y (>= 0), w (1-6), h (>= 1), and x + w <= 6.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardForProviderWidgetsItemLayout {
    ///Height in grid rows.
    pub h: ::std::num::NonZeroU64,
    ///Width in grid columns (1-6).
    pub w: ::std::num::NonZeroU64,
    ///Column position (0-indexed).
    pub x: i64,
    ///Row position (0-indexed).
    pub y: u64,
}
#[doc = "* `default`\n* `breakdown`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1DashboardForProviderWidgetsItemLegendType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "breakdown")]
    Breakdown,
}
impl ::std::fmt::Display for SentryV1alpha1DashboardForProviderWidgetsItemLegendType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("default"),
            Self::Breakdown => f.write_str("breakdown"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1DashboardForProviderWidgetsItemLegendType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "breakdown" => Ok(Self::Breakdown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for SentryV1alpha1DashboardForProviderWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1DashboardForProviderWidgetsItemLegendType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardForProviderWidgetsItemQueriesItem {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub aggregates: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub columns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub conditions: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub field_aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub fields: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_hidden: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub linked_dashboards: ::std::option::Option<
        ::std::vec::Vec<
            SentryV1alpha1DashboardForProviderWidgetsItemQueriesItemLinkedDashboardsItem,
        >,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub on_demand_extraction: ::std::option::Option<
        SentryV1alpha1DashboardForProviderWidgetsItemQueriesItemOnDemandExtraction,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub on_demand_extraction_disabled: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub orderby: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub selected_aggregate: ::std::option::Option<i64>,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardForProviderWidgetsItemQueriesItemLinkedDashboardsItem {
    pub dashboard_id: ::std::string::String,
    pub field: ::std::string::String,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1DashboardForProviderWidgetsItemQueriesItemOnDemandExtraction {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub extraction_state: ::std::option::Option<::std::string::String>,
}
///`DashboardForProviderWidgetsItemTitle`
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1DashboardForProviderWidgetsItemTitle(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1DashboardForProviderWidgetsItemTitle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1DashboardForProviderWidgetsItemTitle>
for ::std::string::String {
    fn from(value: SentryV1alpha1DashboardForProviderWidgetsItemTitle) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1DashboardForProviderWidgetsItemTitle {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
for SentryV1alpha1DashboardForProviderWidgetsItemTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1DashboardForProviderWidgetsItemTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
for SentryV1alpha1DashboardForProviderWidgetsItemTitle {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "* `discover`\n* `issue`\n* `metrics`\n* `error-events`\n* `transaction-like`\n* `spans`\n* `logs`\n* `tracemetrics`\n* `preprod-app-size`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1DashboardForProviderWidgetsItemWidgetType {
    #[serde(rename = "discover")]
    Discover,
    #[serde(rename = "issue")]
    Issue,
    #[serde(rename = "metrics")]
    Metrics,
    #[serde(rename = "error-events")]
    ErrorEvents,
    #[serde(rename = "transaction-like")]
    TransactionLike,
    #[serde(rename = "spans")]
    Spans,
    #[serde(rename = "logs")]
    Logs,
    #[serde(rename = "tracemetrics")]
    Tracemetrics,
    #[serde(rename = "preprod-app-size")]
    PreprodAppSize,
}
impl ::std::fmt::Display for SentryV1alpha1DashboardForProviderWidgetsItemWidgetType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Discover => f.write_str("discover"),
            Self::Issue => f.write_str("issue"),
            Self::Metrics => f.write_str("metrics"),
            Self::ErrorEvents => f.write_str("error-events"),
            Self::TransactionLike => f.write_str("transaction-like"),
            Self::Spans => f.write_str("spans"),
            Self::Logs => f.write_str("logs"),
            Self::Tracemetrics => f.write_str("tracemetrics"),
            Self::PreprodAppSize => f.write_str("preprod-app-size"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1DashboardForProviderWidgetsItemWidgetType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "discover" => Ok(Self::Discover),
            "issue" => Ok(Self::Issue),
            "metrics" => Ok(Self::Metrics),
            "error-events" => Ok(Self::ErrorEvents),
            "transaction-like" => Ok(Self::TransactionLike),
            "spans" => Ok(Self::Spans),
            "logs" => Ok(Self::Logs),
            "tracemetrics" => Ok(Self::Tracemetrics),
            "preprod-app-size" => Ok(Self::PreprodAppSize),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for SentryV1alpha1DashboardForProviderWidgetsItemWidgetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1DashboardForProviderWidgetsItemWidgetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MonitorAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorAtProvider {
    #[serde(
        rename = "alertRule",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub alert_rule: ::std::option::Option<SentryV1alpha1MonitorAtProviderAlertRule>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub config: ::std::option::Option<SentryV1alpha1MonitorAtProviderConfig>,
    #[serde(
        rename = "dateCreated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub environments: ::std::option::Option<SentryV1alpha1MonitorAtProviderEnvironments>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isMuted", skip_serializing_if = "::std::option::Option::is_none")]
    pub is_muted: ::std::option::Option<bool>,
    #[serde(
        rename = "isUpserting",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_upserting: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<SentryV1alpha1MonitorAtProviderOwner>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project: ::std::option::Option<SentryV1alpha1MonitorAtProviderProject>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
}
///`MonitorAtProviderAlertRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorAtProviderAlertRule {
    pub environment: ::std::string::String,
    pub targets: ::std::vec::Vec<SentryV1alpha1MonitorAtProviderAlertRuleTargetsItem>,
}
///`MonitorAtProviderAlertRuleTargetsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorAtProviderAlertRuleTargetsItem {
    #[serde(rename = "targetIdentifier")]
    pub target_identifier: i64,
    #[serde(rename = "targetType")]
    pub target_type: ::std::string::String,
}
///`MonitorAtProviderConfig`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorAtProviderConfig {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub alert_rule_id: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub checkin_margin: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub failure_issue_threshold: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub max_runtime: ::std::option::Option<i64>,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub recovery_threshold: ::std::option::Option<i64>,
    pub schedule: SentryV1alpha1MonitorAtProviderConfigSchedule,
    pub schedule_type: SentryV1alpha1MonitorAtProviderConfigScheduleType,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub timezone: ::std::option::Option<::std::string::String>,
}
///`MonitorAtProviderConfigSchedule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum SentryV1alpha1MonitorAtProviderConfigSchedule {
    String(::std::string::String),
    Array(::std::vec::Vec<i64>),
}
impl ::std::convert::From<::std::vec::Vec<i64>>
for SentryV1alpha1MonitorAtProviderConfigSchedule {
    fn from(value: ::std::vec::Vec<i64>) -> Self {
        Self::Array(value)
    }
}
///`MonitorAtProviderConfigScheduleType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1MonitorAtProviderConfigScheduleType {
    #[serde(rename = "crontab")]
    Crontab,
    #[serde(rename = "interval")]
    Interval,
}
impl ::std::fmt::Display for SentryV1alpha1MonitorAtProviderConfigScheduleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Crontab => f.write_str("crontab"),
            Self::Interval => f.write_str("interval"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1MonitorAtProviderConfigScheduleType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "crontab" => Ok(Self::Crontab),
            "interval" => Ok(Self::Interval),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for SentryV1alpha1MonitorAtProviderConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1MonitorAtProviderConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MonitorAtProviderEnvironments`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorAtProviderEnvironments {
    #[serde(
        rename = "activeIncident",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub active_incident: ::std::option::Option<
        SentryV1alpha1MonitorAtProviderEnvironmentsActiveIncident,
    >,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    #[serde(rename = "lastCheckIn")]
    pub last_check_in: ::chrono::DateTime<::chrono::offset::Utc>,
    pub name: ::std::string::String,
    #[serde(rename = "nextCheckIn")]
    pub next_check_in: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "nextCheckInLatest")]
    pub next_check_in_latest: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
}
///`MonitorAtProviderEnvironmentsActiveIncident`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorAtProviderEnvironmentsActiveIncident {
    #[serde(
        rename = "brokenNotice",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub broken_notice: ::std::option::Option<
        SentryV1alpha1MonitorAtProviderEnvironmentsActiveIncidentBrokenNotice,
    >,
    #[serde(rename = "resolvingTimestamp")]
    pub resolving_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "startingTimestamp")]
    pub starting_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
///`MonitorAtProviderEnvironmentsActiveIncidentBrokenNotice`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorAtProviderEnvironmentsActiveIncidentBrokenNotice {
    #[serde(rename = "environmentMutedTimestamp")]
    pub environment_muted_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "userNotifiedTimestamp")]
    pub user_notified_timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
///`MonitorAtProviderOwner`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorAtProviderOwner {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: SentryV1alpha1MonitorAtProviderOwnerType,
}
///`MonitorAtProviderOwnerType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1MonitorAtProviderOwnerType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "team")]
    Team,
}
impl ::std::fmt::Display for SentryV1alpha1MonitorAtProviderOwnerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Team => f.write_str("team"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1MonitorAtProviderOwnerType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "team" => Ok(Self::Team),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1MonitorAtProviderOwnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1MonitorAtProviderOwnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MonitorAtProviderProject`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorAtProviderProject {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: SentryV1alpha1MonitorAtProviderProjectAvatar,
    pub color: ::std::string::String,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub name: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    pub status: ::std::string::String,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
}
///`MonitorAtProviderProjectAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorAtProviderProjectAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`MonitorForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorForProvider {
    ///Alert rule configuration created alongside the monitor.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub alert_rule: ::std::option::Option<SentryV1alpha1MonitorForProviderAlertRule>,
    ///The configuration for the monitor.
    pub config: SentryV1alpha1MonitorForProviderConfig,
    ///Disable creation of monitor incidents
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_muted: ::std::option::Option<bool>,
    ///Name of the monitor. Used for notifications. If not set the slug will be derived from your monitor name.
    pub name: SentryV1alpha1MonitorForProviderName,
    ///The ID of the team or user that owns the monitor. (eg. user:51 or team:6)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<::std::string::String>,
    ///The project ID or slug to associate the monitor to.
    pub project: ::std::string::String,
    ///Uniquely identifies your monitor within your organization. Changing this slug will require updates to any instrumented check-in calls.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<SentryV1alpha1MonitorForProviderSlug>,
    /**Status of the monitor. Disabled monitors will not accept events and will not count towards the monitor quota.

* `active`
* `disabled`*/
    #[serde(default = "defaults::sentry_v1alpha1_monitor_for_provider_status")]
    pub status: SentryV1alpha1MonitorForProviderStatus,
}
///`MonitorForProviderAlertRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorForProviderAlertRule {
    ///Name of the environment
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub environment: ::std::option::Option<
        SentryV1alpha1MonitorForProviderAlertRuleEnvironment,
    >,
    ///Array of dictionaries with information of the user or team to be notified
    pub targets: ::std::vec::Vec<SentryV1alpha1MonitorForProviderAlertRuleTargetsItem>,
}
///Name of the environment
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1MonitorForProviderAlertRuleEnvironment(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1MonitorForProviderAlertRuleEnvironment {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1MonitorForProviderAlertRuleEnvironment>
for ::std::string::String {
    fn from(value: SentryV1alpha1MonitorForProviderAlertRuleEnvironment) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1MonitorForProviderAlertRuleEnvironment {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
for SentryV1alpha1MonitorForProviderAlertRuleEnvironment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1MonitorForProviderAlertRuleEnvironment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
for SentryV1alpha1MonitorForProviderAlertRuleEnvironment {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`MonitorForProviderAlertRuleTargetsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorForProviderAlertRuleTargetsItem {
    ///ID of target object
    pub target_identifier: i64,
    ///One of [Member, Team]
    pub target_type: ::std::string::String,
}
///`MonitorForProviderConfig`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1MonitorForProviderConfig {
    ///How long (in minutes) after the expected checkin time will we wait until we consider the checkin to have been missed.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub checkin_margin: ::std::option::Option<::std::num::NonZeroU64>,
    ///How many consecutive missed or failed check-ins in a row before creating a new issue.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub failure_issue_threshold: ::std::option::Option<::std::num::NonZeroU64>,
    ///How long (in minutes) is the checkin allowed to run for in CheckInStatus.IN_PROGRESS before it is considered failed.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_runtime: ::std::option::Option<::std::num::NonZeroU64>,
    ///How many successful check-ins in a row before resolving an issue.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub recovery_threshold: ::std::option::Option<::std::num::NonZeroU64>,
    ///Varies depending on the schedule_type. Is either a crontab string, or a 2 element tuple for intervals (e.g. [1, 'day'])
    pub schedule: ::serde_json::Value,
    /**Currently supports "crontab" or "interval"

* `crontab`
* `interval`*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub schedule_type: ::std::option::Option<
        SentryV1alpha1MonitorForProviderConfigScheduleType,
    >,
    /**tz database style timezone string

* `Africa/Abidjan`
* `Africa/Accra`
* `Africa/Addis_Ababa`
* `Africa/Algiers`
* `Africa/Asmara`
* `Africa/Asmera`
* `Africa/Bamako`
* `Africa/Bangui`
* `Africa/Banjul`
* `Africa/Bissau`
* `Africa/Blantyre`
* `Africa/Brazzaville`
* `Africa/Bujumbura`
* `Africa/Cairo`
* `Africa/Casablanca`
* `Africa/Ceuta`
* `Africa/Conakry`
* `Africa/Dakar`
* `Africa/Dar_es_Salaam`
* `Africa/Djibouti`
* `Africa/Douala`
* `Africa/El_Aaiun`
* `Africa/Freetown`
* `Africa/Gaborone`
* `Africa/Harare`
* `Africa/Johannesburg`
* `Africa/Juba`
* `Africa/Kampala`
* `Africa/Khartoum`
* `Africa/Kigali`
* `Africa/Kinshasa`
* `Africa/Lagos`
* `Africa/Libreville`
* `Africa/Lome`
* `Africa/Luanda`
* `Africa/Lubumbashi`
* `Africa/Lusaka`
* `Africa/Malabo`
* `Africa/Maputo`
* `Africa/Maseru`
* `Africa/Mbabane`
* `Africa/Mogadishu`
* `Africa/Monrovia`
* `Africa/Nairobi`
* `Africa/Ndjamena`
* `Africa/Niamey`
* `Africa/Nouakchott`
* `Africa/Ouagadougou`
* `Africa/Porto-Novo`
* `Africa/Sao_Tome`
* `Africa/Timbuktu`
* `Africa/Tripoli`
* `Africa/Tunis`
* `Africa/Windhoek`
* `America/Adak`
* `America/Anchorage`
* `America/Anguilla`
* `America/Antigua`
* `America/Araguaina`
* `America/Argentina/Buenos_Aires`
* `America/Argentina/Catamarca`
* `America/Argentina/ComodRivadavia`
* `America/Argentina/Cordoba`
* `America/Argentina/Jujuy`
* `America/Argentina/La_Rioja`
* `America/Argentina/Mendoza`
* `America/Argentina/Rio_Gallegos`
* `America/Argentina/Salta`
* `America/Argentina/San_Juan`
* `America/Argentina/San_Luis`
* `America/Argentina/Tucuman`
* `America/Argentina/Ushuaia`
* `America/Aruba`
* `America/Asuncion`
* `America/Atikokan`
* `America/Atka`
* `America/Bahia`
* `America/Bahia_Banderas`
* `America/Barbados`
* `America/Belem`
* `America/Belize`
* `America/Blanc-Sablon`
* `America/Boa_Vista`
* `America/Bogota`
* `America/Boise`
* `America/Buenos_Aires`
* `America/Cambridge_Bay`
* `America/Campo_Grande`
* `America/Cancun`
* `America/Caracas`
* `America/Catamarca`
* `America/Cayenne`
* `America/Cayman`
* `America/Chicago`
* `America/Chihuahua`
* `America/Ciudad_Juarez`
* `America/Coral_Harbour`
* `America/Cordoba`
* `America/Costa_Rica`
* `America/Coyhaique`
* `America/Creston`
* `America/Cuiaba`
* `America/Curacao`
* `America/Danmarkshavn`
* `America/Dawson`
* `America/Dawson_Creek`
* `America/Denver`
* `America/Detroit`
* `America/Dominica`
* `America/Edmonton`
* `America/Eirunepe`
* `America/El_Salvador`
* `America/Ensenada`
* `America/Fort_Nelson`
* `America/Fort_Wayne`
* `America/Fortaleza`
* `America/Glace_Bay`
* `America/Godthab`
* `America/Goose_Bay`
* `America/Grand_Turk`
* `America/Grenada`
* `America/Guadeloupe`
* `America/Guatemala`
* `America/Guayaquil`
* `America/Guyana`
* `America/Halifax`
* `America/Havana`
* `America/Hermosillo`
* `America/Indiana/Indianapolis`
* `America/Indiana/Knox`
* `America/Indiana/Marengo`
* `America/Indiana/Petersburg`
* `America/Indiana/Tell_City`
* `America/Indiana/Vevay`
* `America/Indiana/Vincennes`
* `America/Indiana/Winamac`
* `America/Indianapolis`
* `America/Inuvik`
* `America/Iqaluit`
* `America/Jamaica`
* `America/Jujuy`
* `America/Juneau`
* `America/Kentucky/Louisville`
* `America/Kentucky/Monticello`
* `America/Knox_IN`
* `America/Kralendijk`
* `America/La_Paz`
* `America/Lima`
* `America/Los_Angeles`
* `America/Louisville`
* `America/Lower_Princes`
* `America/Maceio`
* `America/Managua`
* `America/Manaus`
* `America/Marigot`
* `America/Martinique`
* `America/Matamoros`
* `America/Mazatlan`
* `America/Mendoza`
* `America/Menominee`
* `America/Merida`
* `America/Metlakatla`
* `America/Mexico_City`
* `America/Miquelon`
* `America/Moncton`
* `America/Monterrey`
* `America/Montevideo`
* `America/Montreal`
* `America/Montserrat`
* `America/Nassau`
* `America/New_York`
* `America/Nipigon`
* `America/Nome`
* `America/Noronha`
* `America/North_Dakota/Beulah`
* `America/North_Dakota/Center`
* `America/North_Dakota/New_Salem`
* `America/Nuuk`
* `America/Ojinaga`
* `America/Panama`
* `America/Pangnirtung`
* `America/Paramaribo`
* `America/Phoenix`
* `America/Port-au-Prince`
* `America/Port_of_Spain`
* `America/Porto_Acre`
* `America/Porto_Velho`
* `America/Puerto_Rico`
* `America/Punta_Arenas`
* `America/Rainy_River`
* `America/Rankin_Inlet`
* `America/Recife`
* `America/Regina`
* `America/Resolute`
* `America/Rio_Branco`
* `America/Rosario`
* `America/Santa_Isabel`
* `America/Santarem`
* `America/Santiago`
* `America/Santo_Domingo`
* `America/Sao_Paulo`
* `America/Scoresbysund`
* `America/Shiprock`
* `America/Sitka`
* `America/St_Barthelemy`
* `America/St_Johns`
* `America/St_Kitts`
* `America/St_Lucia`
* `America/St_Thomas`
* `America/St_Vincent`
* `America/Swift_Current`
* `America/Tegucigalpa`
* `America/Thule`
* `America/Thunder_Bay`
* `America/Tijuana`
* `America/Toronto`
* `America/Tortola`
* `America/Vancouver`
* `America/Virgin`
* `America/Whitehorse`
* `America/Winnipeg`
* `America/Yakutat`
* `America/Yellowknife`
* `Antarctica/Casey`
* `Antarctica/Davis`
* `Antarctica/DumontDUrville`
* `Antarctica/Macquarie`
* `Antarctica/Mawson`
* `Antarctica/McMurdo`
* `Antarctica/Palmer`
* `Antarctica/Rothera`
* `Antarctica/South_Pole`
* `Antarctica/Syowa`
* `Antarctica/Troll`
* `Antarctica/Vostok`
* `Arctic/Longyearbyen`
* `Asia/Aden`
* `Asia/Almaty`
* `Asia/Amman`
* `Asia/Anadyr`
* `Asia/Aqtau`
* `Asia/Aqtobe`
* `Asia/Ashgabat`
* `Asia/Ashkhabad`
* `Asia/Atyrau`
* `Asia/Baghdad`
* `Asia/Bahrain`
* `Asia/Baku`
* `Asia/Bangkok`
* `Asia/Barnaul`
* `Asia/Beirut`
* `Asia/Bishkek`
* `Asia/Brunei`
* `Asia/Calcutta`
* `Asia/Chita`
* `Asia/Choibalsan`
* `Asia/Chongqing`
* `Asia/Chungking`
* `Asia/Colombo`
* `Asia/Dacca`
* `Asia/Damascus`
* `Asia/Dhaka`
* `Asia/Dili`
* `Asia/Dubai`
* `Asia/Dushanbe`
* `Asia/Famagusta`
* `Asia/Gaza`
* `Asia/Harbin`
* `Asia/Hebron`
* `Asia/Ho_Chi_Minh`
* `Asia/Hong_Kong`
* `Asia/Hovd`
* `Asia/Irkutsk`
* `Asia/Istanbul`
* `Asia/Jakarta`
* `Asia/Jayapura`
* `Asia/Jerusalem`
* `Asia/Kabul`
* `Asia/Kamchatka`
* `Asia/Karachi`
* `Asia/Kashgar`
* `Asia/Kathmandu`
* `Asia/Katmandu`
* `Asia/Khandyga`
* `Asia/Kolkata`
* `Asia/Krasnoyarsk`
* `Asia/Kuala_Lumpur`
* `Asia/Kuching`
* `Asia/Kuwait`
* `Asia/Macao`
* `Asia/Macau`
* `Asia/Magadan`
* `Asia/Makassar`
* `Asia/Manila`
* `Asia/Muscat`
* `Asia/Nicosia`
* `Asia/Novokuznetsk`
* `Asia/Novosibirsk`
* `Asia/Omsk`
* `Asia/Oral`
* `Asia/Phnom_Penh`
* `Asia/Pontianak`
* `Asia/Pyongyang`
* `Asia/Qatar`
* `Asia/Qostanay`
* `Asia/Qyzylorda`
* `Asia/Rangoon`
* `Asia/Riyadh`
* `Asia/Saigon`
* `Asia/Sakhalin`
* `Asia/Samarkand`
* `Asia/Seoul`
* `Asia/Shanghai`
* `Asia/Singapore`
* `Asia/Srednekolymsk`
* `Asia/Taipei`
* `Asia/Tashkent`
* `Asia/Tbilisi`
* `Asia/Tehran`
* `Asia/Tel_Aviv`
* `Asia/Thimbu`
* `Asia/Thimphu`
* `Asia/Tokyo`
* `Asia/Tomsk`
* `Asia/Ujung_Pandang`
* `Asia/Ulaanbaatar`
* `Asia/Ulan_Bator`
* `Asia/Urumqi`
* `Asia/Ust-Nera`
* `Asia/Vientiane`
* `Asia/Vladivostok`
* `Asia/Yakutsk`
* `Asia/Yangon`
* `Asia/Yekaterinburg`
* `Asia/Yerevan`
* `Atlantic/Azores`
* `Atlantic/Bermuda`
* `Atlantic/Canary`
* `Atlantic/Cape_Verde`
* `Atlantic/Faeroe`
* `Atlantic/Faroe`
* `Atlantic/Jan_Mayen`
* `Atlantic/Madeira`
* `Atlantic/Reykjavik`
* `Atlantic/South_Georgia`
* `Atlantic/St_Helena`
* `Atlantic/Stanley`
* `Australia/ACT`
* `Australia/Adelaide`
* `Australia/Brisbane`
* `Australia/Broken_Hill`
* `Australia/Canberra`
* `Australia/Currie`
* `Australia/Darwin`
* `Australia/Eucla`
* `Australia/Hobart`
* `Australia/LHI`
* `Australia/Lindeman`
* `Australia/Lord_Howe`
* `Australia/Melbourne`
* `Australia/NSW`
* `Australia/North`
* `Australia/Perth`
* `Australia/Queensland`
* `Australia/South`
* `Australia/Sydney`
* `Australia/Tasmania`
* `Australia/Victoria`
* `Australia/West`
* `Australia/Yancowinna`
* `Brazil/Acre`
* `Brazil/DeNoronha`
* `Brazil/East`
* `Brazil/West`
* `CET`
* `CST6CDT`
* `Canada/Atlantic`
* `Canada/Central`
* `Canada/Eastern`
* `Canada/Mountain`
* `Canada/Newfoundland`
* `Canada/Pacific`
* `Canada/Saskatchewan`
* `Canada/Yukon`
* `Chile/Continental`
* `Chile/EasterIsland`
* `Cuba`
* `EET`
* `EST`
* `EST5EDT`
* `Egypt`
* `Eire`
* `Etc/GMT`
* `Etc/GMT+0`
* `Etc/GMT+1`
* `Etc/GMT+10`
* `Etc/GMT+11`
* `Etc/GMT+12`
* `Etc/GMT+2`
* `Etc/GMT+3`
* `Etc/GMT+4`
* `Etc/GMT+5`
* `Etc/GMT+6`
* `Etc/GMT+7`
* `Etc/GMT+8`
* `Etc/GMT+9`
* `Etc/GMT-0`
* `Etc/GMT-1`
* `Etc/GMT-10`
* `Etc/GMT-11`
* `Etc/GMT-12`
* `Etc/GMT-13`
* `Etc/GMT-14`
* `Etc/GMT-2`
* `Etc/GMT-3`
* `Etc/GMT-4`
* `Etc/GMT-5`
* `Etc/GMT-6`
* `Etc/GMT-7`
* `Etc/GMT-8`
* `Etc/GMT-9`
* `Etc/GMT0`
* `Etc/Greenwich`
* `Etc/UCT`
* `Etc/UTC`
* `Etc/Universal`
* `Etc/Zulu`
* `Europe/Amsterdam`
* `Europe/Andorra`
* `Europe/Astrakhan`
* `Europe/Athens`
* `Europe/Belfast`
* `Europe/Belgrade`
* `Europe/Berlin`
* `Europe/Bratislava`
* `Europe/Brussels`
* `Europe/Bucharest`
* `Europe/Budapest`
* `Europe/Busingen`
* `Europe/Chisinau`
* `Europe/Copenhagen`
* `Europe/Dublin`
* `Europe/Gibraltar`
* `Europe/Guernsey`
* `Europe/Helsinki`
* `Europe/Isle_of_Man`
* `Europe/Istanbul`
* `Europe/Jersey`
* `Europe/Kaliningrad`
* `Europe/Kiev`
* `Europe/Kirov`
* `Europe/Kyiv`
* `Europe/Lisbon`
* `Europe/Ljubljana`
* `Europe/London`
* `Europe/Luxembourg`
* `Europe/Madrid`
* `Europe/Malta`
* `Europe/Mariehamn`
* `Europe/Minsk`
* `Europe/Monaco`
* `Europe/Moscow`
* `Europe/Nicosia`
* `Europe/Oslo`
* `Europe/Paris`
* `Europe/Podgorica`
* `Europe/Prague`
* `Europe/Riga`
* `Europe/Rome`
* `Europe/Samara`
* `Europe/San_Marino`
* `Europe/Sarajevo`
* `Europe/Saratov`
* `Europe/Simferopol`
* `Europe/Skopje`
* `Europe/Sofia`
* `Europe/Stockholm`
* `Europe/Tallinn`
* `Europe/Tirane`
* `Europe/Tiraspol`
* `Europe/Ulyanovsk`
* `Europe/Uzhgorod`
* `Europe/Vaduz`
* `Europe/Vatican`
* `Europe/Vienna`
* `Europe/Vilnius`
* `Europe/Volgograd`
* `Europe/Warsaw`
* `Europe/Zagreb`
* `Europe/Zaporozhye`
* `Europe/Zurich`
* `GB`
* `GB-Eire`
* `GMT`
* `GMT+0`
* `GMT-0`
* `GMT0`
* `Greenwich`
* `HST`
* `Hongkong`
* `Iceland`
* `Indian/Antananarivo`
* `Indian/Chagos`
* `Indian/Christmas`
* `Indian/Cocos`
* `Indian/Comoro`
* `Indian/Kerguelen`
* `Indian/Mahe`
* `Indian/Maldives`
* `Indian/Mauritius`
* `Indian/Mayotte`
* `Indian/Reunion`
* `Iran`
* `Israel`
* `Jamaica`
* `Japan`
* `Kwajalein`
* `Libya`
* `MET`
* `MST`
* `MST7MDT`
* `Mexico/BajaNorte`
* `Mexico/BajaSur`
* `Mexico/General`
* `NZ`
* `NZ-CHAT`
* `Navajo`
* `PRC`
* `PST8PDT`
* `Pacific/Apia`
* `Pacific/Auckland`
* `Pacific/Bougainville`
* `Pacific/Chatham`
* `Pacific/Chuuk`
* `Pacific/Easter`
* `Pacific/Efate`
* `Pacific/Enderbury`
* `Pacific/Fakaofo`
* `Pacific/Fiji`
* `Pacific/Funafuti`
* `Pacific/Galapagos`
* `Pacific/Gambier`
* `Pacific/Guadalcanal`
* `Pacific/Guam`
* `Pacific/Honolulu`
* `Pacific/Johnston`
* `Pacific/Kanton`
* `Pacific/Kiritimati`
* `Pacific/Kosrae`
* `Pacific/Kwajalein`
* `Pacific/Majuro`
* `Pacific/Marquesas`
* `Pacific/Midway`
* `Pacific/Nauru`
* `Pacific/Niue`
* `Pacific/Norfolk`
* `Pacific/Noumea`
* `Pacific/Pago_Pago`
* `Pacific/Palau`
* `Pacific/Pitcairn`
* `Pacific/Pohnpei`
* `Pacific/Ponape`
* `Pacific/Port_Moresby`
* `Pacific/Rarotonga`
* `Pacific/Saipan`
* `Pacific/Samoa`
* `Pacific/Tahiti`
* `Pacific/Tarawa`
* `Pacific/Tongatapu`
* `Pacific/Truk`
* `Pacific/Wake`
* `Pacific/Wallis`
* `Pacific/Yap`
* `Poland`
* `Portugal`
* `ROC`
* `ROK`
* `Singapore`
* `Turkey`
* `UCT`
* `US/Alaska`
* `US/Aleutian`
* `US/Arizona`
* `US/Central`
* `US/East-Indiana`
* `US/Eastern`
* `US/Hawaii`
* `US/Indiana-Starke`
* `US/Michigan`
* `US/Mountain`
* `US/Pacific`
* `US/Samoa`
* `UTC`
* `Universal`
* `W-SU`
* `WET`
* `Zulu`
* `localtime`*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timezone: ::std::option::Option<::std::string::String>,
}
/**Currently supports "crontab" or "interval"

* `crontab`
* `interval`*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1MonitorForProviderConfigScheduleType {
    #[serde(rename = "crontab")]
    Crontab,
    #[serde(rename = "interval")]
    Interval,
}
impl ::std::fmt::Display for SentryV1alpha1MonitorForProviderConfigScheduleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Crontab => f.write_str("crontab"),
            Self::Interval => f.write_str("interval"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1MonitorForProviderConfigScheduleType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "crontab" => Ok(Self::Crontab),
            "interval" => Ok(Self::Interval),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for SentryV1alpha1MonitorForProviderConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1MonitorForProviderConfigScheduleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Name of the monitor. Used for notifications. If not set the slug will be derived from your monitor name.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1MonitorForProviderName(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1MonitorForProviderName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1MonitorForProviderName>
for ::std::string::String {
    fn from(value: SentryV1alpha1MonitorForProviderName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1MonitorForProviderName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1MonitorForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1MonitorForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SentryV1alpha1MonitorForProviderName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Uniquely identifies your monitor within your organization. Changing this slug will require updates to any instrumented check-in calls.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1MonitorForProviderSlug(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1MonitorForProviderSlug {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1MonitorForProviderSlug>
for ::std::string::String {
    fn from(value: SentryV1alpha1MonitorForProviderSlug) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1MonitorForProviderSlug {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 50usize {
            return Err("longer than 50 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^(?![0-9]+$)[a-z0-9_\\-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(?![0-9]+$)[a-z0-9_\\-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1MonitorForProviderSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1MonitorForProviderSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SentryV1alpha1MonitorForProviderSlug {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Status of the monitor. Disabled monitors will not accept events and will not count towards the monitor quota.

* `active`
* `disabled`*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1MonitorForProviderStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for SentryV1alpha1MonitorForProviderStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1MonitorForProviderStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1MonitorForProviderStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1MonitorForProviderStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for SentryV1alpha1MonitorForProviderStatus {
    fn default() -> Self {
        SentryV1alpha1MonitorForProviderStatus::Active
    }
}
///`OrganizationAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationAtProvider {
    #[serde(
        rename = "dateCreated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isEarlyAdopter",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_early_adopter: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "require2FA",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub require2_fa: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<::std::string::String>,
}
///`OrganizationForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationForProvider {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
///`OrganizationMemberAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProvider {
    #[serde(
        rename = "dateCreated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expired: ::std::option::Option<bool>,
    #[serde(
        rename = "externalUsers",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_users: ::std::vec::Vec<
        SentryV1alpha1OrganizationMemberAtProviderExternalUsersItem,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<SentryV1alpha1OrganizationMemberAtProviderFlags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub invite_link: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "inviteStatus",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub invite_status: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "inviterName",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub inviter_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isOnlyOwner",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_only_owner: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orgRole", skip_serializing_if = "::std::option::Option::is_none")]
    pub org_role: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "orgRoleList",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub org_role_list: ::std::vec::Vec<
        SentryV1alpha1OrganizationMemberAtProviderOrgRoleListItem,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pending: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "roleName", skip_serializing_if = "::std::option::Option::is_none")]
    pub role_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "teamRoleList",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub team_role_list: ::std::vec::Vec<
        SentryV1alpha1OrganizationMemberAtProviderTeamRoleListItem,
    >,
    #[serde(
        rename = "teamRoles",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub team_roles: ::std::vec::Vec<
        SentryV1alpha1OrganizationMemberAtProviderTeamRolesItem,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub teams: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<SentryV1alpha1OrganizationMemberAtProviderUser>,
}
///`OrganizationMemberAtProviderExternalUsersItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProviderExternalUsersItem {
    #[serde(
        rename = "externalId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalName")]
    pub external_name: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "integrationId")]
    pub integration_id: ::std::string::String,
    pub provider: ::std::string::String,
    #[serde(rename = "teamId", skip_serializing_if = "::std::option::Option::is_none")]
    pub team_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId", skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
///`OrganizationMemberAtProviderFlags`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProviderFlags {
    #[serde(rename = "idp:provisioned")]
    pub idp_provisioned: bool,
    #[serde(rename = "idp:role-restricted")]
    pub idp_role_restricted: bool,
    #[serde(rename = "member-limit:restricted")]
    pub member_limit_restricted: bool,
    #[serde(rename = "partnership:restricted")]
    pub partnership_restricted: bool,
    #[serde(rename = "sso:invalid")]
    pub sso_invalid: bool,
    #[serde(rename = "sso:linked")]
    pub sso_linked: bool,
}
///`OrganizationMemberAtProviderOrgRoleListItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProviderOrgRoleListItem {
    pub allowed: bool,
    pub desc: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "isAllowed")]
    pub is_allowed: bool,
    #[serde(rename = "isGlobal")]
    pub is_global: bool,
    #[serde(rename = "isRetired")]
    pub is_retired: bool,
    #[serde(rename = "isTeamRolesAllowed")]
    pub is_team_roles_allowed: bool,
    #[serde(rename = "minimumTeamRole")]
    pub minimum_team_role: ::std::string::String,
    pub name: ::std::string::String,
    pub scopes: ::std::vec::Vec<::std::string::String>,
}
///`OrganizationMemberAtProviderTeamRoleListItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProviderTeamRoleListItem {
    pub allowed: bool,
    pub desc: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "isAllowed")]
    pub is_allowed: bool,
    #[serde(
        rename = "isMinimumRoleFor",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub is_minimum_role_for: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isRetired")]
    pub is_retired: bool,
    #[serde(rename = "isTeamRolesAllowed")]
    pub is_team_roles_allowed: bool,
    pub name: ::std::string::String,
    pub scopes: ::std::vec::Vec<::std::string::String>,
}
///`OrganizationMemberAtProviderTeamRolesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProviderTeamRolesItem {
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamSlug")]
    pub team_slug: ::std::string::String,
}
///`OrganizationMemberAtProviderUser`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProviderUser {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub authenticators: ::std::vec::Vec<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<
        SentryV1alpha1OrganizationMemberAtProviderUserAvatar,
    >,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: ::std::string::String,
    #[serde(
        rename = "canReset2fa",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub can_reset2fa: ::std::option::Option<bool>,
    #[serde(rename = "dateJoined")]
    pub date_joined: ::chrono::DateTime<::chrono::offset::Utc>,
    pub email: ::std::string::String,
    pub emails: ::std::vec::Vec<
        SentryV1alpha1OrganizationMemberAtProviderUserEmailsItem,
    >,
    pub experiments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub has2fa: bool,
    #[serde(rename = "hasPasswordAuth")]
    pub has_password_auth: bool,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub identities: ::std::vec::Vec<
        SentryV1alpha1OrganizationMemberAtProviderUserIdentitiesItem,
    >,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    #[serde(rename = "isStaff")]
    pub is_staff: bool,
    #[serde(rename = "isSuperuser")]
    pub is_superuser: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(
        rename = "lastActive",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_active: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "lastLogin",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub last_login: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub name: ::std::string::String,
    pub username: ::std::string::String,
}
///`OrganizationMemberAtProviderUserAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProviderUserAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`OrganizationMemberAtProviderUserEmailsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProviderUserEmailsItem {
    pub email: ::std::string::String,
    pub id: ::std::string::String,
    pub is_verified: bool,
}
///`OrganizationMemberAtProviderUserIdentitiesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProviderUserIdentitiesItem {
    #[serde(rename = "dateSynced")]
    pub date_synced: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(rename = "dateVerified")]
    pub date_verified: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub organization: SentryV1alpha1OrganizationMemberAtProviderUserIdentitiesItemOrganization,
    pub provider: SentryV1alpha1OrganizationMemberAtProviderUserIdentitiesItemProvider,
}
///`OrganizationMemberAtProviderUserIdentitiesItemOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProviderUserIdentitiesItemOrganization {
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`OrganizationMemberAtProviderUserIdentitiesItemProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberAtProviderUserIdentitiesItemProvider {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`OrganizationMemberForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1OrganizationMemberForProvider {
    ///The email address to send the invitation to.
    pub email: SentryV1alpha1OrganizationMemberForProviderEmail,
    /**The organization-level role of the new member. Roles include:

* `billing` - Can manage payment and compliance details.
* `member` - Can view and act on events, as well as view most other data within the organization.
* `manager` - Has full management access to all teams and projects. Can also manage
        the organization's membership.
* `owner` - Has unrestricted access to the organization, its data, and its
        settings. Can add, modify, and delete projects and members, as well as
        make billing and plan changes.
* `admin` - Can edit global integrations, manage projects, and add/remove teams.
        They automatically assume the Team Admin role for teams they join.
        Note: This role can no longer be assigned in Business and Enterprise plans. Use `TeamRoles` instead.
        */
    #[serde(
        rename = "orgRole",
        default = "defaults::sentry_v1alpha1_organization_member_for_provider_org_role"
    )]
    pub org_role: SentryV1alpha1OrganizationMemberForProviderOrgRole,
    ///Reissue the invite token, invalidating any previously sent invite link.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub regenerate: ::std::option::Option<bool>,
    ///Whether or not to re-invite a user who has already been invited to the organization. Defaults to True.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reinvite: ::std::option::Option<bool>,
    /**Deprecated, use `orgRole`. The organization-level role to assign.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
    #[serde(default = "defaults::sentry_v1alpha1_organization_member_for_provider_role")]
    pub role: SentryV1alpha1OrganizationMemberForProviderRole,
    ///Whether or not to send an invite notification through email. Defaults to True.
    #[serde(rename = "sendInvite", default = "defaults::default_bool::<true>")]
    pub send_invite: bool,
    /**The team and team-roles assigned to the member. Team roles can be either:
        - `contributor` - Can view and act on issues. Depending on organization settings, they can also add team members.
        - `admin` - Has full management access to their team's membership and projects.*/
    #[serde(
        rename = "teamRoles",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub team_roles: ::std::option::Option<
        ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    >,
    ///Deprecated, use `teamRoles`. Slugs of teams to add the member to.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub teams: ::std::vec::Vec<::serde_json::Value>,
}
///The email address to send the invitation to.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1OrganizationMemberForProviderEmail(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1OrganizationMemberForProviderEmail {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1OrganizationMemberForProviderEmail>
for ::std::string::String {
    fn from(value: SentryV1alpha1OrganizationMemberForProviderEmail) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1OrganizationMemberForProviderEmail {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 75usize {
            return Err("longer than 75 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1OrganizationMemberForProviderEmail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1OrganizationMemberForProviderEmail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
for SentryV1alpha1OrganizationMemberForProviderEmail {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**The organization-level role of the new member. Roles include:

* `billing` - Can manage payment and compliance details.
* `member` - Can view and act on events, as well as view most other data within the organization.
* `manager` - Has full management access to all teams and projects. Can also manage
        the organization's membership.
* `owner` - Has unrestricted access to the organization, its data, and its
        settings. Can add, modify, and delete projects and members, as well as
        make billing and plan changes.
* `admin` - Can edit global integrations, manage projects, and add/remove teams.
        They automatically assume the Team Admin role for teams they join.
        Note: This role can no longer be assigned in Business and Enterprise plans. Use `TeamRoles` instead.
        */
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1OrganizationMemberForProviderOrgRole {
    #[serde(rename = "billing")]
    Billing,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "admin")]
    Admin,
}
impl ::std::fmt::Display for SentryV1alpha1OrganizationMemberForProviderOrgRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Billing => f.write_str("billing"),
            Self::Member => f.write_str("member"),
            Self::Manager => f.write_str("manager"),
            Self::Owner => f.write_str("owner"),
            Self::Admin => f.write_str("admin"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1OrganizationMemberForProviderOrgRole {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "billing" => Ok(Self::Billing),
            "member" => Ok(Self::Member),
            "manager" => Ok(Self::Manager),
            "owner" => Ok(Self::Owner),
            "admin" => Ok(Self::Admin),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for SentryV1alpha1OrganizationMemberForProviderOrgRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1OrganizationMemberForProviderOrgRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for SentryV1alpha1OrganizationMemberForProviderOrgRole {
    fn default() -> Self {
        SentryV1alpha1OrganizationMemberForProviderOrgRole::Member
    }
}
/**Deprecated, use `orgRole`. The organization-level role to assign.

* `member` - Member
* `admin` - Admin
* `manager` - Manager
* `owner` - Owner*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1OrganizationMemberForProviderRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "owner")]
    Owner,
}
impl ::std::fmt::Display for SentryV1alpha1OrganizationMemberForProviderRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Member => f.write_str("member"),
            Self::Admin => f.write_str("admin"),
            Self::Manager => f.write_str("manager"),
            Self::Owner => f.write_str("owner"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1OrganizationMemberForProviderRole {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "member" => Ok(Self::Member),
            "admin" => Ok(Self::Admin),
            "manager" => Ok(Self::Manager),
            "owner" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1OrganizationMemberForProviderRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1OrganizationMemberForProviderRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for SentryV1alpha1OrganizationMemberForProviderRole {
    fn default() -> Self {
        SentryV1alpha1OrganizationMemberForProviderRole::Member
    }
}
///`ProjectAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectAtProvider {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "allowedDomains",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub allowed_domains: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "autofixAutomationTuning",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub autofix_automation_tuning: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<SentryV1alpha1ProjectAtProviderAvatar>,
    #[serde(
        rename = "builtinSymbolSources",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub builtin_symbol_sources: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "dataScrubber",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub data_scrubber: ::std::option::Option<bool>,
    #[serde(
        rename = "dataScrubberDefaults",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub data_scrubber_defaults: ::std::option::Option<bool>,
    #[serde(
        rename = "dateCreated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "debugFilesRole",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub debug_files_role: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "defaultEnvironment",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_environment: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "derivedGroupingEnhancements",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub derived_grouping_enhancements: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "digestsMaxDelay",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub digests_max_delay: ::std::option::Option<i64>,
    #[serde(
        rename = "digestsMinDelay",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub digests_min_delay: ::std::option::Option<i64>,
    #[serde(
        rename = "dynamicSamplingBiases",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub dynamic_sampling_biases: ::std::option::Option<
        ::std::vec::Vec<
            ::std::collections::HashMap<
                ::std::string::String,
                SentryV1alpha1ProjectAtProviderDynamicSamplingBiasesItemValue,
            >,
        >,
    >,
    #[serde(
        rename = "enableAutoReleaseCreation",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub enable_auto_release_creation: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "fingerprintingRules",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fingerprinting_rules: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "firstTransactionEvent",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_transaction_event: ::std::option::Option<bool>,
    #[serde(
        rename = "groupingConfig",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub grouping_config: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "groupingEnhancements",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub grouping_enhancements: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "hasAccess",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_access: ::std::option::Option<bool>,
    #[serde(
        rename = "hasFeedbacks",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_feedbacks: ::std::option::Option<bool>,
    #[serde(rename = "hasFlags", skip_serializing_if = "::std::option::Option::is_none")]
    pub has_flags: ::std::option::Option<bool>,
    #[serde(
        rename = "hasInsightsAgentMonitoring",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_insights_agent_monitoring: ::std::option::Option<bool>,
    #[serde(
        rename = "hasInsightsAppStart",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_insights_app_start: ::std::option::Option<bool>,
    #[serde(
        rename = "hasInsightsAssets",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_insights_assets: ::std::option::Option<bool>,
    #[serde(
        rename = "hasInsightsCaches",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_insights_caches: ::std::option::Option<bool>,
    #[serde(
        rename = "hasInsightsDb",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_insights_db: ::std::option::Option<bool>,
    #[serde(
        rename = "hasInsightsHttp",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_insights_http: ::std::option::Option<bool>,
    #[serde(
        rename = "hasInsightsMCP",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_insights_mcp: ::std::option::Option<bool>,
    #[serde(
        rename = "hasInsightsQueues",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_insights_queues: ::std::option::Option<bool>,
    #[serde(
        rename = "hasInsightsScreenLoad",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_insights_screen_load: ::std::option::Option<bool>,
    #[serde(
        rename = "hasInsightsVitals",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_insights_vitals: ::std::option::Option<bool>,
    #[serde(rename = "hasLogs", skip_serializing_if = "::std::option::Option::is_none")]
    pub has_logs: ::std::option::Option<bool>,
    #[serde(
        rename = "hasMinifiedStackTrace",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_minified_stack_trace: ::std::option::Option<bool>,
    #[serde(
        rename = "hasMonitors",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_monitors: ::std::option::Option<bool>,
    #[serde(
        rename = "hasNewFeedbacks",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_new_feedbacks: ::std::option::Option<bool>,
    #[serde(
        rename = "hasProfiles",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_profiles: ::std::option::Option<bool>,
    #[serde(
        rename = "hasReplays",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_replays: ::std::option::Option<bool>,
    #[serde(
        rename = "hasSessions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_sessions: ::std::option::Option<bool>,
    #[serde(
        rename = "hasTraceMetrics",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_trace_metrics: ::std::option::Option<bool>,
    #[serde(
        rename = "highlightContext",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub highlight_context: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "highlightPreset",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub highlight_preset: ::std::option::Option<
        SentryV1alpha1ProjectAtProviderHighlightPreset,
    >,
    #[serde(
        rename = "highlightTags",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub highlight_tags: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isBookmarked",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_bookmarked: ::std::option::Option<bool>,
    #[serde(
        rename = "isDynamicallySampled",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_dynamically_sampled: ::std::option::Option<bool>,
    #[serde(
        rename = "isInternal",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_internal: ::std::option::Option<bool>,
    #[serde(rename = "isMember", skip_serializing_if = "::std::option::Option::is_none")]
    pub is_member: ::std::option::Option<bool>,
    #[serde(rename = "isPublic", skip_serializing_if = "::std::option::Option::is_none")]
    pub is_public: ::std::option::Option<bool>,
    #[serde(
        rename = "latestRelease",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub latest_release: ::std::option::Option<
        SentryV1alpha1ProjectAtProviderLatestRelease,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub options: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub organization: ::std::option::Option<SentryV1alpha1ProjectAtProviderOrganization>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub platforms: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "processingIssues",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub processing_issues: ::std::option::Option<i64>,
    #[serde(
        rename = "relayPiiConfig",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub relay_pii_config: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "resolveAge",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub resolve_age: ::std::option::Option<i64>,
    #[serde(
        rename = "safeFields",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub safe_fields: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "scmSourceContextEnabled",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scm_source_context_enabled: ::std::option::Option<bool>,
    #[serde(
        rename = "scrapeJavaScript",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrape_java_script: ::std::option::Option<bool>,
    #[serde(
        rename = "scrubIPAddresses",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrub_ip_addresses: ::std::option::Option<bool>,
    #[serde(
        rename = "secondaryGroupingConfig",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub secondary_grouping_config: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "secondaryGroupingExpiry",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub secondary_grouping_expiry: ::std::option::Option<i64>,
    #[serde(
        rename = "securityToken",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub security_token: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "securityTokenHeader",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub security_token_header: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "seerNightshiftTweaks",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub seer_nightshift_tweaks: ::std::option::Option<::serde_json::Value>,
    #[serde(
        rename = "seerScannerAutomation",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub seer_scanner_automation: ::std::option::Option<bool>,
    #[serde(
        rename = "sensitiveFields",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub sensitive_fields: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "storeCrashReports",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub store_crash_reports: ::std::option::Option<i64>,
    #[serde(
        rename = "subjectPrefix",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subject_prefix: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "subjectTemplate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subject_template: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "symbolSources",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub symbol_sources: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub team: ::std::option::Option<SentryV1alpha1ProjectAtProviderTeam>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub teams: ::std::vec::Vec<SentryV1alpha1ProjectAtProviderTeamsItem>,
    #[serde(
        rename = "tempestFetchScreenshots",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tempest_fetch_screenshots: ::std::option::Option<bool>,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
    #[serde(
        rename = "verifySSL",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub verify_ssl: ::std::option::Option<bool>,
}
///`ProjectAtProviderAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectAtProviderAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`ProjectAtProviderDynamicSamplingBiasesItemValue`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum SentryV1alpha1ProjectAtProviderDynamicSamplingBiasesItemValue {
    String(::std::string::String),
    Boolean(bool),
}
impl ::std::fmt::Display
for SentryV1alpha1ProjectAtProviderDynamicSamplingBiasesItemValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Boolean(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool>
for SentryV1alpha1ProjectAtProviderDynamicSamplingBiasesItemValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
///`ProjectAtProviderHighlightPreset`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectAtProviderHighlightPreset {
    pub context: ::std::collections::HashMap<
        ::std::string::String,
        ::std::vec::Vec<::std::string::String>,
    >,
    pub tags: ::std::vec::Vec<::std::string::String>,
}
///`ProjectAtProviderLatestRelease`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectAtProviderLatestRelease {
    pub version: ::std::string::String,
}
///`ProjectAtProviderOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectAtProviderOrganization {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowMemberInvite")]
    pub allow_member_invite: bool,
    #[serde(rename = "allowMemberProjectCreation")]
    pub allow_member_project_creation: bool,
    #[serde(rename = "allowSuperuserAccess")]
    pub allow_superuser_access: bool,
    pub avatar: SentryV1alpha1ProjectAtProviderOrganizationAvatar,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "extraOptions",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extra_options: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "hasAuthProvider")]
    pub has_auth_provider: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isEarlyAdopter")]
    pub is_early_adopter: bool,
    pub links: SentryV1alpha1ProjectAtProviderOrganizationLinks,
    pub name: ::std::string::String,
    #[serde(
        rename = "onboardingTasks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub onboarding_tasks: ::std::vec::Vec<
        SentryV1alpha1ProjectAtProviderOrganizationOnboardingTasksItem,
    >,
    #[serde(rename = "require2FA")]
    pub require2_fa: bool,
    pub slug: ::std::string::String,
    pub status: SentryV1alpha1ProjectAtProviderOrganizationStatus,
}
///`ProjectAtProviderOrganizationAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectAtProviderOrganizationAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`ProjectAtProviderOrganizationLinks`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectAtProviderOrganizationLinks {
    #[serde(rename = "organizationUrl")]
    pub organization_url: ::std::string::String,
    #[serde(rename = "regionUrl")]
    pub region_url: ::std::string::String,
}
///`ProjectAtProviderOrganizationOnboardingTasksItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectAtProviderOrganizationOnboardingTasksItem {
    #[serde(
        rename = "completionSeen",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub completion_seen: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    pub data: ::serde_json::Value,
    #[serde(rename = "dateCompleted")]
    pub date_completed: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub task: ::std::option::Option<::std::string::String>,
}
///`ProjectAtProviderOrganizationStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectAtProviderOrganizationStatus {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`ProjectAtProviderTeam`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectAtProviderTeam {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`ProjectAtProviderTeamsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectAtProviderTeamsItem {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub slug: ::std::string::String,
}
///`ProjectForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectForProvider {
    /**
Defaults to true where the behavior is to alert the user on every new
issue. Setting this to false will turn this off and the user must create
their own alerts to be notified of new issues.
        */
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default_rules: ::std::option::Option<bool>,
    ///The name for the project.
    pub name: SentryV1alpha1ProjectForProviderName,
    ///The platform for the project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub platform: ::std::option::Option<::std::string::String>,
    /**Uniquely identifies a project and is used for the interface.
        If not provided, it is automatically generated from the name.*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<SentryV1alpha1ProjectForProviderSlug>,
}
///The name for the project.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1ProjectForProviderName(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1ProjectForProviderName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1ProjectForProviderName>
for ::std::string::String {
    fn from(value: SentryV1alpha1ProjectForProviderName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1ProjectForProviderName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 50usize {
            return Err("longer than 50 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1ProjectForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1ProjectForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SentryV1alpha1ProjectForProviderName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Uniquely identifies a project and is used for the interface.
        If not provided, it is automatically generated from the name.*/
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1ProjectForProviderSlug(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1ProjectForProviderSlug {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1ProjectForProviderSlug>
for ::std::string::String {
    fn from(value: SentryV1alpha1ProjectForProviderSlug) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1ProjectForProviderSlug {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^(?![0-9]+$)[a-z0-9_\\-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(?![0-9]+$)[a-z0-9_\\-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1ProjectForProviderSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1ProjectForProviderSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SentryV1alpha1ProjectForProviderSlug {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`ProjectKeyAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectKeyAtProvider {
    #[serde(
        rename = "browserSdk",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub browser_sdk: ::std::option::Option<SentryV1alpha1ProjectKeyAtProviderBrowserSdk>,
    #[serde(
        rename = "browserSdkVersion",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub browser_sdk_version: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "dateCreated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dsn: ::std::option::Option<SentryV1alpha1ProjectKeyAtProviderDsn>,
    #[serde(
        rename = "dynamicSdkLoaderOptions",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub dynamic_sdk_loader_options: ::std::option::Option<
        SentryV1alpha1ProjectKeyAtProviderDynamicSdkLoaderOptions,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isActive", skip_serializing_if = "::std::option::Option::is_none")]
    pub is_active: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "projectId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub project_id: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub public: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "rateLimit",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rate_limit: ::std::option::Option<SentryV1alpha1ProjectKeyAtProviderRateLimit>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub secret: ::std::option::Option<::std::string::String>,
    #[serde(rename = "useCase", skip_serializing_if = "::std::option::Option::is_none")]
    pub use_case: ::std::option::Option<::std::string::String>,
}
///`ProjectKeyAtProviderBrowserSdk`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectKeyAtProviderBrowserSdk {
    pub choices: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
}
///`ProjectKeyAtProviderDsn`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectKeyAtProviderDsn {
    pub cdn: ::std::string::String,
    pub crons: ::std::string::String,
    pub csp: ::std::string::String,
    pub integration: ::std::string::String,
    pub minidump: ::std::string::String,
    pub nel: ::std::string::String,
    pub otlp_logs: ::std::string::String,
    pub otlp_traces: ::std::string::String,
    pub playstation: ::std::string::String,
    pub public: ::std::string::String,
    pub secret: ::std::string::String,
    pub security: ::std::string::String,
    pub unreal: ::std::string::String,
}
///`ProjectKeyAtProviderDynamicSdkLoaderOptions`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectKeyAtProviderDynamicSdkLoaderOptions {
    #[serde(rename = "hasDebug")]
    pub has_debug: bool,
    #[serde(rename = "hasFeedback")]
    pub has_feedback: bool,
    #[serde(rename = "hasLogsAndMetrics")]
    pub has_logs_and_metrics: bool,
    #[serde(rename = "hasPerformance")]
    pub has_performance: bool,
    #[serde(rename = "hasReplay")]
    pub has_replay: bool,
}
///`ProjectKeyAtProviderRateLimit`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectKeyAtProviderRateLimit {
    pub count: i64,
    pub window: i64,
}
///`ProjectKeyForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectKeyForProvider {
    ///The optional name of the key. If not provided it will be automatically generated.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<SentryV1alpha1ProjectKeyForProviderName>,
    #[serde(
        rename = "rateLimit",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub rate_limit: ::std::option::Option<SentryV1alpha1ProjectKeyForProviderRateLimit>,
    #[doc = "* `user`\n* `profiling`\n* `tempest`\n* `demo`"]
    #[serde(
        rename = "useCase",
        default = "defaults::sentry_v1alpha1_project_key_for_provider_use_case"
    )]
    pub use_case: SentryV1alpha1ProjectKeyForProviderUseCase,
}
impl ::std::default::Default for SentryV1alpha1ProjectKeyForProvider {
    fn default() -> Self {
        Self {
            name: Default::default(),
            rate_limit: Default::default(),
            use_case: defaults::sentry_v1alpha1_project_key_for_provider_use_case(),
        }
    }
}
///The optional name of the key. If not provided it will be automatically generated.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1ProjectKeyForProviderName(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1ProjectKeyForProviderName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1ProjectKeyForProviderName>
for ::std::string::String {
    fn from(value: SentryV1alpha1ProjectKeyForProviderName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1ProjectKeyForProviderName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1ProjectKeyForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1ProjectKeyForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SentryV1alpha1ProjectKeyForProviderName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Applies a rate limit to cap the number of errors accepted during a given time window. To
disable entirely set `rateLimit` to null.
```json
{
    "rateLimit": {
        "window": 7200, // time in seconds
        "count": 1000 // error cap
    }
}
```*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ProjectKeyForProviderRateLimit {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub window: ::std::option::Option<i64>,
}
#[doc = "* `user`\n* `profiling`\n* `tempest`\n* `demo`"]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
pub enum SentryV1alpha1ProjectKeyForProviderUseCase {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "profiling")]
    Profiling,
    #[serde(rename = "tempest")]
    Tempest,
    #[serde(rename = "demo")]
    Demo,
}
impl ::std::fmt::Display for SentryV1alpha1ProjectKeyForProviderUseCase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Profiling => f.write_str("profiling"),
            Self::Tempest => f.write_str("tempest"),
            Self::Demo => f.write_str("demo"),
        }
    }
}
impl ::std::str::FromStr for SentryV1alpha1ProjectKeyForProviderUseCase {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "profiling" => Ok(Self::Profiling),
            "tempest" => Ok(Self::Tempest),
            "demo" => Ok(Self::Demo),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1ProjectKeyForProviderUseCase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1ProjectKeyForProviderUseCase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for SentryV1alpha1ProjectKeyForProviderUseCase {
    fn default() -> Self {
        SentryV1alpha1ProjectKeyForProviderUseCase::User
    }
}
///`ServiceHookAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ServiceHookAtProvider {
    #[serde(
        rename = "dateCreated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_created: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub events: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub secret: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
///`ServiceHookForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1ServiceHookForProvider {
    ///The events to subscribe to.
    pub events: ::std::vec::Vec<::std::string::String>,
    ///The URL for the webhook.
    pub url: ::std::string::String,
}
///`TeamAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1TeamAtProvider {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<SentryV1alpha1TeamAtProviderAvatar>,
    #[serde(
        rename = "dateCreated",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "externalTeams",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_teams: ::std::vec::Vec<SentryV1alpha1TeamAtProviderExternalTeamsItem>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "hasAccess",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_access: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isMember", skip_serializing_if = "::std::option::Option::is_none")]
    pub is_member: ::std::option::Option<bool>,
    #[serde(
        rename = "isPending",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_pending: ::std::option::Option<bool>,
    #[serde(
        rename = "memberCount",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub member_count: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub organization: ::std::option::Option<SentryV1alpha1TeamAtProviderOrganization>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub projects: ::std::vec::Vec<SentryV1alpha1TeamAtProviderProjectsItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<::std::string::String>,
    #[serde(rename = "teamRole", skip_serializing_if = "::std::option::Option::is_none")]
    pub team_role: ::std::option::Option<::std::string::String>,
}
///`TeamAtProviderAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1TeamAtProviderAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`TeamAtProviderExternalTeamsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1TeamAtProviderExternalTeamsItem {
    #[serde(
        rename = "externalId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalName")]
    pub external_name: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "integrationId")]
    pub integration_id: ::std::string::String,
    pub provider: ::std::string::String,
    #[serde(rename = "teamId", skip_serializing_if = "::std::option::Option::is_none")]
    pub team_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId", skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
///`TeamAtProviderOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1TeamAtProviderOrganization {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowMemberInvite")]
    pub allow_member_invite: bool,
    #[serde(rename = "allowMemberProjectCreation")]
    pub allow_member_project_creation: bool,
    #[serde(rename = "allowSuperuserAccess")]
    pub allow_superuser_access: bool,
    pub avatar: SentryV1alpha1TeamAtProviderOrganizationAvatar,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "extraOptions",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extra_options: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "hasAuthProvider")]
    pub has_auth_provider: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isEarlyAdopter")]
    pub is_early_adopter: bool,
    pub links: SentryV1alpha1TeamAtProviderOrganizationLinks,
    pub name: ::std::string::String,
    #[serde(
        rename = "onboardingTasks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub onboarding_tasks: ::std::vec::Vec<
        SentryV1alpha1TeamAtProviderOrganizationOnboardingTasksItem,
    >,
    #[serde(rename = "require2FA")]
    pub require2_fa: bool,
    pub slug: ::std::string::String,
    pub status: SentryV1alpha1TeamAtProviderOrganizationStatus,
}
///`TeamAtProviderOrganizationAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1TeamAtProviderOrganizationAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`TeamAtProviderOrganizationLinks`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1TeamAtProviderOrganizationLinks {
    #[serde(rename = "organizationUrl")]
    pub organization_url: ::std::string::String,
    #[serde(rename = "regionUrl")]
    pub region_url: ::std::string::String,
}
///`TeamAtProviderOrganizationOnboardingTasksItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1TeamAtProviderOrganizationOnboardingTasksItem {
    #[serde(
        rename = "completionSeen",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub completion_seen: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    pub data: ::serde_json::Value,
    #[serde(rename = "dateCompleted")]
    pub date_completed: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub task: ::std::option::Option<::std::string::String>,
}
///`TeamAtProviderOrganizationStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1TeamAtProviderOrganizationStatus {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`TeamAtProviderProjectsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1TeamAtProviderProjectsItem {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: SentryV1alpha1TeamAtProviderProjectsItemAvatar,
    pub color: ::std::string::String,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub name: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    pub status: ::std::string::String,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
}
///`TeamAtProviderProjectsItemAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1TeamAtProviderProjectsItemAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`TeamForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct SentryV1alpha1TeamForProvider {
    #[doc = "**`[DEPRECATED]`** The name for the team. If not provided, it is\n        automatically generated from the slug"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<SentryV1alpha1TeamForProviderName>,
    /**Uniquely identifies a team and is used for the interface. If not
        provided, it is automatically generated from the name.*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<SentryV1alpha1TeamForProviderSlug>,
}
#[doc = "**`[DEPRECATED]`** The name for the team. If not provided, it is\n        automatically generated from the slug"]
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1TeamForProviderName(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1TeamForProviderName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1TeamForProviderName> for ::std::string::String {
    fn from(value: SentryV1alpha1TeamForProviderName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1TeamForProviderName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1TeamForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1TeamForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SentryV1alpha1TeamForProviderName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Uniquely identifies a team and is used for the interface. If not
        provided, it is automatically generated from the name.*/
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct SentryV1alpha1TeamForProviderSlug(::std::string::String);
impl ::std::ops::Deref for SentryV1alpha1TeamForProviderSlug {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SentryV1alpha1TeamForProviderSlug> for ::std::string::String {
    fn from(value: SentryV1alpha1TeamForProviderSlug) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SentryV1alpha1TeamForProviderSlug {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 50usize {
            return Err("longer than 50 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^(?![0-9]+$)[a-z0-9_\\-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(?![0-9]+$)[a-z0-9_\\-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SentryV1alpha1TeamForProviderSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SentryV1alpha1TeamForProviderSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SentryV1alpha1TeamForProviderSlug {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`ServiceHookCreateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ServiceHookCreateRequest {
    ///The events to subscribe to.
    pub events: ::std::vec::Vec<::std::string::String>,
    ///The URL for the webhook.
    pub url: ::std::string::String,
}
///`ServiceHookCreateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ServiceHookCreateResponse {
    #[serde(rename = "dateCreated")]
    pub date_created: ::std::string::String,
    pub events: ::std::vec::Vec<::std::string::String>,
    pub id: ::std::string::String,
    pub secret: ::std::string::String,
    pub status: ::std::string::String,
    pub url: ::std::string::String,
}
///`ServiceHookObserveResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ServiceHookObserveResponse {
    #[serde(rename = "dateCreated")]
    pub date_created: ::std::string::String,
    pub events: ::std::vec::Vec<::std::string::String>,
    pub id: ::std::string::String,
    pub secret: ::std::string::String,
    pub status: ::std::string::String,
    pub url: ::std::string::String,
}
///`ServiceHookUpdateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ServiceHookUpdateRequest {
    ///The events to subscribe to.
    pub events: ::std::vec::Vec<::std::string::String>,
    ///The URL for the webhook.
    pub url: ::std::string::String,
}
///`ServiceHookUpdateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ServiceHookUpdateResponse {
    #[serde(rename = "dateCreated")]
    pub date_created: ::std::string::String,
    pub events: ::std::vec::Vec<::std::string::String>,
    pub id: ::std::string::String,
    pub secret: ::std::string::String,
    pub status: ::std::string::String,
    pub url: ::std::string::String,
}
///`TeamCreateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamCreateRequest {
    #[doc = "**`[DEPRECATED]`** The name for the team. If not provided, it is\n        automatically generated from the slug"]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<TeamCreateRequestName>,
    /**Uniquely identifies a team and is used for the interface. If not
        provided, it is automatically generated from the name.*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: ::std::option::Option<TeamCreateRequestSlug>,
}
#[doc = "**`[DEPRECATED]`** The name for the team. If not provided, it is\n        automatically generated from the slug"]
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct TeamCreateRequestName(::std::string::String);
impl ::std::ops::Deref for TeamCreateRequestName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TeamCreateRequestName> for ::std::string::String {
    fn from(value: TeamCreateRequestName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TeamCreateRequestName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TeamCreateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeamCreateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TeamCreateRequestName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
/**Uniquely identifies a team and is used for the interface. If not
        provided, it is automatically generated from the name.*/
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct TeamCreateRequestSlug(::std::string::String);
impl ::std::ops::Deref for TeamCreateRequestSlug {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TeamCreateRequestSlug> for ::std::string::String {
    fn from(value: TeamCreateRequestSlug) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TeamCreateRequestSlug {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 50usize {
            return Err("longer than 50 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^(?![0-9]+$)[a-z0-9_\\-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(?![0-9]+$)[a-z0-9_\\-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TeamCreateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeamCreateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TeamCreateRequestSlug {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`TeamCreateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamCreateResponse {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: TeamCreateResponseAvatar,
    #[serde(
        rename = "dateCreated",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "externalTeams",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_teams: ::std::vec::Vec<TeamCreateResponseExternalTeamsItem>,
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "memberCount")]
    pub member_count: i64,
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub organization: ::std::option::Option<TeamCreateResponseOrganization>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub projects: ::std::vec::Vec<TeamCreateResponseProjectsItem>,
    pub slug: ::std::string::String,
    #[serde(
        rename = "teamRole",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub team_role: ::std::option::Option<::std::string::String>,
}
///`TeamCreateResponseAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamCreateResponseAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`TeamCreateResponseExternalTeamsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamCreateResponseExternalTeamsItem {
    #[serde(
        rename = "externalId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalName")]
    pub external_name: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "integrationId")]
    pub integration_id: ::std::string::String,
    pub provider: ::std::string::String,
    #[serde(rename = "teamId", skip_serializing_if = "::std::option::Option::is_none")]
    pub team_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId", skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
///`TeamCreateResponseOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamCreateResponseOrganization {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowMemberInvite")]
    pub allow_member_invite: bool,
    #[serde(rename = "allowMemberProjectCreation")]
    pub allow_member_project_creation: bool,
    #[serde(rename = "allowSuperuserAccess")]
    pub allow_superuser_access: bool,
    pub avatar: TeamCreateResponseOrganizationAvatar,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "extraOptions",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extra_options: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "hasAuthProvider")]
    pub has_auth_provider: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isEarlyAdopter")]
    pub is_early_adopter: bool,
    pub links: TeamCreateResponseOrganizationLinks,
    pub name: ::std::string::String,
    #[serde(
        rename = "onboardingTasks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub onboarding_tasks: ::std::vec::Vec<
        TeamCreateResponseOrganizationOnboardingTasksItem,
    >,
    #[serde(rename = "require2FA")]
    pub require2_fa: bool,
    pub slug: ::std::string::String,
    pub status: TeamCreateResponseOrganizationStatus,
}
///`TeamCreateResponseOrganizationAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamCreateResponseOrganizationAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`TeamCreateResponseOrganizationLinks`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamCreateResponseOrganizationLinks {
    #[serde(rename = "organizationUrl")]
    pub organization_url: ::std::string::String,
    #[serde(rename = "regionUrl")]
    pub region_url: ::std::string::String,
}
///`TeamCreateResponseOrganizationOnboardingTasksItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamCreateResponseOrganizationOnboardingTasksItem {
    #[serde(
        rename = "completionSeen",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub completion_seen: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    pub data: ::serde_json::Value,
    #[serde(rename = "dateCompleted")]
    pub date_completed: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub task: ::std::option::Option<::std::string::String>,
}
///`TeamCreateResponseOrganizationStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamCreateResponseOrganizationStatus {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`TeamCreateResponseProjectsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamCreateResponseProjectsItem {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: TeamCreateResponseProjectsItemAvatar,
    pub color: ::std::string::String,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub name: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    pub status: ::std::string::String,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
}
///`TeamCreateResponseProjectsItemAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamCreateResponseProjectsItemAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`TeamObserveResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamObserveResponse {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: TeamObserveResponseAvatar,
    #[serde(
        rename = "dateCreated",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "externalTeams",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_teams: ::std::vec::Vec<TeamObserveResponseExternalTeamsItem>,
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "memberCount")]
    pub member_count: i64,
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub organization: ::std::option::Option<TeamObserveResponseOrganization>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub projects: ::std::vec::Vec<TeamObserveResponseProjectsItem>,
    pub slug: ::std::string::String,
    #[serde(
        rename = "teamRole",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub team_role: ::std::option::Option<::std::string::String>,
}
///`TeamObserveResponseAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamObserveResponseAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`TeamObserveResponseExternalTeamsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamObserveResponseExternalTeamsItem {
    #[serde(
        rename = "externalId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalName")]
    pub external_name: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "integrationId")]
    pub integration_id: ::std::string::String,
    pub provider: ::std::string::String,
    #[serde(rename = "teamId", skip_serializing_if = "::std::option::Option::is_none")]
    pub team_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId", skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
///`TeamObserveResponseOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamObserveResponseOrganization {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowMemberInvite")]
    pub allow_member_invite: bool,
    #[serde(rename = "allowMemberProjectCreation")]
    pub allow_member_project_creation: bool,
    #[serde(rename = "allowSuperuserAccess")]
    pub allow_superuser_access: bool,
    pub avatar: TeamObserveResponseOrganizationAvatar,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "extraOptions",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extra_options: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "hasAuthProvider")]
    pub has_auth_provider: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isEarlyAdopter")]
    pub is_early_adopter: bool,
    pub links: TeamObserveResponseOrganizationLinks,
    pub name: ::std::string::String,
    #[serde(
        rename = "onboardingTasks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub onboarding_tasks: ::std::vec::Vec<
        TeamObserveResponseOrganizationOnboardingTasksItem,
    >,
    #[serde(rename = "require2FA")]
    pub require2_fa: bool,
    pub slug: ::std::string::String,
    pub status: TeamObserveResponseOrganizationStatus,
}
///`TeamObserveResponseOrganizationAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamObserveResponseOrganizationAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`TeamObserveResponseOrganizationLinks`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamObserveResponseOrganizationLinks {
    #[serde(rename = "organizationUrl")]
    pub organization_url: ::std::string::String,
    #[serde(rename = "regionUrl")]
    pub region_url: ::std::string::String,
}
///`TeamObserveResponseOrganizationOnboardingTasksItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamObserveResponseOrganizationOnboardingTasksItem {
    #[serde(
        rename = "completionSeen",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub completion_seen: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    pub data: ::serde_json::Value,
    #[serde(rename = "dateCompleted")]
    pub date_completed: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub task: ::std::option::Option<::std::string::String>,
}
///`TeamObserveResponseOrganizationStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamObserveResponseOrganizationStatus {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`TeamObserveResponseProjectsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamObserveResponseProjectsItem {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: TeamObserveResponseProjectsItemAvatar,
    pub color: ::std::string::String,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub name: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    pub status: ::std::string::String,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
}
///`TeamObserveResponseProjectsItemAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamObserveResponseProjectsItemAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
/**Allows parameters to be defined in snake case, but passed as camel case.

Errors are output in camel case.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamUpdateRequest {
    ///The name of the team.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<TeamUpdateRequestName>,
    ///Uniquely identifies a team. This is must be available.
    pub slug: TeamUpdateRequestSlug,
}
///The name of the team.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct TeamUpdateRequestName(::std::string::String);
impl ::std::ops::Deref for TeamUpdateRequestName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TeamUpdateRequestName> for ::std::string::String {
    fn from(value: TeamUpdateRequestName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TeamUpdateRequestName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TeamUpdateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeamUpdateRequestName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TeamUpdateRequestName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Uniquely identifies a team. This is must be available.
#[derive(
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct TeamUpdateRequestSlug(::std::string::String);
impl ::std::ops::Deref for TeamUpdateRequestSlug {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TeamUpdateRequestSlug> for ::std::string::String {
    fn from(value: TeamUpdateRequestSlug) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TeamUpdateRequestSlug {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 50usize {
            return Err("longer than 50 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^(?![0-9]+$)[a-z0-9_\\-]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^(?![0-9]+$)[a-z0-9_\\-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TeamUpdateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TeamUpdateRequestSlug {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TeamUpdateRequestSlug {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`TeamUpdateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamUpdateResponse {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: TeamUpdateResponseAvatar,
    #[serde(
        rename = "dateCreated",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub date_created: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(
        rename = "externalTeams",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub external_teams: ::std::vec::Vec<TeamUpdateResponseExternalTeamsItem>,
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "memberCount")]
    pub member_count: i64,
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub organization: ::std::option::Option<TeamUpdateResponseOrganization>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub projects: ::std::vec::Vec<TeamUpdateResponseProjectsItem>,
    pub slug: ::std::string::String,
    #[serde(
        rename = "teamRole",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub team_role: ::std::option::Option<::std::string::String>,
}
///`TeamUpdateResponseAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamUpdateResponseAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`TeamUpdateResponseExternalTeamsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamUpdateResponseExternalTeamsItem {
    #[serde(
        rename = "externalId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalName")]
    pub external_name: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "integrationId")]
    pub integration_id: ::std::string::String,
    pub provider: ::std::string::String,
    #[serde(rename = "teamId", skip_serializing_if = "::std::option::Option::is_none")]
    pub team_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId", skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
///`TeamUpdateResponseOrganization`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamUpdateResponseOrganization {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub access: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowMemberInvite")]
    pub allow_member_invite: bool,
    #[serde(rename = "allowMemberProjectCreation")]
    pub allow_member_project_creation: bool,
    #[serde(rename = "allowSuperuserAccess")]
    pub allow_superuser_access: bool,
    pub avatar: TeamUpdateResponseOrganizationAvatar,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(
        rename = "extraOptions",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extra_options: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "hasAuthProvider")]
    pub has_auth_provider: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isEarlyAdopter")]
    pub is_early_adopter: bool,
    pub links: TeamUpdateResponseOrganizationLinks,
    pub name: ::std::string::String,
    #[serde(
        rename = "onboardingTasks",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub onboarding_tasks: ::std::vec::Vec<
        TeamUpdateResponseOrganizationOnboardingTasksItem,
    >,
    #[serde(rename = "require2FA")]
    pub require2_fa: bool,
    pub slug: ::std::string::String,
    pub status: TeamUpdateResponseOrganizationStatus,
}
///`TeamUpdateResponseOrganizationAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamUpdateResponseOrganizationAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
///`TeamUpdateResponseOrganizationLinks`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamUpdateResponseOrganizationLinks {
    #[serde(rename = "organizationUrl")]
    pub organization_url: ::std::string::String,
    #[serde(rename = "regionUrl")]
    pub region_url: ::std::string::String,
}
///`TeamUpdateResponseOrganizationOnboardingTasksItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamUpdateResponseOrganizationOnboardingTasksItem {
    #[serde(
        rename = "completionSeen",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub completion_seen: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    pub data: ::serde_json::Value,
    #[serde(rename = "dateCompleted")]
    pub date_completed: ::chrono::DateTime<::chrono::offset::Utc>,
    pub status: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub task: ::std::option::Option<::std::string::String>,
}
///`TeamUpdateResponseOrganizationStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamUpdateResponseOrganizationStatus {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
///`TeamUpdateResponseProjectsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamUpdateResponseProjectsItem {
    pub access: ::std::vec::Vec<::std::string::String>,
    pub avatar: TeamUpdateResponseProjectsItemAvatar,
    pub color: ::std::string::String,
    #[serde(rename = "dateCreated")]
    pub date_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub features: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "firstEvent",
        deserialize_with = "::std::option::Option::deserialize"
    )]
    pub first_event: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "firstTransactionEvent")]
    pub first_transaction_event: bool,
    #[serde(rename = "hasAccess")]
    pub has_access: bool,
    #[serde(rename = "hasFeedbacks")]
    pub has_feedbacks: bool,
    #[serde(rename = "hasFlags")]
    pub has_flags: bool,
    #[serde(rename = "hasInsightsAgentMonitoring")]
    pub has_insights_agent_monitoring: bool,
    #[serde(rename = "hasInsightsAppStart")]
    pub has_insights_app_start: bool,
    #[serde(rename = "hasInsightsAssets")]
    pub has_insights_assets: bool,
    #[serde(rename = "hasInsightsCaches")]
    pub has_insights_caches: bool,
    #[serde(rename = "hasInsightsDb")]
    pub has_insights_db: bool,
    #[serde(rename = "hasInsightsHttp")]
    pub has_insights_http: bool,
    #[serde(rename = "hasInsightsMCP")]
    pub has_insights_mcp: bool,
    #[serde(rename = "hasInsightsQueues")]
    pub has_insights_queues: bool,
    #[serde(rename = "hasInsightsScreenLoad")]
    pub has_insights_screen_load: bool,
    #[serde(rename = "hasInsightsVitals")]
    pub has_insights_vitals: bool,
    #[serde(rename = "hasLogs")]
    pub has_logs: bool,
    #[serde(rename = "hasMinifiedStackTrace")]
    pub has_minified_stack_trace: bool,
    #[serde(rename = "hasMonitors")]
    pub has_monitors: bool,
    #[serde(rename = "hasNewFeedbacks")]
    pub has_new_feedbacks: bool,
    #[serde(rename = "hasProfiles")]
    pub has_profiles: bool,
    #[serde(rename = "hasReplays")]
    pub has_replays: bool,
    #[serde(rename = "hasSessions")]
    pub has_sessions: bool,
    #[serde(rename = "hasTraceMetrics")]
    pub has_trace_metrics: bool,
    pub id: ::std::string::String,
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(rename = "isMember")]
    pub is_member: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub name: ::std::string::String,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sessionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_stats: ::std::option::Option<::serde_json::Value>,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub stats: ::std::option::Option<::serde_json::Value>,
    pub status: ::std::string::String,
    #[serde(
        rename = "transactionStats",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transaction_stats: ::std::option::Option<::serde_json::Value>,
}
///`TeamUpdateResponseProjectsItemAvatar`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TeamUpdateResponseProjectsItemAvatar {
    #[serde(
        rename = "avatarType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_url: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "avatarUuid",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_uuid: ::std::option::Option<::std::string::String>,
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn monitor_create_request_status() -> super::MonitorCreateRequestStatus {
        super::MonitorCreateRequestStatus::Active
    }
    pub(super) fn monitor_update_request_status() -> super::MonitorUpdateRequestStatus {
        super::MonitorUpdateRequestStatus::Active
    }
    pub(super) fn organization_member_create_request_org_role() -> super::OrganizationMemberCreateRequestOrgRole {
        super::OrganizationMemberCreateRequestOrgRole::Member
    }
    pub(super) fn organization_member_create_request_role() -> super::OrganizationMemberCreateRequestRole {
        super::OrganizationMemberCreateRequestRole::Member
    }
    pub(super) fn project_key_create_request_use_case() -> super::ProjectKeyCreateRequestUseCase {
        super::ProjectKeyCreateRequestUseCase::User
    }
    pub(super) fn sentry_v1alpha1_monitor_for_provider_status() -> super::SentryV1alpha1MonitorForProviderStatus {
        super::SentryV1alpha1MonitorForProviderStatus::Active
    }
    pub(super) fn sentry_v1alpha1_organization_member_for_provider_org_role() -> super::SentryV1alpha1OrganizationMemberForProviderOrgRole {
        super::SentryV1alpha1OrganizationMemberForProviderOrgRole::Member
    }
    pub(super) fn sentry_v1alpha1_organization_member_for_provider_role() -> super::SentryV1alpha1OrganizationMemberForProviderRole {
        super::SentryV1alpha1OrganizationMemberForProviderRole::Member
    }
    pub(super) fn sentry_v1alpha1_project_key_for_provider_use_case() -> super::SentryV1alpha1ProjectKeyForProviderUseCase {
        super::SentryV1alpha1ProjectKeyForProviderUseCase::User
    }
}
/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
