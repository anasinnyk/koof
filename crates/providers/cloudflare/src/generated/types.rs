//! This file is generated. Do not edit manually.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used)]
///`AccountAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct AccountAtProvider {
    ///Timestamp for the creation of the account
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_on: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<IamCommonComponentsSchemasIdentifier>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub managed_by: ::std::option::Option<AccountAtProviderManagedBy>,
    ///Account name
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<AccountAtProviderName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<AccountAtProviderSettings>,
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<IamAccountType>,
}
///Parent container details
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct AccountAtProviderManagedBy {
    ///ID of the parent Organization, if one exists
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_org_id: ::std::option::Option<AccountAtProviderManagedByParentOrgId>,
    ///Name of the parent Organization, if one exists
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_org_name: ::std::option::Option<::std::string::String>,
}
///ID of the parent Organization, if one exists
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
pub struct AccountAtProviderManagedByParentOrgId(::std::string::String);
impl ::std::ops::Deref for AccountAtProviderManagedByParentOrgId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AccountAtProviderManagedByParentOrgId>
for ::std::string::String {
    fn from(value: AccountAtProviderManagedByParentOrgId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for AccountAtProviderManagedByParentOrgId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AccountAtProviderManagedByParentOrgId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for AccountAtProviderManagedByParentOrgId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AccountAtProviderManagedByParentOrgId {
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
///Account name
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
pub struct AccountAtProviderName(::std::string::String);
impl ::std::ops::Deref for AccountAtProviderName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AccountAtProviderName> for ::std::string::String {
    fn from(value: AccountAtProviderName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for AccountAtProviderName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AccountAtProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccountAtProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AccountAtProviderName {
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
///Account settings
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct AccountAtProviderSettings {
    ///Sets an abuse contact email to notify for abuse reports.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub abuse_contact_email: ::std::option::Option<::std::string::String>,
    /**Indicates whether membership in this account requires that
Two-Factor Authentication is enabled*/
    #[serde(default)]
    pub enforce_twofactor: bool,
}
///`AccountForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct AccountForProvider {
    ///Account name
    pub name: AccountForProviderName,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<AccountForProviderSettings>,
}
///Account name
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
pub struct AccountForProviderName(::std::string::String);
impl ::std::ops::Deref for AccountForProviderName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AccountForProviderName> for ::std::string::String {
    fn from(value: AccountForProviderName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for AccountForProviderName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AccountForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccountForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AccountForProviderName {
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
///Account settings
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct AccountForProviderSettings {
    ///Sets an abuse contact email to notify for abuse reports.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub abuse_contact_email: ::std::option::Option<::std::string::String>,
    /**Indicates whether membership in this account requires that
Two-Factor Authentication is enabled*/
    #[serde(default)]
    pub enforce_twofactor: bool,
}
///How the header value is treated when building the cache key.
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
pub enum Action {
    #[serde(rename = "bypass")]
    Bypass,
    #[serde(rename = "passthrough")]
    Passthrough,
    #[serde(rename = "normalize")]
    Normalize,
}
impl ::std::fmt::Display for Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bypass => f.write_str("bypass"),
            Self::Passthrough => f.write_str("passthrough"),
            Self::Normalize => f.write_str("normalize"),
        }
    }
}
impl ::std::str::FromStr for Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bypass" => Ok(Self::Bypass),
            "passthrough" => Ok(Self::Passthrough),
            "normalize" => Ok(Self::Normalize),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AddCacheTagsExpressionOperation`
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
pub enum AddCacheTagsExpressionOperation {
    #[serde(rename = "add")]
    Add,
}
impl ::std::fmt::Display for AddCacheTagsExpressionOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Add => f.write_str("add"),
        }
    }
}
impl ::std::str::FromStr for AddCacheTagsExpressionOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "add" => Ok(Self::Add),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AddCacheTagsExpressionOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AddCacheTagsExpressionOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AddCacheTagsValuesOperation`
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
pub enum AddCacheTagsValuesOperation {
    #[serde(rename = "add")]
    Add,
}
impl ::std::fmt::Display for AddCacheTagsValuesOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Add => f.write_str("add"),
        }
    }
}
impl ::std::str::FromStr for AddCacheTagsValuesOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "add" => Ok(Self::Add),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AddCacheTagsValuesOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AddCacheTagsValuesOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AddDynamicHeaderOperation`
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
pub enum AddDynamicHeaderOperation {
    #[serde(rename = "add")]
    Add,
}
impl ::std::fmt::Display for AddDynamicHeaderOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Add => f.write_str("add"),
        }
    }
}
impl ::std::str::FromStr for AddDynamicHeaderOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "add" => Ok(Self::Add),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AddDynamicHeaderOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AddDynamicHeaderOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AddStaticHeaderOperation`
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
pub enum AddStaticHeaderOperation {
    #[serde(rename = "add")]
    Add,
}
impl ::std::fmt::Display for AddStaticHeaderOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Add => f.write_str("add"),
        }
    }
}
impl ::std::str::FromStr for AddStaticHeaderOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "add" => Ok(Self::Add),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AddStaticHeaderOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AddStaticHeaderOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Compression algorithm to enable.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct Algorithm {
    ///Name of the compression algorithm to enable.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<AlgorithmName>,
}
///Name of the compression algorithm to enable.
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
pub enum AlgorithmName {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "brotli")]
    Brotli,
    #[serde(rename = "zstd")]
    Zstd,
}
impl ::std::fmt::Display for AlgorithmName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Auto => f.write_str("auto"),
            Self::Default => f.write_str("default"),
            Self::Gzip => f.write_str("gzip"),
            Self::Brotli => f.write_str("brotli"),
            Self::Zstd => f.write_str("zstd"),
        }
    }
}
impl ::std::str::FromStr for AlgorithmName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "auto" => Ok(Self::Auto),
            "default" => Ok(Self::Default),
            "gzip" => Ok(Self::Gzip),
            "brotli" => Ok(Self::Brotli),
            "zstd" => Ok(Self::Zstd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AlgorithmName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AlgorithmName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A cache tag value.
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
pub struct CacheTag(::std::string::String);
impl ::std::ops::Deref for CacheTag {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CacheTag> for ::std::string::String {
    fn from(value: CacheTag) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CacheTag {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CacheTag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CacheTag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CacheTag {
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
///A category-level override.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct CategoryOverride {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsRuleAction>,
    pub category: RulesetsRuleCategory,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sensitivity_level: ::std::option::Option<RulesetsExecuteSensitivityLevel>,
}
///The characteristic of the request.
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
pub struct Characteristic(::std::string::String);
impl ::std::ops::Deref for Characteristic {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Characteristic> for ::std::string::String {
    fn from(value: Characteristic) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Characteristic {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Characteristic {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Characteristic {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Characteristic {
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
///The content to return.
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
pub struct Content(::std::string::String);
impl ::std::ops::Deref for Content {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Content> for ::std::string::String {
    fn from(value: Content) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Content {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Content {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Content {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Content {
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
///The type of the content to return.
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
pub struct ContentType(::std::string::String);
impl ::std::ops::Deref for ContentType {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ContentType> for ::std::string::String {
    fn from(value: ContentType) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ContentType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ContentType {
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
///The cookie field to log.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct CookieField {
    ///The name of the cookie.
    pub name: CookieName,
}
///The name of the cookie.
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
pub struct CookieName(::std::string::String);
impl ::std::ops::Deref for CookieName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CookieName> for ::std::string::String {
    fn from(value: CookieName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CookieName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CookieName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CookieName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CookieName {
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
///An expression that defines when the rate limit counter should be incremented. It defaults to the same as the rule's expression.
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
pub struct CountingExpression(::std::string::String);
impl ::std::ops::Deref for CountingExpression {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CountingExpression> for ::std::string::String {
    fn from(value: CountingExpression) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CountingExpression {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CountingExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CountingExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CountingExpression {
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
///A text description of this message.
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
pub struct Description(::std::string::String);
impl ::std::ops::Deref for Description {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Description> for ::std::string::String {
    fn from(value: Description) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Description {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Description {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Description {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Description {
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
///`DnsRecordAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProvider {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A valid IPv4 address.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    ///When the record was created.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_on: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordAtProviderData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<DnsRecordsIdentifier>,
    ///When the record was last modified.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub modified_on: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<DnsRecordsPriority>,
    ///Whether the record can be proxied by Cloudflare or not.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxiable: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordAtProviderSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordAtProviderType>,
}
///`DnsRecordAtProviderData`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderData {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<DnsRecordAtProviderDataSubtype0>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<DnsRecordAtProviderDataSubtype1>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<DnsRecordAtProviderDataSubtype2>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_3: ::std::option::Option<DnsRecordAtProviderDataSubtype3>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_4: ::std::option::Option<DnsRecordAtProviderDataSubtype4>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_5: ::std::option::Option<DnsRecordAtProviderDataSubtype5>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_6: ::std::option::Option<DnsRecordAtProviderDataSubtype6>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_7: ::std::option::Option<DnsRecordAtProviderDataSubtype7>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_8: ::std::option::Option<DnsRecordAtProviderDataSubtype8>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_9: ::std::option::Option<DnsRecordAtProviderDataSubtype9>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_10: ::std::option::Option<DnsRecordAtProviderDataSubtype10>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_11: ::std::option::Option<DnsRecordAtProviderDataSubtype11>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_12: ::std::option::Option<DnsRecordAtProviderDataSubtype12>,
}
///Components of a CAA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype0 {
    ///Flags for the CAA record.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<f64>,
    ///Name of the property controlled by this record (e.g.: issue, issuewild, iodef).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tag: ::std::option::Option<::std::string::String>,
    ///Value of the record. This field's semantics depend on the chosen tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Components of a CERT record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype1 {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Key Tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub key_tag: ::std::option::Option<f64>,
    ///Type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<f64>,
}
///Components of a SVCB record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype10 {
    ///Priority.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
    ///Target.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///Value.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Components of a TLSA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype11 {
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Matching Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub matching_type: ::std::option::Option<f64>,
    ///Selector.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub selector: ::std::option::Option<f64>,
    ///Usage.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<f64>,
}
///Components of a URI record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype12 {
    ///The record content.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///The record weight.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub weight: ::std::option::Option<f64>,
}
///Components of a DNSKEY record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype2 {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Flags.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<f64>,
    ///Protocol.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<f64>,
    ///Public Key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub public_key: ::std::option::Option<::std::string::String>,
}
///Components of a DS record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype3 {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Digest.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub digest: ::std::option::Option<::std::string::String>,
    ///Digest Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub digest_type: ::std::option::Option<f64>,
    ///Key Tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub key_tag: ::std::option::Option<f64>,
}
///Components of a HTTPS record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype4 {
    ///Priority.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
    ///Target.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///Value.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Components of a LOC record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype5 {
    ///Altitude of location in meters.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub altitude: ::std::option::Option<f64>,
    ///Degrees of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_degrees: ::std::option::Option<f64>,
    ///Latitude direction.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_direction: ::std::option::Option<
        DnsRecordAtProviderDataSubtype5LatDirection,
    >,
    ///Minutes of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_minutes: ::std::option::Option<f64>,
    ///Seconds of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_seconds: ::std::option::Option<f64>,
    ///Degrees of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_degrees: ::std::option::Option<f64>,
    ///Longitude direction.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_direction: ::std::option::Option<
        DnsRecordAtProviderDataSubtype5LongDirection,
    >,
    ///Minutes of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_minutes: ::std::option::Option<f64>,
    ///Seconds of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_seconds: ::std::option::Option<f64>,
    ///Horizontal precision of location.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub precision_horz: ::std::option::Option<f64>,
    ///Vertical precision of location.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub precision_vert: ::std::option::Option<f64>,
    ///Size of location in meters.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<f64>,
}
///Latitude direction.
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
pub enum DnsRecordAtProviderDataSubtype5LatDirection {
    N,
    S,
}
impl ::std::fmt::Display for DnsRecordAtProviderDataSubtype5LatDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::N => f.write_str("N"),
            Self::S => f.write_str("S"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordAtProviderDataSubtype5LatDirection {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "N" => Ok(Self::N),
            "S" => Ok(Self::S),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordAtProviderDataSubtype5LatDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordAtProviderDataSubtype5LatDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Longitude direction.
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
pub enum DnsRecordAtProviderDataSubtype5LongDirection {
    E,
    W,
}
impl ::std::fmt::Display for DnsRecordAtProviderDataSubtype5LongDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::E => f.write_str("E"),
            Self::W => f.write_str("W"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordAtProviderDataSubtype5LongDirection {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "E" => Ok(Self::E),
            "W" => Ok(Self::W),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordAtProviderDataSubtype5LongDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordAtProviderDataSubtype5LongDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a NAPTR record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype6 {
    ///Flags.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<::std::string::String>,
    ///Order.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<f64>,
    ///Preference.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub preference: ::std::option::Option<f64>,
    ///Regex.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub regex: ::std::option::Option<::std::string::String>,
    ///Replacement.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub replacement: ::std::option::Option<::std::string::String>,
    ///Service.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service: ::std::option::Option<::std::string::String>,
}
///Components of a SMIMEA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype7 {
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Matching Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub matching_type: ::std::option::Option<f64>,
    ///Selector.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub selector: ::std::option::Option<f64>,
    ///Usage.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<f64>,
}
///Components of a SRV record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype8 {
    ///The port of the service.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub port: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<DnsRecordsPriority>,
    ///A valid hostname.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///The record weight.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub weight: ::std::option::Option<f64>,
}
///Components of a SSHFP record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderDataSubtype9 {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Fingerprint.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    ///Type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<f64>,
}
///`DnsRecordAtProviderSettings`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderSettings {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<DnsRecordsSettings>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<DnsRecordAtProviderSettingsSubtype1>,
}
///`DnsRecordAtProviderSettingsSubtype1`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordAtProviderSettingsSubtype1 {
    ///If enabled, causes the CNAME record to be resolved externally and the resulting address records (e.g., A and AAAA) to be returned instead of the CNAME record itself. This setting is unavailable for proxied records, since they are always flattened.
    #[serde(default)]
    pub flatten_cname: bool,
}
///Record type.
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
pub enum DnsRecordAtProviderType {
    A,
    #[serde(rename = "AAAA")]
    Aaaa,
    #[serde(rename = "CNAME")]
    Cname,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "NS")]
    Ns,
    #[serde(rename = "OPENPGPKEY")]
    Openpgpkey,
    #[serde(rename = "PTR")]
    Ptr,
    #[serde(rename = "TXT")]
    Txt,
    #[serde(rename = "CAA")]
    Caa,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "DNSKEY")]
    Dnskey,
    #[serde(rename = "DS")]
    Ds,
    #[serde(rename = "HTTPS")]
    Https,
    #[serde(rename = "LOC")]
    Loc,
    #[serde(rename = "NAPTR")]
    Naptr,
    #[serde(rename = "SMIMEA")]
    Smimea,
    #[serde(rename = "SRV")]
    Srv,
    #[serde(rename = "SSHFP")]
    Sshfp,
    #[serde(rename = "SVCB")]
    Svcb,
    #[serde(rename = "TLSA")]
    Tlsa,
    #[serde(rename = "URI")]
    Uri,
}
impl ::std::fmt::Display for DnsRecordAtProviderType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::A => f.write_str("A"),
            Self::Aaaa => f.write_str("AAAA"),
            Self::Cname => f.write_str("CNAME"),
            Self::Mx => f.write_str("MX"),
            Self::Ns => f.write_str("NS"),
            Self::Openpgpkey => f.write_str("OPENPGPKEY"),
            Self::Ptr => f.write_str("PTR"),
            Self::Txt => f.write_str("TXT"),
            Self::Caa => f.write_str("CAA"),
            Self::Cert => f.write_str("CERT"),
            Self::Dnskey => f.write_str("DNSKEY"),
            Self::Ds => f.write_str("DS"),
            Self::Https => f.write_str("HTTPS"),
            Self::Loc => f.write_str("LOC"),
            Self::Naptr => f.write_str("NAPTR"),
            Self::Smimea => f.write_str("SMIMEA"),
            Self::Srv => f.write_str("SRV"),
            Self::Sshfp => f.write_str("SSHFP"),
            Self::Svcb => f.write_str("SVCB"),
            Self::Tlsa => f.write_str("TLSA"),
            Self::Uri => f.write_str("URI"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordAtProviderType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "A" => Ok(Self::A),
            "AAAA" => Ok(Self::Aaaa),
            "CNAME" => Ok(Self::Cname),
            "MX" => Ok(Self::Mx),
            "NS" => Ok(Self::Ns),
            "OPENPGPKEY" => Ok(Self::Openpgpkey),
            "PTR" => Ok(Self::Ptr),
            "TXT" => Ok(Self::Txt),
            "CAA" => Ok(Self::Caa),
            "CERT" => Ok(Self::Cert),
            "DNSKEY" => Ok(Self::Dnskey),
            "DS" => Ok(Self::Ds),
            "HTTPS" => Ok(Self::Https),
            "LOC" => Ok(Self::Loc),
            "NAPTR" => Ok(Self::Naptr),
            "SMIMEA" => Ok(Self::Smimea),
            "SRV" => Ok(Self::Srv),
            "SSHFP" => Ok(Self::Sshfp),
            "SVCB" => Ok(Self::Svcb),
            "TLSA" => Ok(Self::Tlsa),
            "URI" => Ok(Self::Uri),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordAtProviderType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordAtProviderType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordDeleteResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordDeleteResponse {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<DnsRecordDeleteResponseResult>,
}
///`DnsRecordDeleteResponseResult`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordDeleteResponseResult {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<DnsRecordsIdentifier>,
}
///`DnsRecordForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum DnsRecordForProvider {
    ARecord(DnsRecordForProviderARecord),
    AaaaRecord(DnsRecordForProviderAaaaRecord),
    CnameRecord(DnsRecordForProviderCnameRecord),
    MxRecord(DnsRecordForProviderMxRecord),
    NsRecord(DnsRecordForProviderNsRecord),
    OpenpgpkeyRecord(DnsRecordForProviderOpenpgpkeyRecord),
    PtrRecord(DnsRecordForProviderPtrRecord),
    TxtRecord(DnsRecordForProviderTxtRecord),
    CaaRecord(DnsRecordForProviderCaaRecord),
    CertRecord(DnsRecordForProviderCertRecord),
    DnskeyRecord(DnsRecordForProviderDnskeyRecord),
    DsRecord(DnsRecordForProviderDsRecord),
    HttpsRecord(DnsRecordForProviderHttpsRecord),
    LocRecord(DnsRecordForProviderLocRecord),
    NaptrRecord(DnsRecordForProviderNaptrRecord),
    SmimeaRecord(DnsRecordForProviderSmimeaRecord),
    SrvRecord(DnsRecordForProviderSrvRecord),
    SshfpRecord(DnsRecordForProviderSshfpRecord),
    SvcbRecord(DnsRecordForProviderSvcbRecord),
    TlsaRecord(DnsRecordForProviderTlsaRecord),
    UriRecord(DnsRecordForProviderUriRecord),
}
impl ::std::convert::From<DnsRecordForProviderARecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderARecord) -> Self {
        Self::ARecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderAaaaRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderAaaaRecord) -> Self {
        Self::AaaaRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderCnameRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderCnameRecord) -> Self {
        Self::CnameRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderMxRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderMxRecord) -> Self {
        Self::MxRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderNsRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderNsRecord) -> Self {
        Self::NsRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderOpenpgpkeyRecord>
for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderOpenpgpkeyRecord) -> Self {
        Self::OpenpgpkeyRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderPtrRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderPtrRecord) -> Self {
        Self::PtrRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderTxtRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderTxtRecord) -> Self {
        Self::TxtRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderCaaRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderCaaRecord) -> Self {
        Self::CaaRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderCertRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderCertRecord) -> Self {
        Self::CertRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderDnskeyRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderDnskeyRecord) -> Self {
        Self::DnskeyRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderDsRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderDsRecord) -> Self {
        Self::DsRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderHttpsRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderHttpsRecord) -> Self {
        Self::HttpsRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderLocRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderLocRecord) -> Self {
        Self::LocRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderNaptrRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderNaptrRecord) -> Self {
        Self::NaptrRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderSmimeaRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderSmimeaRecord) -> Self {
        Self::SmimeaRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderSrvRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderSrvRecord) -> Self {
        Self::SrvRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderSshfpRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderSshfpRecord) -> Self {
        Self::SshfpRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderSvcbRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderSvcbRecord) -> Self {
        Self::SvcbRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderTlsaRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderTlsaRecord) -> Self {
        Self::TlsaRecord(value)
    }
}
impl ::std::convert::From<DnsRecordForProviderUriRecord> for DnsRecordForProvider {
    fn from(value: DnsRecordForProviderUriRecord) -> Self {
        Self::UriRecord(value)
    }
}
///`DnsRecordForProviderARecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderARecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A valid IPv4 address.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::net::Ipv4Addr>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderARecordType>,
}
///Record type.
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
pub enum DnsRecordForProviderARecordType {
    A,
}
impl ::std::fmt::Display for DnsRecordForProviderARecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::A => f.write_str("A"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderARecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "A" => Ok(Self::A),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderARecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordForProviderARecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderAaaaRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderAaaaRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A valid IPv6 address.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::net::Ipv6Addr>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderAaaaRecordType>,
}
///Record type.
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
pub enum DnsRecordForProviderAaaaRecordType {
    #[serde(rename = "AAAA")]
    Aaaa,
}
impl ::std::fmt::Display for DnsRecordForProviderAaaaRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Aaaa => f.write_str("AAAA"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderAaaaRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "AAAA" => Ok(Self::Aaaa),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderAaaaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderAaaaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderCaaRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderCaaRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted CAA content. See 'data' to set CAA properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderCaaRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderCaaRecordType>,
}
///Components of a CAA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderCaaRecordData {
    ///Flags for the CAA record.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<f64>,
    ///Name of the property controlled by this record (e.g.: issue, issuewild, iodef).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tag: ::std::option::Option<::std::string::String>,
    ///Value of the record. This field's semantics depend on the chosen tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Record type.
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
pub enum DnsRecordForProviderCaaRecordType {
    #[serde(rename = "CAA")]
    Caa,
}
impl ::std::fmt::Display for DnsRecordForProviderCaaRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Caa => f.write_str("CAA"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderCaaRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CAA" => Ok(Self::Caa),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderCaaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderCaaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderCertRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderCertRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted CERT content. See 'data' to set CERT properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderCertRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderCertRecordType>,
}
///Components of a CERT record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderCertRecordData {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Key Tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub key_tag: ::std::option::Option<f64>,
    ///Type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordForProviderCertRecordType {
    #[serde(rename = "CERT")]
    Cert,
}
impl ::std::fmt::Display for DnsRecordForProviderCertRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cert => f.write_str("CERT"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderCertRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CERT" => Ok(Self::Cert),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderCertRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderCertRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderCnameRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderCnameRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A valid hostname. Must not match the record's name.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordForProviderCnameRecordSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderCnameRecordType>,
}
///`DnsRecordForProviderCnameRecordSettings`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderCnameRecordSettings {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<DnsRecordsSettings>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<
        DnsRecordForProviderCnameRecordSettingsSubtype1,
    >,
}
///`DnsRecordForProviderCnameRecordSettingsSubtype1`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderCnameRecordSettingsSubtype1 {
    ///If enabled, causes the CNAME record to be resolved externally and the resulting address records (e.g., A and AAAA) to be returned instead of the CNAME record itself. This setting is unavailable for proxied records, since they are always flattened.
    #[serde(default)]
    pub flatten_cname: bool,
}
///Record type.
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
pub enum DnsRecordForProviderCnameRecordType {
    #[serde(rename = "CNAME")]
    Cname,
}
impl ::std::fmt::Display for DnsRecordForProviderCnameRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cname => f.write_str("CNAME"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderCnameRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CNAME" => Ok(Self::Cname),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderCnameRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderCnameRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderDnskeyRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderDnskeyRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted DNSKEY content. See 'data' to set DNSKEY properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderDnskeyRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderDnskeyRecordType>,
}
///Components of a DNSKEY record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderDnskeyRecordData {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Flags.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<f64>,
    ///Protocol.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<f64>,
    ///Public Key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub public_key: ::std::option::Option<::std::string::String>,
}
///Record type.
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
pub enum DnsRecordForProviderDnskeyRecordType {
    #[serde(rename = "DNSKEY")]
    Dnskey,
}
impl ::std::fmt::Display for DnsRecordForProviderDnskeyRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dnskey => f.write_str("DNSKEY"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderDnskeyRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "DNSKEY" => Ok(Self::Dnskey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderDnskeyRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderDnskeyRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderDsRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderDsRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted DS content. See 'data' to set DS properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderDsRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderDsRecordType>,
}
///Components of a DS record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderDsRecordData {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Digest.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub digest: ::std::option::Option<::std::string::String>,
    ///Digest Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub digest_type: ::std::option::Option<f64>,
    ///Key Tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub key_tag: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordForProviderDsRecordType {
    #[serde(rename = "DS")]
    Ds,
}
impl ::std::fmt::Display for DnsRecordForProviderDsRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ds => f.write_str("DS"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderDsRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "DS" => Ok(Self::Ds),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderDsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderDsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderHttpsRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderHttpsRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted HTTPS content. See 'data' to set HTTPS properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderHttpsRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderHttpsRecordType>,
}
///Components of a HTTPS record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderHttpsRecordData {
    ///Priority.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
    ///Target.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///Value.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Record type.
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
pub enum DnsRecordForProviderHttpsRecordType {
    #[serde(rename = "HTTPS")]
    Https,
}
impl ::std::fmt::Display for DnsRecordForProviderHttpsRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Https => f.write_str("HTTPS"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderHttpsRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "HTTPS" => Ok(Self::Https),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderHttpsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderHttpsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderLocRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderLocRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted LOC content. See 'data' to set LOC properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderLocRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderLocRecordType>,
}
///Components of a LOC record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderLocRecordData {
    ///Altitude of location in meters.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub altitude: ::std::option::Option<f64>,
    ///Degrees of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_degrees: ::std::option::Option<f64>,
    ///Latitude direction.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_direction: ::std::option::Option<
        DnsRecordForProviderLocRecordDataLatDirection,
    >,
    ///Minutes of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_minutes: ::std::option::Option<f64>,
    ///Seconds of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_seconds: ::std::option::Option<f64>,
    ///Degrees of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_degrees: ::std::option::Option<f64>,
    ///Longitude direction.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_direction: ::std::option::Option<
        DnsRecordForProviderLocRecordDataLongDirection,
    >,
    ///Minutes of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_minutes: ::std::option::Option<f64>,
    ///Seconds of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_seconds: ::std::option::Option<f64>,
    ///Horizontal precision of location.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub precision_horz: ::std::option::Option<f64>,
    ///Vertical precision of location.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub precision_vert: ::std::option::Option<f64>,
    ///Size of location in meters.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<f64>,
}
///Latitude direction.
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
pub enum DnsRecordForProviderLocRecordDataLatDirection {
    N,
    S,
}
impl ::std::fmt::Display for DnsRecordForProviderLocRecordDataLatDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::N => f.write_str("N"),
            Self::S => f.write_str("S"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderLocRecordDataLatDirection {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "N" => Ok(Self::N),
            "S" => Ok(Self::S),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderLocRecordDataLatDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderLocRecordDataLatDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Longitude direction.
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
pub enum DnsRecordForProviderLocRecordDataLongDirection {
    E,
    W,
}
impl ::std::fmt::Display for DnsRecordForProviderLocRecordDataLongDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::E => f.write_str("E"),
            Self::W => f.write_str("W"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderLocRecordDataLongDirection {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "E" => Ok(Self::E),
            "W" => Ok(Self::W),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderLocRecordDataLongDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderLocRecordDataLongDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Record type.
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
pub enum DnsRecordForProviderLocRecordType {
    #[serde(rename = "LOC")]
    Loc,
}
impl ::std::fmt::Display for DnsRecordForProviderLocRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Loc => f.write_str("LOC"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderLocRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "LOC" => Ok(Self::Loc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderLocRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderLocRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderMxRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderMxRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A valid mail server hostname.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<DnsRecordsPriority>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderMxRecordType>,
}
///Record type.
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
pub enum DnsRecordForProviderMxRecordType {
    #[serde(rename = "MX")]
    Mx,
}
impl ::std::fmt::Display for DnsRecordForProviderMxRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mx => f.write_str("MX"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderMxRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "MX" => Ok(Self::Mx),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderMxRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderMxRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderNaptrRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderNaptrRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted NAPTR content. See 'data' to set NAPTR properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderNaptrRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderNaptrRecordType>,
}
///Components of a NAPTR record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderNaptrRecordData {
    ///Flags.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<::std::string::String>,
    ///Order.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<f64>,
    ///Preference.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub preference: ::std::option::Option<f64>,
    ///Regex.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub regex: ::std::option::Option<::std::string::String>,
    ///Replacement.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub replacement: ::std::option::Option<::std::string::String>,
    ///Service.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service: ::std::option::Option<::std::string::String>,
}
///Record type.
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
pub enum DnsRecordForProviderNaptrRecordType {
    #[serde(rename = "NAPTR")]
    Naptr,
}
impl ::std::fmt::Display for DnsRecordForProviderNaptrRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Naptr => f.write_str("NAPTR"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderNaptrRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NAPTR" => Ok(Self::Naptr),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderNaptrRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderNaptrRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderNsRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderNsRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A valid name server host name.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderNsRecordType>,
}
///Record type.
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
pub enum DnsRecordForProviderNsRecordType {
    #[serde(rename = "NS")]
    Ns,
}
impl ::std::fmt::Display for DnsRecordForProviderNsRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ns => f.write_str("NS"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderNsRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NS" => Ok(Self::Ns),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderNsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderNsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderOpenpgpkeyRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderOpenpgpkeyRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A single Base64-encoded OpenPGP Transferable Public Key (RFC 4880 Section 11.1)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderOpenpgpkeyRecordType>,
}
///Record type.
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
pub enum DnsRecordForProviderOpenpgpkeyRecordType {
    #[serde(rename = "OPENPGPKEY")]
    Openpgpkey,
}
impl ::std::fmt::Display for DnsRecordForProviderOpenpgpkeyRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Openpgpkey => f.write_str("OPENPGPKEY"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderOpenpgpkeyRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "OPENPGPKEY" => Ok(Self::Openpgpkey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderOpenpgpkeyRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderOpenpgpkeyRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderPtrRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderPtrRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Domain name pointing to the address.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderPtrRecordType>,
}
///Record type.
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
pub enum DnsRecordForProviderPtrRecordType {
    #[serde(rename = "PTR")]
    Ptr,
}
impl ::std::fmt::Display for DnsRecordForProviderPtrRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ptr => f.write_str("PTR"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderPtrRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "PTR" => Ok(Self::Ptr),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderPtrRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderPtrRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderSmimeaRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderSmimeaRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted SMIMEA content. See 'data' to set SMIMEA properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderSmimeaRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderSmimeaRecordType>,
}
///Components of a SMIMEA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderSmimeaRecordData {
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Matching Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub matching_type: ::std::option::Option<f64>,
    ///Selector.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub selector: ::std::option::Option<f64>,
    ///Usage.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordForProviderSmimeaRecordType {
    #[serde(rename = "SMIMEA")]
    Smimea,
}
impl ::std::fmt::Display for DnsRecordForProviderSmimeaRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Smimea => f.write_str("SMIMEA"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderSmimeaRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SMIMEA" => Ok(Self::Smimea),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderSmimeaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderSmimeaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderSrvRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderSrvRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Priority, weight, port, and SRV target. See 'data' for setting the individual component values.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderSrvRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderSrvRecordType>,
}
///Components of a SRV record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderSrvRecordData {
    ///The port of the service.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub port: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<DnsRecordsPriority>,
    ///A valid hostname.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///The record weight.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub weight: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordForProviderSrvRecordType {
    #[serde(rename = "SRV")]
    Srv,
}
impl ::std::fmt::Display for DnsRecordForProviderSrvRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Srv => f.write_str("SRV"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderSrvRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SRV" => Ok(Self::Srv),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderSrvRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderSrvRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderSshfpRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderSshfpRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted SSHFP content. See 'data' to set SSHFP properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderSshfpRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderSshfpRecordType>,
}
///Components of a SSHFP record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderSshfpRecordData {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Fingerprint.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    ///Type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordForProviderSshfpRecordType {
    #[serde(rename = "SSHFP")]
    Sshfp,
}
impl ::std::fmt::Display for DnsRecordForProviderSshfpRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sshfp => f.write_str("SSHFP"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderSshfpRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SSHFP" => Ok(Self::Sshfp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderSshfpRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderSshfpRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderSvcbRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderSvcbRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted SVCB content. See 'data' to set SVCB properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderSvcbRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderSvcbRecordType>,
}
///Components of a SVCB record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderSvcbRecordData {
    ///Priority.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
    ///Target.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///Value.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Record type.
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
pub enum DnsRecordForProviderSvcbRecordType {
    #[serde(rename = "SVCB")]
    Svcb,
}
impl ::std::fmt::Display for DnsRecordForProviderSvcbRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Svcb => f.write_str("SVCB"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderSvcbRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SVCB" => Ok(Self::Svcb),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderSvcbRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderSvcbRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderTlsaRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderTlsaRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted TLSA content. See 'data' to set TLSA properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderTlsaRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderTlsaRecordType>,
}
///Components of a TLSA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderTlsaRecordData {
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Matching Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub matching_type: ::std::option::Option<f64>,
    ///Selector.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub selector: ::std::option::Option<f64>,
    ///Usage.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordForProviderTlsaRecordType {
    #[serde(rename = "TLSA")]
    Tlsa,
}
impl ::std::fmt::Display for DnsRecordForProviderTlsaRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Tlsa => f.write_str("TLSA"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderTlsaRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "TLSA" => Ok(Self::Tlsa),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderTlsaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderTlsaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderTxtRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderTxtRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    /**Text content for the record. The content must consist of quoted "character strings" (RFC 1035), each with a length of up to 255 bytes. Strings exceeding this allowed maximum length are automatically split.

Learn more at <https://www.cloudflare.com/learning/dns/dns-records/dns-txt-record/>.*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderTxtRecordType>,
}
///Record type.
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
pub enum DnsRecordForProviderTxtRecordType {
    #[serde(rename = "TXT")]
    Txt,
}
impl ::std::fmt::Display for DnsRecordForProviderTxtRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Txt => f.write_str("TXT"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderTxtRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "TXT" => Ok(Self::Txt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderTxtRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderTxtRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordForProviderUriRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderUriRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted URI content. See 'data' to set URI properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordForProviderUriRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<DnsRecordsPriority>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordForProviderUriRecordType>,
}
///Components of a URI record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordForProviderUriRecordData {
    ///The record content.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///The record weight.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub weight: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordForProviderUriRecordType {
    #[serde(rename = "URI")]
    Uri,
}
impl ::std::fmt::Display for DnsRecordForProviderUriRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Uri => f.write_str("URI"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordForProviderUriRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "URI" => Ok(Self::Uri),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordForProviderUriRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordForProviderUriRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsARecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsARecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A valid IPv4 address.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::net::Ipv4Addr>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    ///Enables private network routing to the origin.
    #[serde(default)]
    pub private_routing: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsARecordType>,
}
///Record type.
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
pub enum DnsRecordsARecordType {
    A,
}
impl ::std::fmt::Display for DnsRecordsARecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::A => f.write_str("A"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsARecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "A" => Ok(Self::A),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsARecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsARecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsAaaaRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsAaaaRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A valid IPv6 address.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::net::Ipv6Addr>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    ///Enables private network routing to the origin.
    #[serde(default)]
    pub private_routing: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsAaaaRecordType>,
}
///Record type.
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
pub enum DnsRecordsAaaaRecordType {
    #[serde(rename = "AAAA")]
    Aaaa,
}
impl ::std::fmt::Display for DnsRecordsAaaaRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Aaaa => f.write_str("AAAA"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsAaaaRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "AAAA" => Ok(Self::Aaaa),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsAaaaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsAaaaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsApiResponseCommon`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsApiResponseCommon {
    pub errors: DnsRecordsMessages,
    pub messages: DnsRecordsMessages,
    ///Whether the API call was successful.
    pub success: bool,
}
///`DnsRecordsApiResponseCommonFailure`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsApiResponseCommonFailure {
    pub errors: ::std::vec::Vec<DnsRecordsApiResponseCommonFailureErrorsItem>,
    pub messages: DnsRecordsMessages,
    pub result: (),
    ///Whether the API call was successful.
    pub success: bool,
}
///`DnsRecordsApiResponseCommonFailureErrorsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsApiResponseCommonFailureErrorsItem {
    pub code: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub documentation_url: ::std::option::Option<::std::string::String>,
    pub message: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<
        DnsRecordsApiResponseCommonFailureErrorsItemSource,
    >,
}
///`DnsRecordsApiResponseCommonFailureErrorsItemSource`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsApiResponseCommonFailureErrorsItemSource {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pointer: ::std::option::Option<::std::string::String>,
}
///`DnsRecordsApiResponseSingle`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DnsRecordsApiResponseSingle(pub DnsRecordsApiResponseCommon);
impl ::std::ops::Deref for DnsRecordsApiResponseSingle {
    type Target = DnsRecordsApiResponseCommon;
    fn deref(&self) -> &DnsRecordsApiResponseCommon {
        &self.0
    }
}
impl ::std::convert::From<DnsRecordsApiResponseSingle> for DnsRecordsApiResponseCommon {
    fn from(value: DnsRecordsApiResponseSingle) -> Self {
        value.0
    }
}
impl ::std::convert::From<DnsRecordsApiResponseCommon> for DnsRecordsApiResponseSingle {
    fn from(value: DnsRecordsApiResponseCommon) -> Self {
        Self(value)
    }
}
///`DnsRecordsCaaRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsCaaRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted CAA content. See 'data' to set CAA properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsCaaRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsCaaRecordType>,
}
///Components of a CAA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsCaaRecordData {
    ///Flags for the CAA record.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<f64>,
    ///Name of the property controlled by this record (e.g.: issue, issuewild, iodef).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tag: ::std::option::Option<::std::string::String>,
    ///Value of the record. This field's semantics depend on the chosen tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Record type.
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
pub enum DnsRecordsCaaRecordType {
    #[serde(rename = "CAA")]
    Caa,
}
impl ::std::fmt::Display for DnsRecordsCaaRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Caa => f.write_str("CAA"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsCaaRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CAA" => Ok(Self::Caa),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsCaaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsCaaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsCertRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsCertRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted CERT content. See 'data' to set CERT properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsCertRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsCertRecordType>,
}
///Components of a CERT record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsCertRecordData {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Key Tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub key_tag: ::std::option::Option<f64>,
    ///Type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordsCertRecordType {
    #[serde(rename = "CERT")]
    Cert,
}
impl ::std::fmt::Display for DnsRecordsCertRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cert => f.write_str("CERT"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsCertRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CERT" => Ok(Self::Cert),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsCertRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsCertRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsCnameRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsCnameRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A valid hostname. Must not match the record's name.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsCnameRecordSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsCnameRecordType>,
}
///`DnsRecordsCnameRecordSettings`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsCnameRecordSettings {
    ///If enabled, causes the CNAME record to be resolved externally and the resulting address records (e.g., A and AAAA) to be returned instead of the CNAME record itself. This setting is unavailable for proxied records, since they are always flattened.
    #[serde(default)]
    pub flatten_cname: bool,
    ///When enabled, only A records will be generated, and AAAA records will not be created. This setting is intended for exceptional cases. Note that this option only applies to proxied records and it has no effect on whether Cloudflare communicates with the origin using IPv4 or IPv6.
    #[serde(default)]
    pub ipv4_only: bool,
    ///When enabled, only AAAA records will be generated, and A records will not be created. This setting is intended for exceptional cases. Note that this option only applies to proxied records and it has no effect on whether Cloudflare communicates with the origin using IPv4 or IPv6.
    #[serde(default)]
    pub ipv6_only: bool,
}
///Record type.
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
pub enum DnsRecordsCnameRecordType {
    #[serde(rename = "CNAME")]
    Cname,
}
impl ::std::fmt::Display for DnsRecordsCnameRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cname => f.write_str("CNAME"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsCnameRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CNAME" => Ok(Self::Cname),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsCnameRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsCnameRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Comments or notes about the DNS record. This field has no effect on DNS responses.
#[derive(
    ::serde::Deserialize,
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
pub struct DnsRecordsComment(pub ::std::string::String);
impl ::std::ops::Deref for DnsRecordsComment {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DnsRecordsComment> for ::std::string::String {
    fn from(value: DnsRecordsComment) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for DnsRecordsComment {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for DnsRecordsComment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for DnsRecordsComment {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`DnsRecordsDnsRecordPatch`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordPatch {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<DnsRecordsDnsRecordWithoutData>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<DnsRecordsDnsRecordWithData>,
}
///`DnsRecordsDnsRecordPost`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordPost {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<DnsRecordsDnsRecordWithoutData>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<DnsRecordsDnsRecordWithData>,
}
///`DnsRecordsDnsRecordResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum DnsRecordsDnsRecordResponse {
    Variant0(DnsRecordsDnsRecordResponseVariant0),
    Variant1(DnsRecordsDnsRecordResponseVariant1),
}
impl ::std::convert::From<DnsRecordsDnsRecordResponseVariant0>
for DnsRecordsDnsRecordResponse {
    fn from(value: DnsRecordsDnsRecordResponseVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<DnsRecordsDnsRecordResponseVariant1>
for DnsRecordsDnsRecordResponse {
    fn from(value: DnsRecordsDnsRecordResponseVariant1) -> Self {
        Self::Variant1(value)
    }
}
///`DnsRecordsDnsRecordResponseVariant0`
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
#[serde(deny_unknown_fields)]
pub enum DnsRecordsDnsRecordResponseVariant0 {}
///`DnsRecordsDnsRecordResponseVariant1`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum DnsRecordsDnsRecordResponseVariant1 {
    Variant0 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted CAA content. See 'data' to set CAA properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant0Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant0Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant0Type,
    },
    Variant1 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted CERT content. See 'data' to set CERT properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant1Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant1Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant1Type,
    },
    Variant2 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted DNSKEY content. See 'data' to set DNSKEY properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant2Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant2Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant2Type,
    },
    Variant3 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted DS content. See 'data' to set DS properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant3Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant3Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant3Type,
    },
    Variant4 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted HTTPS content. See 'data' to set HTTPS properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant4Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant4Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant4Type,
    },
    Variant5 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted LOC content. See 'data' to set LOC properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant5Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant5Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant5Type,
    },
    Variant6 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted NAPTR content. See 'data' to set NAPTR properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant6Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant6Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant6Type,
    },
    Variant7 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted SMIMEA content. See 'data' to set SMIMEA properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant7Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant7Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant7Type,
    },
    Variant8 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Priority, weight, port, and SRV target. See 'data' for setting the individual component values.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant8Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant8Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant8Type,
    },
    Variant9 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted SSHFP content. See 'data' to set SSHFP properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant9Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant9Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant9Type,
    },
    Variant10 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted SVCB content. See 'data' to set SVCB properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant10Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant10Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant10Type,
    },
    Variant11 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted TLSA content. See 'data' to set TLSA properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant11Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant11Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant11Type,
    },
    Variant12 {
        comment: DnsRecordsComment,
        ///When the record comment was last modified. Omitted if there is no comment.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        comment_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ///Formatted URI content. See 'data' to set URI properties.
        content: ::std::string::String,
        ///When the record was created.
        created_on: ::chrono::DateTime<::chrono::offset::Utc>,
        data: DnsRecordsDnsRecordResponseVariant1Variant12Data,
        id: DnsRecordsIdentifier,
        meta: DnsRecordsDnsRecordResponseVariant1Variant12Meta,
        ///When the record was last modified.
        modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
        name: DnsRecordsName,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        priority: ::std::option::Option<DnsRecordsPriority>,
        ///Whether the record can be proxied by Cloudflare or not.
        proxiable: bool,
        proxied: DnsRecordsProxied,
        settings: DnsRecordsSettings,
        tags: DnsRecordsTags,
        ///When the record tags were last modified. Omitted if there are no tags.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        tags_modified_on: ::std::option::Option<
            ::chrono::DateTime<::chrono::offset::Utc>,
        >,
        ttl: DnsRecordsTtl,
        ///Record type.
        #[serde(rename = "type")]
        type_: DnsRecordsDnsRecordResponseVariant1Variant12Type,
    },
}
///Components of a CAA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant0Data {
    ///Flags for the CAA record.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<f64>,
    ///Name of the property controlled by this record (e.g.: issue, issuewild, iodef).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tag: ::std::option::Option<::std::string::String>,
    ///Value of the record. This field's semantics depend on the chosen tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant0Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant0Type {
    #[serde(rename = "CAA")]
    Caa,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant0Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Caa => f.write_str("CAA"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant0Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CAA" => Ok(Self::Caa),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant0Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant0Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a SVCB record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant10Data {
    ///Priority.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
    ///Target.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///Value.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant10Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant10Type {
    #[serde(rename = "SVCB")]
    Svcb,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant10Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Svcb => f.write_str("SVCB"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant10Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SVCB" => Ok(Self::Svcb),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant10Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant10Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a TLSA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant11Data {
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Matching Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub matching_type: ::std::option::Option<f64>,
    ///Selector.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub selector: ::std::option::Option<f64>,
    ///Usage.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<f64>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant11Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant11Type {
    #[serde(rename = "TLSA")]
    Tlsa,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant11Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Tlsa => f.write_str("TLSA"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant11Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "TLSA" => Ok(Self::Tlsa),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant11Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant11Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a URI record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant12Data {
    ///The record content.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///The record weight.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub weight: ::std::option::Option<f64>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant12Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant12Type {
    #[serde(rename = "URI")]
    Uri,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant12Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Uri => f.write_str("URI"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant12Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "URI" => Ok(Self::Uri),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant12Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant12Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a CERT record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant1Data {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Key Tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub key_tag: ::std::option::Option<f64>,
    ///Type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<f64>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant1Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant1Type {
    #[serde(rename = "CERT")]
    Cert,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant1Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cert => f.write_str("CERT"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant1Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CERT" => Ok(Self::Cert),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a DNSKEY record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant2Data {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Flags.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<f64>,
    ///Protocol.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<f64>,
    ///Public Key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub public_key: ::std::option::Option<::std::string::String>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant2Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant2Type {
    #[serde(rename = "DNSKEY")]
    Dnskey,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant2Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dnskey => f.write_str("DNSKEY"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant2Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "DNSKEY" => Ok(Self::Dnskey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant2Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant2Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a DS record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant3Data {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Digest.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub digest: ::std::option::Option<::std::string::String>,
    ///Digest Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub digest_type: ::std::option::Option<f64>,
    ///Key Tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub key_tag: ::std::option::Option<f64>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant3Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant3Type {
    #[serde(rename = "DS")]
    Ds,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant3Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ds => f.write_str("DS"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant3Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "DS" => Ok(Self::Ds),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant3Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant3Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a HTTPS record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant4Data {
    ///Priority.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
    ///Target.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///Value.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant4Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant4Type {
    #[serde(rename = "HTTPS")]
    Https,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant4Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Https => f.write_str("HTTPS"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant4Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "HTTPS" => Ok(Self::Https),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant4Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant4Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a LOC record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant5Data {
    ///Altitude of location in meters.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub altitude: ::std::option::Option<f64>,
    ///Degrees of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_degrees: ::std::option::Option<f64>,
    ///Latitude direction.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_direction: ::std::option::Option<
        DnsRecordsDnsRecordResponseVariant1Variant5DataLatDirection,
    >,
    ///Minutes of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_minutes: ::std::option::Option<f64>,
    ///Seconds of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_seconds: ::std::option::Option<f64>,
    ///Degrees of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_degrees: ::std::option::Option<f64>,
    ///Longitude direction.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_direction: ::std::option::Option<
        DnsRecordsDnsRecordResponseVariant1Variant5DataLongDirection,
    >,
    ///Minutes of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_minutes: ::std::option::Option<f64>,
    ///Seconds of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_seconds: ::std::option::Option<f64>,
    ///Horizontal precision of location.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub precision_horz: ::std::option::Option<f64>,
    ///Vertical precision of location.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub precision_vert: ::std::option::Option<f64>,
    ///Size of location in meters.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<f64>,
}
///Latitude direction.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant5DataLatDirection {
    N,
    S,
}
impl ::std::fmt::Display
for DnsRecordsDnsRecordResponseVariant1Variant5DataLatDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::N => f.write_str("N"),
            Self::S => f.write_str("S"),
        }
    }
}
impl ::std::str::FromStr
for DnsRecordsDnsRecordResponseVariant1Variant5DataLatDirection {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "N" => Ok(Self::N),
            "S" => Ok(Self::S),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for DnsRecordsDnsRecordResponseVariant1Variant5DataLatDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant5DataLatDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Longitude direction.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant5DataLongDirection {
    E,
    W,
}
impl ::std::fmt::Display
for DnsRecordsDnsRecordResponseVariant1Variant5DataLongDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::E => f.write_str("E"),
            Self::W => f.write_str("W"),
        }
    }
}
impl ::std::str::FromStr
for DnsRecordsDnsRecordResponseVariant1Variant5DataLongDirection {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "E" => Ok(Self::E),
            "W" => Ok(Self::W),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for DnsRecordsDnsRecordResponseVariant1Variant5DataLongDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant5DataLongDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant5Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant5Type {
    #[serde(rename = "LOC")]
    Loc,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant5Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Loc => f.write_str("LOC"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant5Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "LOC" => Ok(Self::Loc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant5Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant5Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a NAPTR record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant6Data {
    ///Flags.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<::std::string::String>,
    ///Order.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<f64>,
    ///Preference.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub preference: ::std::option::Option<f64>,
    ///Regex.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub regex: ::std::option::Option<::std::string::String>,
    ///Replacement.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub replacement: ::std::option::Option<::std::string::String>,
    ///Service.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service: ::std::option::Option<::std::string::String>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant6Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant6Type {
    #[serde(rename = "NAPTR")]
    Naptr,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant6Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Naptr => f.write_str("NAPTR"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant6Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NAPTR" => Ok(Self::Naptr),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant6Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant6Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a SMIMEA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant7Data {
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Matching Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub matching_type: ::std::option::Option<f64>,
    ///Selector.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub selector: ::std::option::Option<f64>,
    ///Usage.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<f64>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant7Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant7Type {
    #[serde(rename = "SMIMEA")]
    Smimea,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant7Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Smimea => f.write_str("SMIMEA"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant7Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SMIMEA" => Ok(Self::Smimea),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant7Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant7Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a SRV record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant8Data {
    ///The port of the service.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub port: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<DnsRecordsPriority>,
    ///A valid hostname.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///The record weight.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub weight: ::std::option::Option<f64>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant8Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant8Type {
    #[serde(rename = "SRV")]
    Srv,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant8Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Srv => f.write_str("SRV"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant8Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SRV" => Ok(Self::Srv),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant8Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant8Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Components of a SSHFP record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant9Data {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Fingerprint.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    ///Type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<f64>,
}
///Extra Cloudflare-specific metadata about the record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordResponseVariant1Variant9Meta {
    /**Whether this glue record is not served because a shallower NS delegation takes precedence over the deeper delegation that needs it. Present only when true; reachable glue carries only `is_glue`. See [Unreachable glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#unreachable-glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dead_glue: ::std::option::Option<bool>,
    /**Whether this A or AAAA record is glue for a subdomain NS delegation. See [Glue records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records#glue-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_glue: ::std::option::Option<bool>,
    /**IDs of the NS records that shadow this record. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shadowed_by: ::std::vec::Vec<::std::string::String>,
    /**Number of records shadowed by this NS delegation. See [Shadowed records](https://developers.cloudflare.com/dns/manage-dns-records/reference/shadowed-records).
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shadowed_records_count: ::std::option::Option<i64>,
}
///Record type.
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
pub enum DnsRecordsDnsRecordResponseVariant1Variant9Type {
    #[serde(rename = "SSHFP")]
    Sshfp,
}
impl ::std::fmt::Display for DnsRecordsDnsRecordResponseVariant1Variant9Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sshfp => f.write_str("SSHFP"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnsRecordResponseVariant1Variant9Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SSHFP" => Ok(Self::Sshfp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnsRecordResponseVariant1Variant9Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsDnsRecordResponseVariant1Variant9Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsDnsRecordSharedFields`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsRecordSharedFields {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
}
///`DnsRecordsDnsRecordWithData`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum DnsRecordsDnsRecordWithData {
    CaaRecord(DnsRecordsCaaRecord),
    CertRecord(DnsRecordsCertRecord),
    DnskeyRecord(DnsRecordsDnskeyRecord),
    DsRecord(DnsRecordsDsRecord),
    HttpsRecord(DnsRecordsHttpsRecord),
    LocRecord(DnsRecordsLocRecord),
    NaptrRecord(DnsRecordsNaptrRecord),
    SmimeaRecord(DnsRecordsSmimeaRecord),
    SrvRecord(DnsRecordsSrvRecord),
    SshfpRecord(DnsRecordsSshfpRecord),
    SvcbRecord(DnsRecordsSvcbRecord),
    TlsaRecord(DnsRecordsTlsaRecord),
    UriRecord(DnsRecordsUriRecord),
}
impl ::std::convert::From<DnsRecordsCaaRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsCaaRecord) -> Self {
        Self::CaaRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsCertRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsCertRecord) -> Self {
        Self::CertRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsDnskeyRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsDnskeyRecord) -> Self {
        Self::DnskeyRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsDsRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsDsRecord) -> Self {
        Self::DsRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsHttpsRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsHttpsRecord) -> Self {
        Self::HttpsRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsLocRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsLocRecord) -> Self {
        Self::LocRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsNaptrRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsNaptrRecord) -> Self {
        Self::NaptrRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsSmimeaRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsSmimeaRecord) -> Self {
        Self::SmimeaRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsSrvRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsSrvRecord) -> Self {
        Self::SrvRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsSshfpRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsSshfpRecord) -> Self {
        Self::SshfpRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsSvcbRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsSvcbRecord) -> Self {
        Self::SvcbRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsTlsaRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsTlsaRecord) -> Self {
        Self::TlsaRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsUriRecord> for DnsRecordsDnsRecordWithData {
    fn from(value: DnsRecordsUriRecord) -> Self {
        Self::UriRecord(value)
    }
}
///`DnsRecordsDnsRecordWithoutData`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum DnsRecordsDnsRecordWithoutData {
    ARecord(DnsRecordsARecord),
    AaaaRecord(DnsRecordsAaaaRecord),
    CnameRecord(DnsRecordsCnameRecord),
    MxRecord(DnsRecordsMxRecord),
    NsRecord(DnsRecordsNsRecord),
    OpenpgpkeyRecord(DnsRecordsOpenpgpkeyRecord),
    PtrRecord(DnsRecordsPtrRecord),
    TxtRecord(DnsRecordsTxtRecord),
}
impl ::std::convert::From<DnsRecordsARecord> for DnsRecordsDnsRecordWithoutData {
    fn from(value: DnsRecordsARecord) -> Self {
        Self::ARecord(value)
    }
}
impl ::std::convert::From<DnsRecordsAaaaRecord> for DnsRecordsDnsRecordWithoutData {
    fn from(value: DnsRecordsAaaaRecord) -> Self {
        Self::AaaaRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsCnameRecord> for DnsRecordsDnsRecordWithoutData {
    fn from(value: DnsRecordsCnameRecord) -> Self {
        Self::CnameRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsMxRecord> for DnsRecordsDnsRecordWithoutData {
    fn from(value: DnsRecordsMxRecord) -> Self {
        Self::MxRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsNsRecord> for DnsRecordsDnsRecordWithoutData {
    fn from(value: DnsRecordsNsRecord) -> Self {
        Self::NsRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsOpenpgpkeyRecord>
for DnsRecordsDnsRecordWithoutData {
    fn from(value: DnsRecordsOpenpgpkeyRecord) -> Self {
        Self::OpenpgpkeyRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsPtrRecord> for DnsRecordsDnsRecordWithoutData {
    fn from(value: DnsRecordsPtrRecord) -> Self {
        Self::PtrRecord(value)
    }
}
impl ::std::convert::From<DnsRecordsTxtRecord> for DnsRecordsDnsRecordWithoutData {
    fn from(value: DnsRecordsTxtRecord) -> Self {
        Self::TxtRecord(value)
    }
}
///`DnsRecordsDnsResponseSingle`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnsResponseSingle {
    pub errors: DnsRecordsMessages,
    pub messages: DnsRecordsMessages,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<DnsRecordsDnsRecordResponse>,
    ///Whether the API call was successful.
    pub success: bool,
}
///`DnsRecordsDnskeyRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnskeyRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted DNSKEY content. See 'data' to set DNSKEY properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsDnskeyRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsDnskeyRecordType>,
}
///Components of a DNSKEY record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDnskeyRecordData {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Flags.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<f64>,
    ///Protocol.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<f64>,
    ///Public Key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub public_key: ::std::option::Option<::std::string::String>,
}
///Record type.
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
pub enum DnsRecordsDnskeyRecordType {
    #[serde(rename = "DNSKEY")]
    Dnskey,
}
impl ::std::fmt::Display for DnsRecordsDnskeyRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dnskey => f.write_str("DNSKEY"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDnskeyRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "DNSKEY" => Ok(Self::Dnskey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDnskeyRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsDnskeyRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsDsRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDsRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted DS content. See 'data' to set DS properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsDsRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsDsRecordType>,
}
///Components of a DS record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsDsRecordData {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Digest.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub digest: ::std::option::Option<::std::string::String>,
    ///Digest Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub digest_type: ::std::option::Option<f64>,
    ///Key Tag.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub key_tag: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordsDsRecordType {
    #[serde(rename = "DS")]
    Ds,
}
impl ::std::fmt::Display for DnsRecordsDsRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ds => f.write_str("DS"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsDsRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "DS" => Ok(Self::Ds),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsDsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsDsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsHttpsRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsHttpsRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted HTTPS content. See 'data' to set HTTPS properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsHttpsRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsHttpsRecordType>,
}
///Components of a HTTPS record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsHttpsRecordData {
    ///Priority.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
    ///Target.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///Value.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Record type.
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
pub enum DnsRecordsHttpsRecordType {
    #[serde(rename = "HTTPS")]
    Https,
}
impl ::std::fmt::Display for DnsRecordsHttpsRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Https => f.write_str("HTTPS"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsHttpsRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "HTTPS" => Ok(Self::Https),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsHttpsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsHttpsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Identifier.
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
pub struct DnsRecordsIdentifier(::std::string::String);
impl ::std::ops::Deref for DnsRecordsIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DnsRecordsIdentifier> for ::std::string::String {
    fn from(value: DnsRecordsIdentifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DnsRecordsIdentifier {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DnsRecordsIdentifier {
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
///`DnsRecordsLocRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsLocRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted LOC content. See 'data' to set LOC properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsLocRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsLocRecordType>,
}
///Components of a LOC record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsLocRecordData {
    ///Altitude of location in meters.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub altitude: ::std::option::Option<f64>,
    ///Degrees of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_degrees: ::std::option::Option<f64>,
    ///Latitude direction.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_direction: ::std::option::Option<DnsRecordsLocRecordDataLatDirection>,
    ///Minutes of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_minutes: ::std::option::Option<f64>,
    ///Seconds of latitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lat_seconds: ::std::option::Option<f64>,
    ///Degrees of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_degrees: ::std::option::Option<f64>,
    ///Longitude direction.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_direction: ::std::option::Option<DnsRecordsLocRecordDataLongDirection>,
    ///Minutes of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_minutes: ::std::option::Option<f64>,
    ///Seconds of longitude.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub long_seconds: ::std::option::Option<f64>,
    ///Horizontal precision of location.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub precision_horz: ::std::option::Option<f64>,
    ///Vertical precision of location.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub precision_vert: ::std::option::Option<f64>,
    ///Size of location in meters.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<f64>,
}
///Latitude direction.
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
pub enum DnsRecordsLocRecordDataLatDirection {
    N,
    S,
}
impl ::std::fmt::Display for DnsRecordsLocRecordDataLatDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::N => f.write_str("N"),
            Self::S => f.write_str("S"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsLocRecordDataLatDirection {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "N" => Ok(Self::N),
            "S" => Ok(Self::S),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsLocRecordDataLatDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsLocRecordDataLatDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Longitude direction.
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
pub enum DnsRecordsLocRecordDataLongDirection {
    E,
    W,
}
impl ::std::fmt::Display for DnsRecordsLocRecordDataLongDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::E => f.write_str("E"),
            Self::W => f.write_str("W"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsLocRecordDataLongDirection {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "E" => Ok(Self::E),
            "W" => Ok(Self::W),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsLocRecordDataLongDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DnsRecordsLocRecordDataLongDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Record type.
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
pub enum DnsRecordsLocRecordType {
    #[serde(rename = "LOC")]
    Loc,
}
impl ::std::fmt::Display for DnsRecordsLocRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Loc => f.write_str("LOC"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsLocRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "LOC" => Ok(Self::Loc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsLocRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsLocRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsMessages`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DnsRecordsMessages(pub ::std::vec::Vec<DnsRecordsMessagesItem>);
impl ::std::ops::Deref for DnsRecordsMessages {
    type Target = ::std::vec::Vec<DnsRecordsMessagesItem>;
    fn deref(&self) -> &::std::vec::Vec<DnsRecordsMessagesItem> {
        &self.0
    }
}
impl ::std::convert::From<DnsRecordsMessages>
for ::std::vec::Vec<DnsRecordsMessagesItem> {
    fn from(value: DnsRecordsMessages) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<DnsRecordsMessagesItem>>
for DnsRecordsMessages {
    fn from(value: ::std::vec::Vec<DnsRecordsMessagesItem>) -> Self {
        Self(value)
    }
}
///`DnsRecordsMessagesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsMessagesItem {
    pub code: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub documentation_url: ::std::option::Option<::std::string::String>,
    pub message: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<DnsRecordsMessagesItemSource>,
}
///`DnsRecordsMessagesItemSource`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsMessagesItemSource {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pointer: ::std::option::Option<::std::string::String>,
}
///`DnsRecordsMxRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsMxRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A valid mail server hostname.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<DnsRecordsPriority>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsMxRecordType>,
}
///Record type.
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
pub enum DnsRecordsMxRecordType {
    #[serde(rename = "MX")]
    Mx,
}
impl ::std::fmt::Display for DnsRecordsMxRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mx => f.write_str("MX"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsMxRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "MX" => Ok(Self::Mx),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsMxRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsMxRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Complete DNS record name, including the zone name, in Punycode.
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
pub struct DnsRecordsName(::std::string::String);
impl ::std::ops::Deref for DnsRecordsName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DnsRecordsName> for ::std::string::String {
    fn from(value: DnsRecordsName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DnsRecordsName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DnsRecordsName {
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
///`DnsRecordsNaptrRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsNaptrRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted NAPTR content. See 'data' to set NAPTR properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsNaptrRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsNaptrRecordType>,
}
///Components of a NAPTR record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsNaptrRecordData {
    ///Flags.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub flags: ::std::option::Option<::std::string::String>,
    ///Order.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<f64>,
    ///Preference.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub preference: ::std::option::Option<f64>,
    ///Regex.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub regex: ::std::option::Option<::std::string::String>,
    ///Replacement.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub replacement: ::std::option::Option<::std::string::String>,
    ///Service.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service: ::std::option::Option<::std::string::String>,
}
///Record type.
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
pub enum DnsRecordsNaptrRecordType {
    #[serde(rename = "NAPTR")]
    Naptr,
}
impl ::std::fmt::Display for DnsRecordsNaptrRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Naptr => f.write_str("NAPTR"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsNaptrRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NAPTR" => Ok(Self::Naptr),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsNaptrRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsNaptrRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsNsRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsNsRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A valid name server host name.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsNsRecordType>,
}
///Record type.
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
pub enum DnsRecordsNsRecordType {
    #[serde(rename = "NS")]
    Ns,
}
impl ::std::fmt::Display for DnsRecordsNsRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ns => f.write_str("NS"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsNsRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NS" => Ok(Self::Ns),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsNsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsNsRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsOpenpgpkeyRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsOpenpgpkeyRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///A single Base64-encoded OpenPGP Transferable Public Key (RFC 4880 Section 11.1)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsOpenpgpkeyRecordType>,
}
///Record type.
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
pub enum DnsRecordsOpenpgpkeyRecordType {
    #[serde(rename = "OPENPGPKEY")]
    Openpgpkey,
}
impl ::std::fmt::Display for DnsRecordsOpenpgpkeyRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Openpgpkey => f.write_str("OPENPGPKEY"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsOpenpgpkeyRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "OPENPGPKEY" => Ok(Self::Openpgpkey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsOpenpgpkeyRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsOpenpgpkeyRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Required for MX and URI records; ignored for other record types (but may still be returned by the API). Records with lower priorities are preferred. This field is to be deprecated in favor of the priority field within the data map.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DnsRecordsPriority(pub u16);
impl ::std::ops::Deref for DnsRecordsPriority {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}
impl ::std::convert::From<DnsRecordsPriority> for u16 {
    fn from(value: DnsRecordsPriority) -> Self {
        value.0
    }
}
impl ::std::convert::From<u16> for DnsRecordsPriority {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for DnsRecordsPriority {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for DnsRecordsPriority {
    type Err = <u16 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsPriority {
    type Error = <u16 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for DnsRecordsPriority {
    type Error = <u16 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///Whether the record is receiving the performance and security benefits of Cloudflare.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DnsRecordsProxied(pub bool);
impl ::std::ops::Deref for DnsRecordsProxied {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<DnsRecordsProxied> for bool {
    fn from(value: DnsRecordsProxied) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for DnsRecordsProxied {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for DnsRecordsProxied {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for DnsRecordsProxied {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsProxied {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for DnsRecordsProxied {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///`DnsRecordsPtrRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsPtrRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Domain name pointing to the address.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsPtrRecordType>,
}
///Record type.
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
pub enum DnsRecordsPtrRecordType {
    #[serde(rename = "PTR")]
    Ptr,
}
impl ::std::fmt::Display for DnsRecordsPtrRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ptr => f.write_str("PTR"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsPtrRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "PTR" => Ok(Self::Ptr),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsPtrRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsPtrRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Settings for the DNS record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsSettings {
    ///When enabled, only A records will be generated, and AAAA records will not be created. This setting is intended for exceptional cases. Note that this option only applies to proxied records and it has no effect on whether Cloudflare communicates with the origin using IPv4 or IPv6.
    #[serde(default)]
    pub ipv4_only: bool,
    ///When enabled, only AAAA records will be generated, and A records will not be created. This setting is intended for exceptional cases. Note that this option only applies to proxied records and it has no effect on whether Cloudflare communicates with the origin using IPv4 or IPv6.
    #[serde(default)]
    pub ipv6_only: bool,
}
///`DnsRecordsSmimeaRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsSmimeaRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted SMIMEA content. See 'data' to set SMIMEA properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsSmimeaRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsSmimeaRecordType>,
}
///Components of a SMIMEA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsSmimeaRecordData {
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Matching Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub matching_type: ::std::option::Option<f64>,
    ///Selector.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub selector: ::std::option::Option<f64>,
    ///Usage.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordsSmimeaRecordType {
    #[serde(rename = "SMIMEA")]
    Smimea,
}
impl ::std::fmt::Display for DnsRecordsSmimeaRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Smimea => f.write_str("SMIMEA"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsSmimeaRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SMIMEA" => Ok(Self::Smimea),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsSmimeaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsSmimeaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsSrvRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsSrvRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Priority, weight, port, and SRV target. See 'data' for setting the individual component values.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsSrvRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsSrvRecordType>,
}
///Components of a SRV record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsSrvRecordData {
    ///The port of the service.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub port: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<DnsRecordsPriority>,
    ///A valid hostname.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///The record weight.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub weight: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordsSrvRecordType {
    #[serde(rename = "SRV")]
    Srv,
}
impl ::std::fmt::Display for DnsRecordsSrvRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Srv => f.write_str("SRV"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsSrvRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SRV" => Ok(Self::Srv),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsSrvRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsSrvRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsSshfpRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsSshfpRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted SSHFP content. See 'data' to set SSHFP properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsSshfpRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsSshfpRecordType>,
}
///Components of a SSHFP record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsSshfpRecordData {
    ///Algorithm.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub algorithm: ::std::option::Option<f64>,
    ///Fingerprint.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    ///Type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordsSshfpRecordType {
    #[serde(rename = "SSHFP")]
    Sshfp,
}
impl ::std::fmt::Display for DnsRecordsSshfpRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sshfp => f.write_str("SSHFP"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsSshfpRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SSHFP" => Ok(Self::Sshfp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsSshfpRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsSshfpRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsSvcbRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsSvcbRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted SVCB content. See 'data' to set SVCB properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsSvcbRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsSvcbRecordType>,
}
///Components of a SVCB record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsSvcbRecordData {
    ///Priority.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
    ///Target.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///Value.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///Record type.
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
pub enum DnsRecordsSvcbRecordType {
    #[serde(rename = "SVCB")]
    Svcb,
}
impl ::std::fmt::Display for DnsRecordsSvcbRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Svcb => f.write_str("SVCB"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsSvcbRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SVCB" => Ok(Self::Svcb),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsSvcbRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsSvcbRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Custom tags for the DNS record. This field has no effect on DNS responses.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DnsRecordsTags(pub ::std::vec::Vec<::std::string::String>);
impl ::std::ops::Deref for DnsRecordsTags {
    type Target = ::std::vec::Vec<::std::string::String>;
    fn deref(&self) -> &::std::vec::Vec<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<DnsRecordsTags> for ::std::vec::Vec<::std::string::String> {
    fn from(value: DnsRecordsTags) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for DnsRecordsTags {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self(value)
    }
}
///`DnsRecordsTlsaRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsTlsaRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted TLSA content. See 'data' to set TLSA properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsTlsaRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsTlsaRecordType>,
}
///Components of a TLSA record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsTlsaRecordData {
    ///Certificate.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub certificate: ::std::option::Option<::std::string::String>,
    ///Matching Type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub matching_type: ::std::option::Option<f64>,
    ///Selector.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub selector: ::std::option::Option<f64>,
    ///Usage.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordsTlsaRecordType {
    #[serde(rename = "TLSA")]
    Tlsa,
}
impl ::std::fmt::Display for DnsRecordsTlsaRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Tlsa => f.write_str("TLSA"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsTlsaRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "TLSA" => Ok(Self::Tlsa),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsTlsaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsTlsaRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'. Value must be between 60 and 86400, with the minimum reduced to 30 for Enterprise zones.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct DnsRecordsTtl(pub ::std::num::NonZeroU64);
impl ::std::ops::Deref for DnsRecordsTtl {
    type Target = ::std::num::NonZeroU64;
    fn deref(&self) -> &::std::num::NonZeroU64 {
        &self.0
    }
}
impl ::std::convert::From<DnsRecordsTtl> for ::std::num::NonZeroU64 {
    fn from(value: DnsRecordsTtl) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::num::NonZeroU64> for DnsRecordsTtl {
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for DnsRecordsTtl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for DnsRecordsTtl {
    type Err = <::std::num::NonZeroU64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsTtl {
    type Error = <::std::num::NonZeroU64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for DnsRecordsTtl {
    type Error = <::std::num::NonZeroU64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///`DnsRecordsTxtRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsTxtRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    /**Text content for the record. The content must consist of quoted "character strings" (RFC 1035), each with a length of up to 255 bytes. Strings exceeding this allowed maximum length are automatically split.

Learn more at <https://www.cloudflare.com/learning/dns/dns-records/dns-txt-record/>.*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsTxtRecordType>,
}
///Record type.
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
pub enum DnsRecordsTxtRecordType {
    #[serde(rename = "TXT")]
    Txt,
}
impl ::std::fmt::Display for DnsRecordsTxtRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Txt => f.write_str("TXT"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsTxtRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "TXT" => Ok(Self::Txt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsTxtRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsTxtRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DnsRecordsUriRecord`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsUriRecord {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: ::std::option::Option<DnsRecordsComment>,
    ///Formatted URI content. See 'data' to set URI properties.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DnsRecordsUriRecordData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DnsRecordsName>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<DnsRecordsPriority>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proxied: ::std::option::Option<DnsRecordsProxied>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<DnsRecordsSettings>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<DnsRecordsTags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<DnsRecordsTtl>,
    ///Record type.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<DnsRecordsUriRecordType>,
}
///Components of a URI record.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct DnsRecordsUriRecordData {
    ///The record content.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<::std::string::String>,
    ///The record weight.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub weight: ::std::option::Option<f64>,
}
///Record type.
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
pub enum DnsRecordsUriRecordType {
    #[serde(rename = "URI")]
    Uri,
}
impl ::std::fmt::Display for DnsRecordsUriRecordType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Uri => f.write_str("URI"),
        }
    }
}
impl ::std::str::FromStr for DnsRecordsUriRecordType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "URI" => Ok(Self::Uri),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnsRecordsUriRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnsRecordsUriRecordType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Which query string parameters to exclude from the cache key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct Exclude {
    ///Whether to exclude all query string parameters from the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub all: ::std::option::Option<bool>,
    ///A list of query string parameters to exclude from the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub list: ::std::option::Option<::std::vec::Vec<ParameterName>>,
}
///The expression defining which traffic will match the rule.
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
pub struct Expression(::std::string::String);
impl ::std::ops::Deref for Expression {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Expression> for ::std::string::String {
    fn from(value: Expression) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Expression {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Expression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Expression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Expression {
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
///The name of the header to check for the presence of.
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
pub struct Header(::std::string::String);
impl ::std::ops::Deref for Header {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Header> for ::std::string::String {
    fn from(value: Header) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Header {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Header {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Header {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Header {
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
///The name of the response header.
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
pub struct HeaderName(::std::string::String);
impl ::std::ops::Deref for HeaderName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<HeaderName> for ::std::string::String {
    fn from(value: HeaderName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for HeaderName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for HeaderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HeaderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for HeaderName {
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
///The header value to match against.
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
pub struct HeaderValue(::std::string::String);
impl ::std::ops::Deref for HeaderValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<HeaderValue> for ::std::string::String {
    fn from(value: HeaderValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for HeaderValue {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for HeaderValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HeaderValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for HeaderValue {
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
///A resolved host to route to.
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
pub struct Host(::std::string::String);
impl ::std::ops::Deref for Host {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Host> for ::std::string::String {
    fn from(value: Host) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Host {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Host {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Host {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Host {
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
///`IamAccount`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamAccount {
    ///Timestamp for the creation of the account
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_on: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub id: IamCommonComponentsSchemasIdentifier,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub managed_by: ::std::option::Option<IamAccountManagedBy>,
    ///Account name
    pub name: IamAccountName,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<IamAccountSettings>,
    #[serde(rename = "type")]
    pub type_: IamAccountType,
}
///Account identifier tag.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct IamAccountIdentifier(pub IamCommonComponentsSchemasIdentifier);
impl ::std::ops::Deref for IamAccountIdentifier {
    type Target = IamCommonComponentsSchemasIdentifier;
    fn deref(&self) -> &IamCommonComponentsSchemasIdentifier {
        &self.0
    }
}
impl ::std::convert::From<IamAccountIdentifier>
for IamCommonComponentsSchemasIdentifier {
    fn from(value: IamAccountIdentifier) -> Self {
        value.0
    }
}
impl ::std::convert::From<IamCommonComponentsSchemasIdentifier>
for IamAccountIdentifier {
    fn from(value: IamCommonComponentsSchemasIdentifier) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for IamAccountIdentifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for IamAccountIdentifier {
    type Err = <IamCommonComponentsSchemasIdentifier as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for IamAccountIdentifier {
    type Error = <IamCommonComponentsSchemasIdentifier as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for IamAccountIdentifier {
    type Error = <IamCommonComponentsSchemasIdentifier as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///Parent container details
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamAccountManagedBy {
    ///ID of the parent Organization, if one exists
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_org_id: ::std::option::Option<IamAccountManagedByParentOrgId>,
    ///Name of the parent Organization, if one exists
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_org_name: ::std::option::Option<::std::string::String>,
}
///ID of the parent Organization, if one exists
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
pub struct IamAccountManagedByParentOrgId(::std::string::String);
impl ::std::ops::Deref for IamAccountManagedByParentOrgId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IamAccountManagedByParentOrgId> for ::std::string::String {
    fn from(value: IamAccountManagedByParentOrgId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for IamAccountManagedByParentOrgId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for IamAccountManagedByParentOrgId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IamAccountManagedByParentOrgId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for IamAccountManagedByParentOrgId {
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
///Account name
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
pub struct IamAccountName(::std::string::String);
impl ::std::ops::Deref for IamAccountName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IamAccountName> for ::std::string::String {
    fn from(value: IamAccountName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for IamAccountName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for IamAccountName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IamAccountName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for IamAccountName {
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
///Account settings
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamAccountSettings {
    ///Sets an abuse contact email to notify for abuse reports.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub abuse_contact_email: ::std::option::Option<::std::string::String>,
    /**Indicates whether membership in this account requires that
Two-Factor Authentication is enabled*/
    #[serde(default)]
    pub enforce_twofactor: bool,
}
///`IamAccountType`
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
pub enum IamAccountType {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "enterprise")]
    Enterprise,
}
impl ::std::fmt::Display for IamAccountType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Standard => f.write_str("standard"),
            Self::Enterprise => f.write_str("enterprise"),
        }
    }
}
impl ::std::str::FromStr for IamAccountType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "standard" => Ok(Self::Standard),
            "enterprise" => Ok(Self::Enterprise),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IamAccountType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IamAccountType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`IamApiResponseCommon`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamApiResponseCommon {
    pub errors: IamMessages2,
    pub messages: IamMessages2,
    ///Whether the API call was successful.
    pub success: bool,
}
///`IamApiResponseCommonFailure`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamApiResponseCommonFailure {
    pub errors: ::std::vec::Vec<IamApiResponseCommonFailureErrorsItem>,
    pub messages: IamMessages2,
    pub result: (),
    ///Whether the API call was successful.
    pub success: bool,
}
///`IamApiResponseCommonFailureErrorsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamApiResponseCommonFailureErrorsItem {
    pub code: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub documentation_url: ::std::option::Option<::std::string::String>,
    pub message: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<IamApiResponseCommonFailureErrorsItemSource>,
}
///`IamApiResponseCommonFailureErrorsItemSource`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamApiResponseCommonFailureErrorsItemSource {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pointer: ::std::option::Option<::std::string::String>,
}
///`IamApiResponseSingle`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct IamApiResponseSingle(pub IamApiResponseCommon);
impl ::std::ops::Deref for IamApiResponseSingle {
    type Target = IamApiResponseCommon;
    fn deref(&self) -> &IamApiResponseCommon {
        &self.0
    }
}
impl ::std::convert::From<IamApiResponseSingle> for IamApiResponseCommon {
    fn from(value: IamApiResponseSingle) -> Self {
        value.0
    }
}
impl ::std::convert::From<IamApiResponseCommon> for IamApiResponseSingle {
    fn from(value: IamApiResponseCommon) -> Self {
        Self(value)
    }
}
///`IamApiResponseSingleId`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamApiResponseSingleId {
    pub errors: IamMessages2,
    pub messages: IamMessages2,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<IamApiResponseSingleIdResult>,
    ///Whether the API call was successful.
    pub success: bool,
}
///`IamApiResponseSingleIdResult`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamApiResponseSingleIdResult {
    pub id: IamCommonComponentsSchemasIdentifier,
}
///Identifier
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
pub struct IamCommonComponentsSchemasIdentifier(::std::string::String);
impl ::std::ops::Deref for IamCommonComponentsSchemasIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IamCommonComponentsSchemasIdentifier>
for ::std::string::String {
    fn from(value: IamCommonComponentsSchemasIdentifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for IamCommonComponentsSchemasIdentifier {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        if value.chars().count() < 32usize {
            return Err("shorter than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for IamCommonComponentsSchemasIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for IamCommonComponentsSchemasIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for IamCommonComponentsSchemasIdentifier {
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
///`IamComponentsSchemasAccount`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct IamComponentsSchemasAccount(pub IamAccount);
impl ::std::ops::Deref for IamComponentsSchemasAccount {
    type Target = IamAccount;
    fn deref(&self) -> &IamAccount {
        &self.0
    }
}
impl ::std::convert::From<IamComponentsSchemasAccount> for IamAccount {
    fn from(value: IamComponentsSchemasAccount) -> Self {
        value.0
    }
}
impl ::std::convert::From<IamAccount> for IamComponentsSchemasAccount {
    fn from(value: IamAccount) -> Self {
        Self(value)
    }
}
///`IamCreateAccount`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamCreateAccount {
    ///Account name
    pub name: ::std::string::String,
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<IamAccountType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub unit: ::std::option::Option<IamCreateAccountUnit>,
}
///information related to the tenant unit, and optionally, an id of the unit to create the account on. see https://developers.cloudflare.com/tenant/how-to/manage-accounts/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamCreateAccountUnit {
    ///Tenant unit ID
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
///`IamMessages2`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct IamMessages2(pub ::std::vec::Vec<IamMessages2Item>);
impl ::std::ops::Deref for IamMessages2 {
    type Target = ::std::vec::Vec<IamMessages2Item>;
    fn deref(&self) -> &::std::vec::Vec<IamMessages2Item> {
        &self.0
    }
}
impl ::std::convert::From<IamMessages2> for ::std::vec::Vec<IamMessages2Item> {
    fn from(value: IamMessages2) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<IamMessages2Item>> for IamMessages2 {
    fn from(value: ::std::vec::Vec<IamMessages2Item>) -> Self {
        Self(value)
    }
}
///`IamMessages2Item`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamMessages2Item {
    pub code: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub documentation_url: ::std::option::Option<::std::string::String>,
    pub message: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<IamMessages2ItemSource>,
}
///`IamMessages2ItemSource`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamMessages2ItemSource {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pointer: ::std::option::Option<::std::string::String>,
}
///`IamResponseSingleAccount`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct IamResponseSingleAccount {
    pub errors: IamMessages2,
    pub messages: IamMessages2,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<IamAccount>,
    ///Whether the API call was successful.
    pub success: bool,
}
///Which query string parameters to include in the cache key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct Include {
    ///Whether to include all query string parameters in the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub all: ::std::option::Option<bool>,
    ///A list of query string parameters to include in the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub list: ::std::option::Option<::std::vec::Vec<ParameterName>>,
}
///A language tag to normalize against. Must be printable ASCII.
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
pub struct Language(::std::string::String);
impl ::std::ops::Deref for Language {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Language> for ::std::string::String {
    fn from(value: Language) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Language {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[ -~]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[ -~]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Language {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Language {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Language {
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
///The name of the list to match against.
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
pub struct ListName(::std::string::String);
impl ::std::ops::Deref for ListName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ListName> for ::std::string::String {
    fn from(value: ListName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ListName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-zA-Z0-9_]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9_]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ListName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ListName {
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
///An expression that evaluates to the list lookup key.
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
pub struct LookupKey(::std::string::String);
impl ::std::ops::Deref for LookupKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LookupKey> for ::std::string::String {
    fn from(value: LookupKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LookupKey {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LookupKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LookupKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LookupKey {
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
///URL pattern for the Use-As-Dictionary match field. This pattern specifies which URLs can use this response as a dictionary.
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
pub struct MatchPattern(::std::string::String);
impl ::std::ops::Deref for MatchPattern {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MatchPattern> for ::std::string::String {
    fn from(value: MatchPattern) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MatchPattern {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1024usize {
            return Err("longer than 1024 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MatchPattern {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MatchPattern {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MatchPattern {
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
///A media type to normalize against. Must be printable ASCII.
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
pub struct MediaType(::std::string::String);
impl ::std::ops::Deref for MediaType {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MediaType> for ::std::string::String {
    fn from(value: MediaType) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MediaType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[ -~]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[ -~]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MediaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MediaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MediaType {
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
///Whether to use range requests. `default` is the behaviour the zone gets without this rule.
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
pub enum Mode {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "default")]
    Default,
}
impl ::std::fmt::Display for Mode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::On => f.write_str("on"),
            Self::Off => f.write_str("off"),
            Self::Default => f.write_str("default"),
        }
    }
}
impl ::std::str::FromStr for Mode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "default" => Ok(Self::Default),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Mode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Mode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The human-readable name of the ruleset.
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
pub struct Name(::std::string::String);
impl ::std::ops::Deref for Name {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Name> for ::std::string::String {
    fn from(value: Name) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Name {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Name {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Name {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Name {
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
///`PagesApiResponseCommon`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesApiResponseCommon {
    pub errors: PagesMessages,
    pub messages: PagesMessages,
    ///Whether the API call was successful.
    pub success: bool,
}
///`PagesApiResponseCommonFailure`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesApiResponseCommonFailure {
    pub errors: ::std::vec::Vec<PagesApiResponseCommonFailureErrorsItem>,
    pub messages: PagesMessages,
    pub result: (),
    ///Whether the API call was successful.
    pub success: bool,
}
///`PagesApiResponseCommonFailureErrorsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesApiResponseCommonFailureErrorsItem {
    pub code: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub documentation_url: ::std::option::Option<::std::string::String>,
    pub message: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<PagesApiResponseCommonFailureErrorsItemSource>,
}
///`PagesApiResponseCommonFailureErrorsItemSource`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesApiResponseCommonFailureErrorsItemSource {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pointer: ::std::option::Option<::std::string::String>,
}
///Configs for the project build process.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesBuildConfig {
    ///Enable build caching for the project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub build_caching: ::std::option::Option<bool>,
    ///Command used to build project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub build_command: ::std::option::Option<::std::string::String>,
    ///Assets output directory of the build.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub destination_dir: ::std::option::Option<::std::string::String>,
    ///Directory to run the command.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub root_dir: ::std::option::Option<::std::string::String>,
    ///The classifying tag for analytics.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub web_analytics_tag: ::std::option::Option<::std::string::String>,
    ///The auth token for analytics.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub web_analytics_token: ::std::option::Option<::std::string::String>,
}
///`PagesDeployment`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeployment {
    ///A list of alias URLs pointing to this deployment.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub aliases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub build_config: PagesBuildConfig,
    ///When the deployment was created.
    pub created_on: ::chrono::DateTime<::chrono::offset::Utc>,
    pub deployment_trigger: PagesDeploymentDeploymentTrigger,
    pub env_vars: PagesEnvVars,
    ///Type of deploy.
    pub environment: PagesDeploymentEnvironment,
    ///Id of the deployment.
    pub id: ::std::string::String,
    ///If the deployment has been skipped.
    pub is_skipped: bool,
    pub latest_stage: PagesStage,
    ///When the deployment was last modified.
    pub modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
    ///Id of the project.
    pub project_id: ::std::string::String,
    pub project_name: PagesProjectName,
    ///Short Id (8 character) of the deployment.
    pub short_id: ::std::string::String,
    ///Why the deployment was skipped.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub skip_reason: ::std::option::Option<PagesDeploymentSkipReason>,
    pub source: PagesSource,
    ///List of past stages.
    pub stages: ::std::vec::Vec<PagesStage>,
    ///The live URL to view this deployment.
    pub url: ::std::string::String,
    ///Whether the deployment uses functions.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub uses_functions: ::std::option::Option<bool>,
}
///`PagesDeploymentConfigValues`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValues {
    ///Constellation bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub ai_bindings: ::std::collections::HashMap<
        ::std::string::String,
        PagesDeploymentConfigValuesAiBindingsValue,
    >,
    ///Whether to always use the latest compatibility date for Pages Functions.
    pub always_use_latest_compatibility_date: bool,
    ///Analytics Engine bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub analytics_engine_datasets: ::std::collections::HashMap<
        ::std::string::String,
        PagesDeploymentConfigValuesAnalyticsEngineDatasetsValue,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub browsers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
    >,
    ///The major version of the build image to use for Pages Functions.
    pub build_image_major_version: i64,
    ///Compatibility date used for Pages Functions.
    pub compatibility_date: ::std::string::String,
    ///Compatibility flags used for Pages Functions.
    pub compatibility_flags: ::std::vec::Vec<::std::string::String>,
    ///D1 databases used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub d1_databases: ::std::collections::HashMap<
        ::std::string::String,
        PagesDeploymentConfigValuesD1DatabasesValue,
    >,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub durable_object_namespaces: ::std::collections::HashMap<
        ::std::string::String,
        PagesDeploymentConfigValuesDurableObjectNamespacesValue,
    >,
    pub env_vars: PagesEnvVars,
    ///Whether to fail open when the deployment config cannot be applied.
    pub fail_open: bool,
    ///Hyperdrive bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub hyperdrive_bindings: ::std::collections::HashMap<
        ::std::string::String,
        PagesDeploymentConfigValuesHyperdriveBindingsValue,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub kv_namespaces: ::std::collections::HashMap<
        ::std::string::String,
        PagesDeploymentConfigValuesKvNamespacesValue,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limits: ::std::option::Option<PagesDeploymentConfigValuesLimits>,
    ///mTLS bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub mtls_certificates: ::std::collections::HashMap<
        ::std::string::String,
        PagesDeploymentConfigValuesMtlsCertificatesValue,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub placement: ::std::option::Option<PagesDeploymentConfigValuesPlacement>,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub queue_producers: ::std::collections::HashMap<
        ::std::string::String,
        PagesDeploymentConfigValuesQueueProducersValue,
    >,
    ///R2 buckets used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub r2_buckets: ::std::collections::HashMap<
        ::std::string::String,
        PagesDeploymentConfigValuesR2BucketsValue,
    >,
    ///Services used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub services: ::std::collections::HashMap<
        ::std::string::String,
        PagesDeploymentConfigValuesServicesValue,
    >,
    ///The usage model for Pages Functions.
    pub usage_model: PagesDeploymentConfigValuesUsageModel,
    ///Vectorize bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub vectorize_bindings: ::std::collections::HashMap<
        ::std::string::String,
        PagesDeploymentConfigValuesVectorizeBindingsValue,
    >,
    ///Hash of the Wrangler configuration used for the deployment.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub wrangler_config_hash: ::std::option::Option<::std::string::String>,
}
///AI binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesAiBindingsValue {
    pub project_id: ::std::string::String,
}
///Analytics Engine binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesAnalyticsEngineDatasetsValue {
    ///Name of the dataset.
    pub dataset: ::std::string::String,
}
///D1 binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesD1DatabasesValue {
    ///UUID of the D1 database.
    pub id: ::std::string::String,
}
///Durable Object binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesDurableObjectNamespacesValue {
    ///ID of the Durable Object namespace.
    pub namespace_id: ::std::string::String,
}
///Hyperdrive binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesHyperdriveBindingsValue {
    pub id: ::std::string::String,
}
///KV namespace binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesKvNamespacesValue {
    ///ID of the KV namespace.
    pub namespace_id: ::std::string::String,
}
///Limits for Pages Functions.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesLimits {
    ///CPU time limit in milliseconds.
    pub cpu_ms: i64,
}
///mTLS binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesMtlsCertificatesValue {
    pub certificate_id: ::std::string::String,
}
///Placement setting used for Pages Functions.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesPlacement {
    ///Placement mode.
    pub mode: ::std::string::String,
}
///Queue Producer binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesQueueProducersValue {
    ///Name of the Queue.
    pub name: ::std::string::String,
}
///R2 binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesR2BucketsValue {
    ///Jurisdiction of the R2 bucket.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub jurisdiction: ::std::option::Option<::std::string::String>,
    ///Name of the R2 bucket.
    pub name: ::std::string::String,
}
///`PagesDeploymentConfigValuesRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequest {
    ///Constellation bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub ai_bindings: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<PagesDeploymentConfigValuesRequestAiBindingsValue>,
    >,
    ///Whether to always use the latest compatibility date for Pages Functions.
    #[serde(default)]
    pub always_use_latest_compatibility_date: bool,
    ///Analytics Engine bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub analytics_engine_datasets: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<
            PagesDeploymentConfigValuesRequestAnalyticsEngineDatasetsValue,
        >,
    >,
    ///Browser bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub browsers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        >,
    >,
    ///The major version of the build image to use for Pages Functions.
    #[serde(default = "defaults::default_u64::<i64, 3>")]
    pub build_image_major_version: i64,
    ///Compatibility date used for Pages Functions.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compatibility_date: ::std::option::Option<::std::string::String>,
    ///Compatibility flags used for Pages Functions.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub compatibility_flags: ::std::vec::Vec<::std::string::String>,
    ///D1 databases used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub d1_databases: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<PagesDeploymentConfigValuesRequestD1DatabasesValue>,
    >,
    ///Durable Object namespaces used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub durable_object_namespaces: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<
            PagesDeploymentConfigValuesRequestDurableObjectNamespacesValue,
        >,
    >,
    ///Environment variables used for builds and Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub env_vars: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<PagesDeploymentConfigValuesRequestEnvVarsValue>,
    >,
    ///Whether to fail open when the deployment config cannot be applied.
    #[serde(default = "defaults::default_bool::<true>")]
    pub fail_open: bool,
    ///Hyperdrive bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub hyperdrive_bindings: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<PagesDeploymentConfigValuesRequestHyperdriveBindingsValue>,
    >,
    ///KV namespaces used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub kv_namespaces: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<PagesDeploymentConfigValuesRequestKvNamespacesValue>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limits: ::std::option::Option<PagesDeploymentConfigValuesRequestLimits>,
    ///mTLS bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub mtls_certificates: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<PagesDeploymentConfigValuesRequestMtlsCertificatesValue>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub placement: ::std::option::Option<PagesDeploymentConfigValuesRequestPlacement>,
    ///Queue Producer bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub queue_producers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<PagesDeploymentConfigValuesRequestQueueProducersValue>,
    >,
    ///R2 buckets used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub r2_buckets: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<PagesDeploymentConfigValuesRequestR2BucketsValue>,
    >,
    ///Services used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub services: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<PagesDeploymentConfigValuesRequestServicesValue>,
    >,
    ///The usage model for Pages Functions.
    #[serde(default = "defaults::pages_deployment_config_values_request_usage_model")]
    pub usage_model: PagesDeploymentConfigValuesRequestUsageModel,
    ///Vectorize bindings used for Pages Functions.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub vectorize_bindings: ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<PagesDeploymentConfigValuesRequestVectorizeBindingsValue>,
    >,
    ///Hash of the Wrangler configuration used for the deployment.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub wrangler_config_hash: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PagesDeploymentConfigValuesRequest {
    fn default() -> Self {
        Self {
            ai_bindings: Default::default(),
            always_use_latest_compatibility_date: Default::default(),
            analytics_engine_datasets: Default::default(),
            browsers: Default::default(),
            build_image_major_version: defaults::default_u64::<i64, 3>(),
            compatibility_date: Default::default(),
            compatibility_flags: Default::default(),
            d1_databases: Default::default(),
            durable_object_namespaces: Default::default(),
            env_vars: Default::default(),
            fail_open: defaults::default_bool::<true>(),
            hyperdrive_bindings: Default::default(),
            kv_namespaces: Default::default(),
            limits: Default::default(),
            mtls_certificates: Default::default(),
            placement: Default::default(),
            queue_producers: Default::default(),
            r2_buckets: Default::default(),
            services: Default::default(),
            usage_model: defaults::pages_deployment_config_values_request_usage_model(),
            vectorize_bindings: Default::default(),
            wrangler_config_hash: Default::default(),
        }
    }
}
///AI binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestAiBindingsValue {
    pub project_id: ::std::string::String,
}
///Analytics Engine binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestAnalyticsEngineDatasetsValue {
    ///Name of the dataset.
    pub dataset: ::std::string::String,
}
///D1 binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestD1DatabasesValue {
    ///UUID of the D1 database.
    pub id: ::std::string::String,
}
///Durable Object binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestDurableObjectNamespacesValue {
    ///ID of the Durable Object namespace.
    pub namespace_id: ::std::string::String,
}
///`PagesDeploymentConfigValuesRequestEnvVarsValue`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum PagesDeploymentConfigValuesRequestEnvVarsValue {
    PlainTextEnvVar(PagesPlainTextEnvVar),
    SecretTextEnvVar(PagesSecretTextEnvVar),
}
impl ::std::convert::From<PagesPlainTextEnvVar>
for PagesDeploymentConfigValuesRequestEnvVarsValue {
    fn from(value: PagesPlainTextEnvVar) -> Self {
        Self::PlainTextEnvVar(value)
    }
}
impl ::std::convert::From<PagesSecretTextEnvVar>
for PagesDeploymentConfigValuesRequestEnvVarsValue {
    fn from(value: PagesSecretTextEnvVar) -> Self {
        Self::SecretTextEnvVar(value)
    }
}
///Hyperdrive binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestHyperdriveBindingsValue {
    pub id: ::std::string::String,
}
///KV namespace binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestKvNamespacesValue {
    ///ID of the KV namespace.
    pub namespace_id: ::std::string::String,
}
///Limits for Pages Functions.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestLimits {
    ///CPU time limit in milliseconds.
    pub cpu_ms: i64,
}
///mTLS binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestMtlsCertificatesValue {
    pub certificate_id: ::std::string::String,
}
///Placement setting used for Pages Functions.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestPlacement {
    ///Placement mode.
    pub mode: ::std::string::String,
}
///Queue Producer binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestQueueProducersValue {
    ///Name of the Queue.
    pub name: ::std::string::String,
}
///R2 binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestR2BucketsValue {
    ///Jurisdiction of the R2 bucket.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub jurisdiction: ::std::option::Option<::std::string::String>,
    ///Name of the R2 bucket.
    pub name: ::std::string::String,
}
///Service binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestServicesValue {
    ///The entrypoint to bind to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub entrypoint: ::std::option::Option<::std::string::String>,
    ///The Service environment.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub environment: ::std::option::Option<::std::string::String>,
    ///The Service name.
    pub service: ::std::string::String,
}
///The usage model for Pages Functions.
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
pub enum PagesDeploymentConfigValuesRequestUsageModel {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "bundled")]
    Bundled,
    #[serde(rename = "unbound")]
    Unbound,
}
impl ::std::fmt::Display for PagesDeploymentConfigValuesRequestUsageModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Standard => f.write_str("standard"),
            Self::Bundled => f.write_str("bundled"),
            Self::Unbound => f.write_str("unbound"),
        }
    }
}
impl ::std::str::FromStr for PagesDeploymentConfigValuesRequestUsageModel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "standard" => Ok(Self::Standard),
            "bundled" => Ok(Self::Bundled),
            "unbound" => Ok(Self::Unbound),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesDeploymentConfigValuesRequestUsageModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PagesDeploymentConfigValuesRequestUsageModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for PagesDeploymentConfigValuesRequestUsageModel {
    fn default() -> Self {
        PagesDeploymentConfigValuesRequestUsageModel::Standard
    }
}
///Vectorize binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesRequestVectorizeBindingsValue {
    pub index_name: ::std::string::String,
}
///Service binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesServicesValue {
    ///The entrypoint to bind to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub entrypoint: ::std::option::Option<::std::string::String>,
    ///The Service environment.
    pub environment: ::std::string::String,
    ///The Service name.
    pub service: ::std::string::String,
}
///The usage model for Pages Functions.
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
pub enum PagesDeploymentConfigValuesUsageModel {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "bundled")]
    Bundled,
    #[serde(rename = "unbound")]
    Unbound,
}
impl ::std::fmt::Display for PagesDeploymentConfigValuesUsageModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Standard => f.write_str("standard"),
            Self::Bundled => f.write_str("bundled"),
            Self::Unbound => f.write_str("unbound"),
        }
    }
}
impl ::std::str::FromStr for PagesDeploymentConfigValuesUsageModel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "standard" => Ok(Self::Standard),
            "bundled" => Ok(Self::Bundled),
            "unbound" => Ok(Self::Unbound),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesDeploymentConfigValuesUsageModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PagesDeploymentConfigValuesUsageModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Vectorize binding.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentConfigValuesVectorizeBindingsValue {
    pub index_name: ::std::string::String,
}
///Info about what caused the deployment.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentDeploymentTrigger {
    pub metadata: PagesDeploymentDeploymentTriggerMetadata,
    ///What caused the deployment.
    #[serde(rename = "type")]
    pub type_: PagesDeploymentDeploymentTriggerType,
}
///Additional info about the trigger.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesDeploymentDeploymentTriggerMetadata {
    ///Where the trigger happened.
    pub branch: ::std::string::String,
    ///Whether the deployment trigger commit was dirty.
    pub commit_dirty: bool,
    ///Hash of the deployment trigger commit.
    pub commit_hash: ::std::string::String,
    ///Message of the deployment trigger commit.
    pub commit_message: ::std::string::String,
}
///What caused the deployment.
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
pub enum PagesDeploymentDeploymentTriggerType {
    #[serde(rename = "github:push")]
    GithubPush,
    #[serde(rename = "ad_hoc")]
    AdHoc,
    #[serde(rename = "deploy_hook")]
    DeployHook,
}
impl ::std::fmt::Display for PagesDeploymentDeploymentTriggerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::GithubPush => f.write_str("github:push"),
            Self::AdHoc => f.write_str("ad_hoc"),
            Self::DeployHook => f.write_str("deploy_hook"),
        }
    }
}
impl ::std::str::FromStr for PagesDeploymentDeploymentTriggerType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "github:push" => Ok(Self::GithubPush),
            "ad_hoc" => Ok(Self::AdHoc),
            "deploy_hook" => Ok(Self::DeployHook),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesDeploymentDeploymentTriggerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PagesDeploymentDeploymentTriggerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Type of deploy.
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
pub enum PagesDeploymentEnvironment {
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "production")]
    Production,
}
impl ::std::fmt::Display for PagesDeploymentEnvironment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Preview => f.write_str("preview"),
            Self::Production => f.write_str("production"),
        }
    }
}
impl ::std::str::FromStr for PagesDeploymentEnvironment {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "preview" => Ok(Self::Preview),
            "production" => Ok(Self::Production),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesDeploymentEnvironment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PagesDeploymentEnvironment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Why the deployment was skipped.
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
pub enum PagesDeploymentSkipReason {
    #[serde(rename = "commit_message")]
    CommitMessage,
    #[serde(rename = "preview_deployments_disabled")]
    PreviewDeploymentsDisabled,
    #[serde(rename = "production_deployments_disabled")]
    ProductionDeploymentsDisabled,
    #[serde(rename = "path_config")]
    PathConfig,
    #[serde(rename = "branch_config")]
    BranchConfig,
    #[serde(rename = "pages_to_workers_conversion")]
    PagesToWorkersConversion,
}
impl ::std::fmt::Display for PagesDeploymentSkipReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CommitMessage => f.write_str("commit_message"),
            Self::PreviewDeploymentsDisabled => {
                f.write_str("preview_deployments_disabled")
            }
            Self::ProductionDeploymentsDisabled => {
                f.write_str("production_deployments_disabled")
            }
            Self::PathConfig => f.write_str("path_config"),
            Self::BranchConfig => f.write_str("branch_config"),
            Self::PagesToWorkersConversion => f.write_str("pages_to_workers_conversion"),
        }
    }
}
impl ::std::str::FromStr for PagesDeploymentSkipReason {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "commit_message" => Ok(Self::CommitMessage),
            "preview_deployments_disabled" => Ok(Self::PreviewDeploymentsDisabled),
            "production_deployments_disabled" => Ok(Self::ProductionDeploymentsDisabled),
            "path_config" => Ok(Self::PathConfig),
            "branch_config" => Ok(Self::BranchConfig),
            "pages_to_workers_conversion" => Ok(Self::PagesToWorkersConversion),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesDeploymentSkipReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PagesDeploymentSkipReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Environment variables used for builds and Pages Functions.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct PagesEnvVars(
    pub ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::option::Option<PagesEnvVarsInnerValue>,
        >,
    >,
);
impl ::std::ops::Deref for PagesEnvVars {
    type Target = ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::option::Option<PagesEnvVarsInnerValue>,
        >,
    >;
    fn deref(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::option::Option<PagesEnvVarsInnerValue>,
        >,
    > {
        &self.0
    }
}
impl ::std::convert::From<PagesEnvVars>
for ::std::option::Option<
    ::std::collections::HashMap<
        ::std::string::String,
        ::std::option::Option<PagesEnvVarsInnerValue>,
    >,
> {
    fn from(value: PagesEnvVars) -> Self {
        value.0
    }
}
impl ::std::convert::From<
    ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::option::Option<PagesEnvVarsInnerValue>,
        >,
    >,
> for PagesEnvVars {
    fn from(
        value: ::std::option::Option<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::option::Option<PagesEnvVarsInnerValue>,
            >,
        >,
    ) -> Self {
        Self(value)
    }
}
///`PagesEnvVarsInnerValue`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum PagesEnvVarsInnerValue {
    PlainTextEnvVar(PagesPlainTextEnvVar),
    SecretTextEnvVar(PagesSecretTextEnvVar),
}
impl ::std::convert::From<PagesPlainTextEnvVar> for PagesEnvVarsInnerValue {
    fn from(value: PagesPlainTextEnvVar) -> Self {
        Self::PlainTextEnvVar(value)
    }
}
impl ::std::convert::From<PagesSecretTextEnvVar> for PagesEnvVarsInnerValue {
    fn from(value: PagesSecretTextEnvVar) -> Self {
        Self::SecretTextEnvVar(value)
    }
}
///Identifier.
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
pub struct PagesIdentifier(::std::string::String);
impl ::std::ops::Deref for PagesIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PagesIdentifier> for ::std::string::String {
    fn from(value: PagesIdentifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PagesIdentifier {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PagesIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PagesIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PagesIdentifier {
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
///`PagesMessages`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct PagesMessages(pub ::std::vec::Vec<PagesMessagesItem>);
impl ::std::ops::Deref for PagesMessages {
    type Target = ::std::vec::Vec<PagesMessagesItem>;
    fn deref(&self) -> &::std::vec::Vec<PagesMessagesItem> {
        &self.0
    }
}
impl ::std::convert::From<PagesMessages> for ::std::vec::Vec<PagesMessagesItem> {
    fn from(value: PagesMessages) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<PagesMessagesItem>> for PagesMessages {
    fn from(value: ::std::vec::Vec<PagesMessagesItem>) -> Self {
        Self(value)
    }
}
///`PagesMessagesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesMessagesItem {
    pub code: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub documentation_url: ::std::option::Option<::std::string::String>,
    pub message: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<PagesMessagesItemSource>,
}
///`PagesMessagesItemSource`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesMessagesItemSource {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pointer: ::std::option::Option<::std::string::String>,
}
///A plaintext environment variable.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct PagesPlainTextEnvVar(pub ::std::option::Option<PagesPlainTextEnvVarInner>);
impl ::std::ops::Deref for PagesPlainTextEnvVar {
    type Target = ::std::option::Option<PagesPlainTextEnvVarInner>;
    fn deref(&self) -> &::std::option::Option<PagesPlainTextEnvVarInner> {
        &self.0
    }
}
impl ::std::convert::From<PagesPlainTextEnvVar>
for ::std::option::Option<PagesPlainTextEnvVarInner> {
    fn from(value: PagesPlainTextEnvVar) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<PagesPlainTextEnvVarInner>>
for PagesPlainTextEnvVar {
    fn from(value: ::std::option::Option<PagesPlainTextEnvVarInner>) -> Self {
        Self(value)
    }
}
///A plaintext environment variable.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesPlainTextEnvVarInner {
    #[serde(rename = "type")]
    pub type_: PagesPlainTextEnvVarInnerType,
    ///Environment variable value.
    pub value: ::std::string::String,
}
///`PagesPlainTextEnvVarInnerType`
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
pub enum PagesPlainTextEnvVarInnerType {
    #[serde(rename = "plain_text")]
    PlainText,
}
impl ::std::fmt::Display for PagesPlainTextEnvVarInnerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PlainText => f.write_str("plain_text"),
        }
    }
}
impl ::std::str::FromStr for PagesPlainTextEnvVarInnerType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "plain_text" => Ok(Self::PlainText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesPlainTextEnvVarInnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PagesPlainTextEnvVarInnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PagesProject`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProject {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub build_config: ::std::option::Option<PagesBuildConfig>,
    pub canonical_deployment: PagesDeployment,
    ///When the project was created.
    pub created_on: ::chrono::DateTime<::chrono::offset::Utc>,
    pub deployment_configs: PagesProjectDeploymentConfigs,
    ///A list of associated custom domains for the project.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub domains: ::std::vec::Vec<::std::string::String>,
    ///Framework the project is using.
    pub framework: ::std::string::String,
    ///Version of the framework the project is using.
    pub framework_version: ::std::string::String,
    ///ID of the project.
    pub id: ::std::string::String,
    pub latest_deployment: PagesDeployment,
    pub name: PagesProjectName,
    ///Name of the preview script.
    pub preview_script_name: ::std::string::String,
    ///Production branch of the project. Used to identify production deployments.
    pub production_branch: ::std::string::String,
    ///Name of the production script.
    pub production_script_name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<PagesSource>,
    ///The Cloudflare subdomain associated with the project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subdomain: ::std::option::Option<::std::string::String>,
    ///Whether the project uses functions.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub uses_functions: ::std::option::Option<bool>,
}
///`PagesProjectAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectAtProvider {
    ///When the project was created.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_on: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///A list of associated custom domains for the project.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub domains: ::std::vec::Vec<::std::string::String>,
    ///Framework the project is using.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub framework: ::std::option::Option<::std::string::String>,
    ///Version of the framework the project is using.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub framework_version: ::std::option::Option<::std::string::String>,
    ///ID of the project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<PagesProjectName>,
    ///Name of the preview script.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub preview_script_name: ::std::option::Option<::std::string::String>,
    ///Production branch of the project. Used to identify production deployments.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub production_branch: ::std::option::Option<::std::string::String>,
    ///Name of the production script.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub production_script_name: ::std::option::Option<::std::string::String>,
    ///The Cloudflare subdomain associated with the project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subdomain: ::std::option::Option<::std::string::String>,
    ///Whether the project uses functions.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub uses_functions: ::std::option::Option<bool>,
}
///`PagesProjectCreateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectCreateRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub build_config: ::std::option::Option<PagesProjectCreateRequestBuildConfig>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deployment_configs: ::std::option::Option<
        PagesProjectCreateRequestDeploymentConfigs,
    >,
    ///Name of the project.
    pub name: ::std::string::String,
    ///Production branch of the project. Used to identify production deployments.
    pub production_branch: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<PagesProjectCreateRequestSource>,
}
///Configs for the project build process.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectCreateRequestBuildConfig {
    ///Enable build caching for the project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub build_caching: ::std::option::Option<bool>,
    ///Command used to build project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub build_command: ::std::option::Option<::std::string::String>,
    ///Output directory of the build.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub destination_dir: ::std::option::Option<::std::string::String>,
    ///Directory to run the command.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub root_dir: ::std::option::Option<::std::string::String>,
    ///The classifying tag for analytics.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub web_analytics_tag: ::std::option::Option<::std::string::String>,
    ///The auth token for analytics.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub web_analytics_token: ::std::option::Option<::std::string::String>,
}
///Configs for deployments in a project.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectCreateRequestDeploymentConfigs {
    ///Configs for preview deploys.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub preview: ::std::option::Option<PagesDeploymentConfigValuesRequest>,
    ///Configs for production deploys.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub production: ::std::option::Option<PagesDeploymentConfigValuesRequest>,
}
///Configs for the project source control.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectCreateRequestSource {
    pub config: PagesProjectCreateRequestSourceConfig,
    ///The source control management provider.
    #[serde(rename = "type")]
    pub type_: PagesProjectCreateRequestSourceType,
}
///`PagesProjectCreateRequestSourceConfig`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectCreateRequestSourceConfig {
    /**Whether to enable automatic deployments when pushing to the source repository.
When disabled, no deployments (production or preview) will be triggered automatically.
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deployments_enabled: ::std::option::Option<bool>,
    ///The owner of the repository.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<::std::string::String>,
    ///The owner ID of the repository.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub owner_id: ::std::option::Option<::std::string::String>,
    ///A list of paths that should be excluded from triggering a preview deployment. Wildcard syntax (`*`) is supported.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub path_excludes: ::std::vec::Vec<::std::string::String>,
    ///A list of paths that should be watched to trigger a preview deployment. Wildcard syntax (`*`) is supported.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub path_includes: ::std::vec::Vec<::std::string::String>,
    ///Whether to enable PR comments.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pr_comments_enabled: ::std::option::Option<bool>,
    ///A list of branches that should not trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub preview_branch_excludes: ::std::vec::Vec<::std::string::String>,
    ///A list of branches that should trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub preview_branch_includes: ::std::vec::Vec<::std::string::String>,
    ///Controls whether commits to preview branches trigger a preview deployment.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub preview_deployment_setting: ::std::option::Option<
        PagesProjectCreateRequestSourceConfigPreviewDeploymentSetting,
    >,
    ///The production branch of the repository.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub production_branch: ::std::option::Option<::std::string::String>,
    ///Whether to trigger a production deployment on commits to the production branch.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub production_deployments_enabled: ::std::option::Option<bool>,
    ///The ID of the repository.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub repo_id: ::std::option::Option<::std::string::String>,
    ///The name of the repository.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub repo_name: ::std::option::Option<::std::string::String>,
}
///Controls whether commits to preview branches trigger a preview deployment.
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
pub enum PagesProjectCreateRequestSourceConfigPreviewDeploymentSetting {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display
for PagesProjectCreateRequestSourceConfigPreviewDeploymentSetting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => f.write_str("all"),
            Self::None => f.write_str("none"),
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr
for PagesProjectCreateRequestSourceConfigPreviewDeploymentSetting {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for PagesProjectCreateRequestSourceConfigPreviewDeploymentSetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PagesProjectCreateRequestSourceConfigPreviewDeploymentSetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The source control management provider.
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
pub enum PagesProjectCreateRequestSourceType {
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
}
impl ::std::fmt::Display for PagesProjectCreateRequestSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Github => f.write_str("github"),
            Self::Gitlab => f.write_str("gitlab"),
        }
    }
}
impl ::std::str::FromStr for PagesProjectCreateRequestSourceType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "github" => Ok(Self::Github),
            "gitlab" => Ok(Self::Gitlab),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesProjectCreateRequestSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PagesProjectCreateRequestSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PagesProjectCreateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectCreateResponse {
    pub errors: PagesMessages,
    pub messages: PagesMessages,
    pub result: PagesProject,
    ///Whether the API call was successful.
    pub success: bool,
}
///`PagesProjectDeleteResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectDeleteResponse {
    pub errors: PagesMessages,
    pub messages: PagesMessages,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub result: ::std::option::Option<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    ///Whether the API call was successful.
    pub success: bool,
}
///Configs for deployments in a project.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectDeploymentConfigs {
    ///Configs for preview deploys.
    pub preview: PagesDeploymentConfigValues,
    ///Configs for production deploys.
    pub production: PagesDeploymentConfigValues,
}
///`PagesProjectForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectForProvider {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub build_config: ::std::option::Option<PagesBuildConfig>,
    pub deployment_configs: PagesProjectForProviderDeploymentConfigs,
    pub name: PagesProjectName,
    ///Production branch of the project. Used to identify production deployments.
    pub production_branch: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<PagesSource>,
}
///Configs for deployments in a project.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectForProviderDeploymentConfigs {
    ///Configs for preview deploys.
    pub preview: PagesDeploymentConfigValues,
    ///Configs for production deploys.
    pub production: PagesDeploymentConfigValues,
}
///Name of the project.
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
pub struct PagesProjectName(::std::string::String);
impl ::std::ops::Deref for PagesProjectName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PagesProjectName> for ::std::string::String {
    fn from(value: PagesProjectName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PagesProjectName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-z0-9][a-z0-9-]*$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z0-9][a-z0-9-]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PagesProjectName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PagesProjectName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PagesProjectName {
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
///`PagesProjectObserveResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectObserveResponse {
    pub errors: PagesMessages,
    pub messages: PagesMessages,
    pub result: PagesProject,
    ///Whether the API call was successful.
    pub success: bool,
}
///`PagesProjectUpdateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectUpdateRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub build_config: ::std::option::Option<PagesProjectUpdateRequestBuildConfig>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deployment_configs: ::std::option::Option<
        PagesProjectUpdateRequestDeploymentConfigs,
    >,
    ///Name of the project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Production branch of the project. Used to identify production deployments.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub production_branch: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<PagesProjectUpdateRequestSource>,
}
///Configs for the project build process.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectUpdateRequestBuildConfig {
    ///Enable build caching for the project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub build_caching: ::std::option::Option<bool>,
    ///Command used to build project.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub build_command: ::std::option::Option<::std::string::String>,
    ///Output directory of the build.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub destination_dir: ::std::option::Option<::std::string::String>,
    ///Directory to run the command.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub root_dir: ::std::option::Option<::std::string::String>,
    ///The classifying tag for analytics.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub web_analytics_tag: ::std::option::Option<::std::string::String>,
    ///The auth token for analytics.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub web_analytics_token: ::std::option::Option<::std::string::String>,
}
///Configs for deployments in a project.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectUpdateRequestDeploymentConfigs {
    ///Configs for preview deploys.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub preview: ::std::option::Option<PagesDeploymentConfigValuesRequest>,
    ///Configs for production deploys.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub production: ::std::option::Option<PagesDeploymentConfigValuesRequest>,
}
///Configs for the project source control.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectUpdateRequestSource {
    pub config: PagesProjectUpdateRequestSourceConfig,
    ///The source control management provider.
    #[serde(rename = "type")]
    pub type_: PagesProjectUpdateRequestSourceType,
}
///`PagesProjectUpdateRequestSourceConfig`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectUpdateRequestSourceConfig {
    /**Whether to enable automatic deployments when pushing to the source repository.
When disabled, no deployments (production or preview) will be triggered automatically.
*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub deployments_enabled: ::std::option::Option<bool>,
    ///The owner of the repository.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: ::std::option::Option<::std::string::String>,
    ///The owner ID of the repository.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub owner_id: ::std::option::Option<::std::string::String>,
    ///A list of paths that should be excluded from triggering a preview deployment. Wildcard syntax (`*`) is supported.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub path_excludes: ::std::vec::Vec<::std::string::String>,
    ///A list of paths that should be watched to trigger a preview deployment. Wildcard syntax (`*`) is supported.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub path_includes: ::std::vec::Vec<::std::string::String>,
    ///Whether to enable PR comments.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pr_comments_enabled: ::std::option::Option<bool>,
    ///A list of branches that should not trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub preview_branch_excludes: ::std::vec::Vec<::std::string::String>,
    ///A list of branches that should trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub preview_branch_includes: ::std::vec::Vec<::std::string::String>,
    ///Controls whether commits to preview branches trigger a preview deployment.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub preview_deployment_setting: ::std::option::Option<
        PagesProjectUpdateRequestSourceConfigPreviewDeploymentSetting,
    >,
    ///The production branch of the repository.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub production_branch: ::std::option::Option<::std::string::String>,
    ///Whether to trigger a production deployment on commits to the production branch.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub production_deployments_enabled: ::std::option::Option<bool>,
    ///The ID of the repository.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub repo_id: ::std::option::Option<::std::string::String>,
    ///The name of the repository.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub repo_name: ::std::option::Option<::std::string::String>,
}
///Controls whether commits to preview branches trigger a preview deployment.
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
pub enum PagesProjectUpdateRequestSourceConfigPreviewDeploymentSetting {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display
for PagesProjectUpdateRequestSourceConfigPreviewDeploymentSetting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => f.write_str("all"),
            Self::None => f.write_str("none"),
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr
for PagesProjectUpdateRequestSourceConfigPreviewDeploymentSetting {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for PagesProjectUpdateRequestSourceConfigPreviewDeploymentSetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PagesProjectUpdateRequestSourceConfigPreviewDeploymentSetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The source control management provider.
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
pub enum PagesProjectUpdateRequestSourceType {
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
}
impl ::std::fmt::Display for PagesProjectUpdateRequestSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Github => f.write_str("github"),
            Self::Gitlab => f.write_str("gitlab"),
        }
    }
}
impl ::std::str::FromStr for PagesProjectUpdateRequestSourceType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "github" => Ok(Self::Github),
            "gitlab" => Ok(Self::Gitlab),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesProjectUpdateRequestSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PagesProjectUpdateRequestSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PagesProjectUpdateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesProjectUpdateResponse {
    pub errors: PagesMessages,
    pub messages: PagesMessages,
    pub result: PagesProject,
    ///Whether the API call was successful.
    pub success: bool,
}
///An encrypted environment variable.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct PagesSecretTextEnvVar(pub ::std::option::Option<PagesSecretTextEnvVarInner>);
impl ::std::ops::Deref for PagesSecretTextEnvVar {
    type Target = ::std::option::Option<PagesSecretTextEnvVarInner>;
    fn deref(&self) -> &::std::option::Option<PagesSecretTextEnvVarInner> {
        &self.0
    }
}
impl ::std::convert::From<PagesSecretTextEnvVar>
for ::std::option::Option<PagesSecretTextEnvVarInner> {
    fn from(value: PagesSecretTextEnvVar) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<PagesSecretTextEnvVarInner>>
for PagesSecretTextEnvVar {
    fn from(value: ::std::option::Option<PagesSecretTextEnvVarInner>) -> Self {
        Self(value)
    }
}
///An encrypted environment variable.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesSecretTextEnvVarInner {
    #[serde(rename = "type")]
    pub type_: PagesSecretTextEnvVarInnerType,
    ///Secret value.
    pub value: ::std::string::String,
}
///`PagesSecretTextEnvVarInnerType`
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
pub enum PagesSecretTextEnvVarInnerType {
    #[serde(rename = "secret_text")]
    SecretText,
}
impl ::std::fmt::Display for PagesSecretTextEnvVarInnerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SecretText => f.write_str("secret_text"),
        }
    }
}
impl ::std::str::FromStr for PagesSecretTextEnvVarInnerType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "secret_text" => Ok(Self::SecretText),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesSecretTextEnvVarInnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PagesSecretTextEnvVarInnerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Configs for the project source control.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesSource {
    pub config: PagesSourceConfig,
    ///The source control management provider.
    #[serde(rename = "type")]
    pub type_: PagesSourceType,
}
///`PagesSourceConfig`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesSourceConfig {
    /**Whether to enable automatic deployments when pushing to the source repository.
When disabled, no deployments (production or preview) will be triggered automatically.
*/
    pub deployments_enabled: bool,
    ///The owner of the repository.
    pub owner: ::std::string::String,
    ///The owner ID of the repository.
    pub owner_id: ::std::string::String,
    ///A list of paths that should be excluded from triggering a preview deployment. Wildcard syntax (`*`) is supported.
    pub path_excludes: ::std::vec::Vec<::std::string::String>,
    ///A list of paths that should be watched to trigger a preview deployment. Wildcard syntax (`*`) is supported.
    pub path_includes: ::std::vec::Vec<::std::string::String>,
    ///Whether to enable PR comments.
    pub pr_comments_enabled: bool,
    ///A list of branches that should not trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    pub preview_branch_excludes: ::std::vec::Vec<::std::string::String>,
    ///A list of branches that should trigger a preview deployment. Wildcard syntax (`*`) is supported. Must be used with `preview_deployment_setting` set to `custom`.
    pub preview_branch_includes: ::std::vec::Vec<::std::string::String>,
    ///Controls whether commits to preview branches trigger a preview deployment.
    pub preview_deployment_setting: PagesSourceConfigPreviewDeploymentSetting,
    ///The production branch of the repository.
    pub production_branch: ::std::string::String,
    ///Whether to trigger a production deployment on commits to the production branch.
    pub production_deployments_enabled: bool,
    ///The ID of the repository.
    pub repo_id: ::std::string::String,
    ///The name of the repository.
    pub repo_name: ::std::string::String,
}
///Controls whether commits to preview branches trigger a preview deployment.
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
pub enum PagesSourceConfigPreviewDeploymentSetting {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "custom")]
    Custom,
}
impl ::std::fmt::Display for PagesSourceConfigPreviewDeploymentSetting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => f.write_str("all"),
            Self::None => f.write_str("none"),
            Self::Custom => f.write_str("custom"),
        }
    }
}
impl ::std::str::FromStr for PagesSourceConfigPreviewDeploymentSetting {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesSourceConfigPreviewDeploymentSetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PagesSourceConfigPreviewDeploymentSetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The source control management provider.
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
pub enum PagesSourceType {
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
}
impl ::std::fmt::Display for PagesSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Github => f.write_str("github"),
            Self::Gitlab => f.write_str("gitlab"),
        }
    }
}
impl ::std::str::FromStr for PagesSourceType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "github" => Ok(Self::Github),
            "gitlab" => Ok(Self::Gitlab),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PagesSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The status of the deployment.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct PagesStage {
    ///When the stage ended.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub ended_on: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///The current build stage.
    pub name: PagesStageName,
    ///When the stage started.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub started_on: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///State of the current stage.
    pub status: PagesStageStatus,
}
///The current build stage.
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
pub enum PagesStageName {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "initialize")]
    Initialize,
    #[serde(rename = "clone_repo")]
    CloneRepo,
    #[serde(rename = "build")]
    Build,
    #[serde(rename = "deploy")]
    Deploy,
}
impl ::std::fmt::Display for PagesStageName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Queued => f.write_str("queued"),
            Self::Initialize => f.write_str("initialize"),
            Self::CloneRepo => f.write_str("clone_repo"),
            Self::Build => f.write_str("build"),
            Self::Deploy => f.write_str("deploy"),
        }
    }
}
impl ::std::str::FromStr for PagesStageName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "queued" => Ok(Self::Queued),
            "initialize" => Ok(Self::Initialize),
            "clone_repo" => Ok(Self::CloneRepo),
            "build" => Ok(Self::Build),
            "deploy" => Ok(Self::Deploy),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesStageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PagesStageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///State of the current stage.
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
pub enum PagesStageStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "canceled")]
    Canceled,
}
impl ::std::fmt::Display for PagesStageStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Success => f.write_str("success"),
            Self::Idle => f.write_str("idle"),
            Self::Active => f.write_str("active"),
            Self::Failure => f.write_str("failure"),
            Self::Canceled => f.write_str("canceled"),
        }
    }
}
impl ::std::str::FromStr for PagesStageStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "success" => Ok(Self::Success),
            "idle" => Ok(Self::Idle),
            "active" => Ok(Self::Active),
            "failure" => Ok(Self::Failure),
            "canceled" => Ok(Self::Canceled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PagesStageStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PagesStageStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The name of the query string parameter to exclude.
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
pub struct ParameterName(::std::string::String);
impl ::std::ops::Deref for ParameterName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ParameterName> for ::std::string::String {
    fn from(value: ParameterName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ParameterName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ParameterName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ParameterName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ParameterName {
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
///An expression that selects the password used in the credentials check.
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
pub struct PasswordExpression(::std::string::String);
impl ::std::ops::Deref for PasswordExpression {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PasswordExpression> for ::std::string::String {
    fn from(value: PasswordExpression) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PasswordExpression {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PasswordExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PasswordExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PasswordExpression {
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
///An expression that evaluates to a value to rewrite the URI path to.
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
pub struct PathExpression(::std::string::String);
impl ::std::ops::Deref for PathExpression {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PathExpression> for ::std::string::String {
    fn from(value: PathExpression) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PathExpression {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PathExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PathExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PathExpression {
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
///A value to rewrite the URI path to.
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
pub struct PathValue(::std::string::String);
impl ::std::ops::Deref for PathValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PathValue> for ::std::string::String {
    fn from(value: PathValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PathValue {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PathValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PathValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PathValue {
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
///A JSON pointer to the field that is the source of the message.
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
pub struct Pointer(::std::string::String);
impl ::std::ops::Deref for Pointer {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Pointer> for ::std::string::String {
    fn from(value: Pointer) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Pointer {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Pointer {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Pointer {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Pointer {
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
///The Polish level to configure.
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
pub enum Polish {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "lossless")]
    Lossless,
    #[serde(rename = "lossy")]
    Lossy,
    #[serde(rename = "webp")]
    Webp,
}
impl ::std::fmt::Display for Polish {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Off => f.write_str("off"),
            Self::Lossless => f.write_str("lossless"),
            Self::Lossy => f.write_str("lossy"),
            Self::Webp => f.write_str("webp"),
        }
    }
}
impl ::std::str::FromStr for Polish {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "off" => Ok(Self::Off),
            "lossless" => Ok(Self::Lossless),
            "lossy" => Ok(Self::Lossy),
            "webp" => Ok(Self::Webp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Polish {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Polish {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The name of a legacy security product to skip the execution of.
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
pub enum Product {
    #[serde(rename = "bic")]
    Bic,
    #[serde(rename = "hot")]
    Hot,
    #[serde(rename = "rateLimit")]
    RateLimit,
    #[serde(rename = "securityLevel")]
    SecurityLevel,
    #[serde(rename = "uaBlock")]
    UaBlock,
    #[serde(rename = "waf")]
    Waf,
    #[serde(rename = "zoneLockdown")]
    ZoneLockdown,
}
impl ::std::fmt::Display for Product {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bic => f.write_str("bic"),
            Self::Hot => f.write_str("hot"),
            Self::RateLimit => f.write_str("rateLimit"),
            Self::SecurityLevel => f.write_str("securityLevel"),
            Self::UaBlock => f.write_str("uaBlock"),
            Self::Waf => f.write_str("waf"),
            Self::ZoneLockdown => f.write_str("zoneLockdown"),
        }
    }
}
impl ::std::str::FromStr for Product {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bic" => Ok(Self::Bic),
            "hot" => Ok(Self::Hot),
            "rateLimit" => Ok(Self::RateLimit),
            "securityLevel" => Ok(Self::SecurityLevel),
            "uaBlock" => Ok(Self::UaBlock),
            "waf" => Ok(Self::Waf),
            "zoneLockdown" => Ok(Self::ZoneLockdown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Product {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Product {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The public key to encrypt matched data logs with.
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
pub struct PublicKey(::std::string::String);
impl ::std::ops::Deref for PublicKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PublicKey> for ::std::string::String {
    fn from(value: PublicKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PublicKey {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PublicKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PublicKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PublicKey {
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
///A header name to qualify the directive.
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
pub struct Qualifier(::std::string::String);
impl ::std::ops::Deref for Qualifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Qualifier> for ::std::string::String {
    fn from(value: Qualifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Qualifier {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Qualifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Qualifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Qualifier {
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
///An expression that evaluates to a value to rewrite the URI query to.
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
pub struct QueryExpression(::std::string::String);
impl ::std::ops::Deref for QueryExpression {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<QueryExpression> for ::std::string::String {
    fn from(value: QueryExpression) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for QueryExpression {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for QueryExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QueryExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for QueryExpression {
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
///The raw request field to log.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RawRequestField {
    ///The name of the header.
    pub name: HeaderName,
}
///The raw response field to log.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RawResponseField {
    ///The name of the response header.
    pub name: HeaderName,
    ///Whether to log duplicate values of the same header.
    #[serde(default)]
    pub preserve_duplicates: bool,
}
///An expression that evaluates to a URL to redirect the request to.
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
pub struct RedirectExpression(::std::string::String);
impl ::std::ops::Deref for RedirectExpression {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RedirectExpression> for ::std::string::String {
    fn from(value: RedirectExpression) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RedirectExpression {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RedirectExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RedirectExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RedirectExpression {
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
///A URL to redirect the request to.
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
pub struct RedirectValue(::std::string::String);
impl ::std::ops::Deref for RedirectValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RedirectValue> for ::std::string::String {
    fn from(value: RedirectValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RedirectValue {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RedirectValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RedirectValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RedirectValue {
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
///The reference of the rule (the rule's ID by default).
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
pub struct Ref(::std::string::String);
impl ::std::ops::Deref for Ref {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Ref> for ::std::string::String {
    fn from(value: Ref) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Ref {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Ref {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Ref {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Ref {
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
///`RemoveCacheTagsExpressionOperation`
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
pub enum RemoveCacheTagsExpressionOperation {
    #[serde(rename = "remove")]
    Remove,
}
impl ::std::fmt::Display for RemoveCacheTagsExpressionOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Remove => f.write_str("remove"),
        }
    }
}
impl ::std::str::FromStr for RemoveCacheTagsExpressionOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "remove" => Ok(Self::Remove),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RemoveCacheTagsExpressionOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RemoveCacheTagsExpressionOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RemoveCacheTagsValuesOperation`
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
pub enum RemoveCacheTagsValuesOperation {
    #[serde(rename = "remove")]
    Remove,
}
impl ::std::fmt::Display for RemoveCacheTagsValuesOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Remove => f.write_str("remove"),
        }
    }
}
impl ::std::str::FromStr for RemoveCacheTagsValuesOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "remove" => Ok(Self::Remove),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RemoveCacheTagsValuesOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RemoveCacheTagsValuesOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RemoveDirectiveOperation`
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
pub enum RemoveDirectiveOperation {
    #[serde(rename = "remove")]
    Remove,
}
impl ::std::fmt::Display for RemoveDirectiveOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Remove => f.write_str("remove"),
        }
    }
}
impl ::std::str::FromStr for RemoveDirectiveOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "remove" => Ok(Self::Remove),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RemoveDirectiveOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RemoveDirectiveOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RemoveHeaderOperation`
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
pub enum RemoveHeaderOperation {
    #[serde(rename = "remove")]
    Remove,
}
impl ::std::fmt::Display for RemoveHeaderOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Remove => f.write_str("remove"),
        }
    }
}
impl ::std::str::FromStr for RemoveHeaderOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "remove" => Ok(Self::Remove),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RemoveHeaderOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RemoveHeaderOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The request body buffering mode.
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
pub enum RequestBodyBuffering {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "full")]
    Full,
}
impl ::std::fmt::Display for RequestBodyBuffering {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Standard => f.write_str("standard"),
            Self::Full => f.write_str("full"),
        }
    }
}
impl ::std::str::FromStr for RequestBodyBuffering {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "standard" => Ok(Self::Standard),
            "full" => Ok(Self::Full),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RequestBodyBuffering {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RequestBodyBuffering {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The response to show when the block is applied.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct Response {
    ///The content to return.
    pub content: Content,
    ///The type of the content to return.
    pub content_type: ContentType,
    ///The status code to return.
    pub status_code: i64,
}
///The response body buffering mode.
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
pub enum ResponseBodyBuffering {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "standard")]
    Standard,
}
impl ::std::fmt::Display for ResponseBodyBuffering {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Standard => f.write_str("standard"),
        }
    }
}
impl ::std::str::FromStr for ResponseBodyBuffering {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "standard" => Ok(Self::Standard),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseBodyBuffering {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseBodyBuffering {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A rule-level override.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RuleOverride {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    pub id: RulesetsRuleId,
    ///The score threshold to use for the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub score_threshold: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sensitivity_level: ::std::option::Option<RulesetsExecuteSensitivityLevel>,
}
///`RulesetAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetAtProvider {
    ///An informative description of the ruleset.
    #[serde(default)]
    pub description: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRulesetId>,
    ///The timestamp of when the ruleset was last modified.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_updated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///The human-readable name of the ruleset.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<RulesetsRulesetVersion>,
}
///`RulesetForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetForProvider {
    ///An informative description of the ruleset.
    #[serde(default)]
    pub description: ::std::string::String,
    ///The human-readable name of the ruleset.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}
///`RulesetsBlockRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsBlockRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsBlockRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<RulesetsBlockRuleActionParameters>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsBlockRuleAction`
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
pub enum RulesetsBlockRuleAction {
    #[serde(rename = "block")]
    Block,
}
impl ::std::fmt::Display for RulesetsBlockRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Block => f.write_str("block"),
        }
    }
}
impl ::std::str::FromStr for RulesetsBlockRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsBlockRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsBlockRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsBlockRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsBlockRuleActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response: ::std::option::Option<Response>,
}
///`RulesetsChallengeRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsChallengeRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsChallengeRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsChallengeRuleAction`
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
pub enum RulesetsChallengeRuleAction {
    #[serde(rename = "challenge")]
    Challenge,
}
impl ::std::fmt::Display for RulesetsChallengeRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Challenge => f.write_str("challenge"),
        }
    }
}
impl ::std::str::FromStr for RulesetsChallengeRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "challenge" => Ok(Self::Challenge),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsChallengeRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsChallengeRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsCompressResponseRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsCompressResponseRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsCompressResponseRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<
        RulesetsCompressResponseRuleActionParameters,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsCompressResponseRuleAction`
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
pub enum RulesetsCompressResponseRuleAction {
    #[serde(rename = "compress_response")]
    CompressResponse,
}
impl ::std::fmt::Display for RulesetsCompressResponseRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CompressResponse => f.write_str("compress_response"),
        }
    }
}
impl ::std::str::FromStr for RulesetsCompressResponseRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "compress_response" => Ok(Self::CompressResponse),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsCompressResponseRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsCompressResponseRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsCompressResponseRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsCompressResponseRuleActionParameters {
    ///Custom order for compression algorithms.
    pub algorithms: ::std::vec::Vec<Algorithm>,
}
///`RulesetsDDoSDynamicRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsDDoSDynamicRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsDDoSDynamicRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsDDoSDynamicRuleAction`
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
pub enum RulesetsDDoSDynamicRuleAction {
    #[serde(rename = "ddos_dynamic")]
    DdosDynamic,
}
impl ::std::fmt::Display for RulesetsDDoSDynamicRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DdosDynamic => f.write_str("ddos_dynamic"),
        }
    }
}
impl ::std::str::FromStr for RulesetsDDoSDynamicRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ddos_dynamic" => Ok(Self::DdosDynamic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsDDoSDynamicRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsDDoSDynamicRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A list of error messages.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsErrors(pub ::std::vec::Vec<RulesetsMessage>);
impl ::std::ops::Deref for RulesetsErrors {
    type Target = ::std::vec::Vec<RulesetsMessage>;
    fn deref(&self) -> &::std::vec::Vec<RulesetsMessage> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsErrors> for ::std::vec::Vec<RulesetsMessage> {
    fn from(value: RulesetsErrors) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<RulesetsMessage>> for RulesetsErrors {
    fn from(value: ::std::vec::Vec<RulesetsMessage>) -> Self {
        Self(value)
    }
}
///A list of category-level overrides. This option has the second-highest precedence after rule-level overrides.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsExecuteCategoryOverrides(pub ::std::vec::Vec<CategoryOverride>);
impl ::std::ops::Deref for RulesetsExecuteCategoryOverrides {
    type Target = ::std::vec::Vec<CategoryOverride>;
    fn deref(&self) -> &::std::vec::Vec<CategoryOverride> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsExecuteCategoryOverrides>
for ::std::vec::Vec<CategoryOverride> {
    fn from(value: RulesetsExecuteCategoryOverrides) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<CategoryOverride>>
for RulesetsExecuteCategoryOverrides {
    fn from(value: ::std::vec::Vec<CategoryOverride>) -> Self {
        Self(value)
    }
}
///The configuration to use for matched data logging.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsExecuteMatchedData {
    ///The public key to encrypt matched data logs with.
    pub public_key: PublicKey,
}
///A set of overrides to apply to the target ruleset.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsExecuteOverrides {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsExecuteCategoryOverrides>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rules: ::std::option::Option<RulesetsExecuteRuleOverrides>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sensitivity_level: ::std::option::Option<RulesetsExecuteSensitivityLevel>,
}
///`RulesetsExecuteRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsExecuteRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsExecuteRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<RulesetsExecuteRuleActionParameters>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsExecuteRuleAction`
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
pub enum RulesetsExecuteRuleAction {
    #[serde(rename = "execute")]
    Execute,
}
impl ::std::fmt::Display for RulesetsExecuteRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Execute => f.write_str("execute"),
        }
    }
}
impl ::std::str::FromStr for RulesetsExecuteRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "execute" => Ok(Self::Execute),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsExecuteRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsExecuteRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsExecuteRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsExecuteRuleActionParameters {
    pub id: RulesetsRulesetId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub matched_data: ::std::option::Option<RulesetsExecuteMatchedData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub overrides: ::std::option::Option<RulesetsExecuteOverrides>,
}
///A list of rule-level overrides. This option has the highest precedence.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsExecuteRuleOverrides(pub ::std::vec::Vec<RuleOverride>);
impl ::std::ops::Deref for RulesetsExecuteRuleOverrides {
    type Target = ::std::vec::Vec<RuleOverride>;
    fn deref(&self) -> &::std::vec::Vec<RuleOverride> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsExecuteRuleOverrides>
for ::std::vec::Vec<RuleOverride> {
    fn from(value: RulesetsExecuteRuleOverrides) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<RuleOverride>>
for RulesetsExecuteRuleOverrides {
    fn from(value: ::std::vec::Vec<RuleOverride>) -> Self {
        Self(value)
    }
}
///`RulesetsExecuteSensitivityLevel`
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
pub enum RulesetsExecuteSensitivityLevel {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "eoff")]
    Eoff,
}
impl ::std::fmt::Display for RulesetsExecuteSensitivityLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("default"),
            Self::Medium => f.write_str("medium"),
            Self::Low => f.write_str("low"),
            Self::Eoff => f.write_str("eoff"),
        }
    }
}
impl ::std::str::FromStr for RulesetsExecuteSensitivityLevel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "medium" => Ok(Self::Medium),
            "low" => Ok(Self::Low),
            "eoff" => Ok(Self::Eoff),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsExecuteSensitivityLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsExecuteSensitivityLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsForceConnectionCloseRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsForceConnectionCloseRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsForceConnectionCloseRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsForceConnectionCloseRuleAction`
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
pub enum RulesetsForceConnectionCloseRuleAction {
    #[serde(rename = "force_connection_close")]
    ForceConnectionClose,
}
impl ::std::fmt::Display for RulesetsForceConnectionCloseRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ForceConnectionClose => f.write_str("force_connection_close"),
        }
    }
}
impl ::std::str::FromStr for RulesetsForceConnectionCloseRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "force_connection_close" => Ok(Self::ForceConnectionClose),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsForceConnectionCloseRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsForceConnectionCloseRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsJsChallengeRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsJsChallengeRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsJsChallengeRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsJsChallengeRuleAction`
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
pub enum RulesetsJsChallengeRuleAction {
    #[serde(rename = "js_challenge")]
    JsChallenge,
}
impl ::std::fmt::Display for RulesetsJsChallengeRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::JsChallenge => f.write_str("js_challenge"),
        }
    }
}
impl ::std::str::FromStr for RulesetsJsChallengeRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "js_challenge" => Ok(Self::JsChallenge),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsJsChallengeRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsJsChallengeRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The cookie fields to log.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsLogCustomFieldCookieFields(pub ::std::vec::Vec<CookieField>);
impl ::std::ops::Deref for RulesetsLogCustomFieldCookieFields {
    type Target = ::std::vec::Vec<CookieField>;
    fn deref(&self) -> &::std::vec::Vec<CookieField> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsLogCustomFieldCookieFields>
for ::std::vec::Vec<CookieField> {
    fn from(value: RulesetsLogCustomFieldCookieFields) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<CookieField>>
for RulesetsLogCustomFieldCookieFields {
    fn from(value: ::std::vec::Vec<CookieField>) -> Self {
        Self(value)
    }
}
///The raw response fields to log.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsLogCustomFieldRawResponseFields(
    pub ::std::vec::Vec<RawResponseField>,
);
impl ::std::ops::Deref for RulesetsLogCustomFieldRawResponseFields {
    type Target = ::std::vec::Vec<RawResponseField>;
    fn deref(&self) -> &::std::vec::Vec<RawResponseField> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsLogCustomFieldRawResponseFields>
for ::std::vec::Vec<RawResponseField> {
    fn from(value: RulesetsLogCustomFieldRawResponseFields) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<RawResponseField>>
for RulesetsLogCustomFieldRawResponseFields {
    fn from(value: ::std::vec::Vec<RawResponseField>) -> Self {
        Self(value)
    }
}
///The raw request fields to log.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsLogCustomFieldRequestFields(pub ::std::vec::Vec<RawRequestField>);
impl ::std::ops::Deref for RulesetsLogCustomFieldRequestFields {
    type Target = ::std::vec::Vec<RawRequestField>;
    fn deref(&self) -> &::std::vec::Vec<RawRequestField> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsLogCustomFieldRequestFields>
for ::std::vec::Vec<RawRequestField> {
    fn from(value: RulesetsLogCustomFieldRequestFields) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<RawRequestField>>
for RulesetsLogCustomFieldRequestFields {
    fn from(value: ::std::vec::Vec<RawRequestField>) -> Self {
        Self(value)
    }
}
///The transformed response fields to log.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsLogCustomFieldResponseFields(
    pub ::std::vec::Vec<TransformedResponseField>,
);
impl ::std::ops::Deref for RulesetsLogCustomFieldResponseFields {
    type Target = ::std::vec::Vec<TransformedResponseField>;
    fn deref(&self) -> &::std::vec::Vec<TransformedResponseField> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsLogCustomFieldResponseFields>
for ::std::vec::Vec<TransformedResponseField> {
    fn from(value: RulesetsLogCustomFieldResponseFields) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<TransformedResponseField>>
for RulesetsLogCustomFieldResponseFields {
    fn from(value: ::std::vec::Vec<TransformedResponseField>) -> Self {
        Self(value)
    }
}
///`RulesetsLogCustomFieldRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsLogCustomFieldRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsLogCustomFieldRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<
        RulesetsLogCustomFieldRuleActionParameters,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsLogCustomFieldRuleAction`
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
pub enum RulesetsLogCustomFieldRuleAction {
    #[serde(rename = "log_custom_field")]
    LogCustomField,
}
impl ::std::fmt::Display for RulesetsLogCustomFieldRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LogCustomField => f.write_str("log_custom_field"),
        }
    }
}
impl ::std::str::FromStr for RulesetsLogCustomFieldRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "log_custom_field" => Ok(Self::LogCustomField),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsLogCustomFieldRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsLogCustomFieldRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsLogCustomFieldRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsLogCustomFieldRuleActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cookie_fields: ::std::option::Option<RulesetsLogCustomFieldCookieFields>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub raw_response_fields: ::std::option::Option<
        RulesetsLogCustomFieldRawResponseFields,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub request_fields: ::std::option::Option<RulesetsLogCustomFieldRequestFields>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_fields: ::std::option::Option<RulesetsLogCustomFieldResponseFields>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transformed_request_fields: ::std::option::Option<
        RulesetsLogCustomFieldTransformedRequestFields,
    >,
}
///The transformed request fields to log.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsLogCustomFieldTransformedRequestFields(
    pub ::std::vec::Vec<TransformedRequestField>,
);
impl ::std::ops::Deref for RulesetsLogCustomFieldTransformedRequestFields {
    type Target = ::std::vec::Vec<TransformedRequestField>;
    fn deref(&self) -> &::std::vec::Vec<TransformedRequestField> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsLogCustomFieldTransformedRequestFields>
for ::std::vec::Vec<TransformedRequestField> {
    fn from(value: RulesetsLogCustomFieldTransformedRequestFields) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<TransformedRequestField>>
for RulesetsLogCustomFieldTransformedRequestFields {
    fn from(value: ::std::vec::Vec<TransformedRequestField>) -> Self {
        Self(value)
    }
}
///`RulesetsLogRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsLogRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsLogRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsLogRuleAction`
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
pub enum RulesetsLogRuleAction {
    #[serde(rename = "log")]
    Log,
}
impl ::std::fmt::Display for RulesetsLogRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Log => f.write_str("log"),
        }
    }
}
impl ::std::str::FromStr for RulesetsLogRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "log" => Ok(Self::Log),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsLogRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsLogRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsManagedChallengeRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsManagedChallengeRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsManagedChallengeRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsManagedChallengeRuleAction`
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
pub enum RulesetsManagedChallengeRuleAction {
    #[serde(rename = "managed_challenge")]
    ManagedChallenge,
}
impl ::std::fmt::Display for RulesetsManagedChallengeRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ManagedChallenge => f.write_str("managed_challenge"),
        }
    }
}
impl ::std::str::FromStr for RulesetsManagedChallengeRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "managed_challenge" => Ok(Self::ManagedChallenge),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsManagedChallengeRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsManagedChallengeRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A message.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsMessage {
    ///A unique code for this message.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<i64>,
    ///A text description of this message.
    pub message: Description,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<Source>,
}
///A list of warning messages.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsMessages(pub ::std::vec::Vec<RulesetsMessage>);
impl ::std::ops::Deref for RulesetsMessages {
    type Target = ::std::vec::Vec<RulesetsMessage>;
    fn deref(&self) -> &::std::vec::Vec<RulesetsMessage> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsMessages> for ::std::vec::Vec<RulesetsMessage> {
    fn from(value: RulesetsMessages) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<RulesetsMessage>> for RulesetsMessages {
    fn from(value: ::std::vec::Vec<RulesetsMessage>) -> Self {
        Self(value)
    }
}
///A redirect based on a bulk list lookup.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRedirectFromList {
    ///An expression that evaluates to the list lookup key.
    pub key: LookupKey,
    ///The name of the list to match against.
    pub name: ListName,
}
///A redirect based on the request properties.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRedirectFromValue {
    ///Whether to keep the query string of the original request.
    #[serde(default)]
    pub preserve_query_string: bool,
    ///The status code to use for the redirect (301, 302, 303, 307 or 308).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status_code: ::std::option::Option<i64>,
    pub target_url: TargetUrl,
}
///`RulesetsRedirectRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRedirectRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsRedirectRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<RulesetsRedirectRuleActionParameters>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsRedirectRuleAction`
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
pub enum RulesetsRedirectRuleAction {
    #[serde(rename = "redirect")]
    Redirect,
}
impl ::std::fmt::Display for RulesetsRedirectRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Redirect => f.write_str("redirect"),
        }
    }
}
impl ::std::str::FromStr for RulesetsRedirectRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "redirect" => Ok(Self::Redirect),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRedirectRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRedirectRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsRedirectRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRedirectRuleActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub from_list: ::std::option::Option<RulesetsRedirectFromList>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub from_value: ::std::option::Option<RulesetsRedirectFromValue>,
}
///`RulesetsRequestRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum RulesetsRequestRule {
    BlockRule(RulesetsBlockRule),
    ChallengeRule(RulesetsChallengeRule),
    CompressResponseRule(RulesetsCompressResponseRule),
    DDoSDynamicRule(RulesetsDDoSDynamicRule),
    ExecuteRule(RulesetsExecuteRule),
    ForceConnectionCloseRule(RulesetsForceConnectionCloseRule),
    JsChallengeRule(RulesetsJsChallengeRule),
    LogRule(RulesetsLogRule),
    LogCustomFieldRule(RulesetsLogCustomFieldRule),
    ManagedChallengeRule(RulesetsManagedChallengeRule),
    RedirectRule(RulesetsRedirectRule),
    RewriteRule(RulesetsRewriteRule),
    RouteRule(RulesetsRouteRule),
    ScoreRule(RulesetsScoreRule),
    ServeErrorRule(RulesetsServeErrorRule),
    SetCacheControlRule(RulesetsSetCacheControlRule),
    SetCacheSettingsRule(RulesetsSetCacheSettingsRule),
    SetCacheTagsRule(RulesetsSetCacheTagsRule),
    SetConfigRule(RulesetsSetConfigRule),
    SkipRule(RulesetsSkipRule),
    TransformResponseHtmlRule(RulesetsTransformResponseHtmlRule),
}
impl ::std::convert::From<RulesetsBlockRule> for RulesetsRequestRule {
    fn from(value: RulesetsBlockRule) -> Self {
        Self::BlockRule(value)
    }
}
impl ::std::convert::From<RulesetsChallengeRule> for RulesetsRequestRule {
    fn from(value: RulesetsChallengeRule) -> Self {
        Self::ChallengeRule(value)
    }
}
impl ::std::convert::From<RulesetsCompressResponseRule> for RulesetsRequestRule {
    fn from(value: RulesetsCompressResponseRule) -> Self {
        Self::CompressResponseRule(value)
    }
}
impl ::std::convert::From<RulesetsDDoSDynamicRule> for RulesetsRequestRule {
    fn from(value: RulesetsDDoSDynamicRule) -> Self {
        Self::DDoSDynamicRule(value)
    }
}
impl ::std::convert::From<RulesetsExecuteRule> for RulesetsRequestRule {
    fn from(value: RulesetsExecuteRule) -> Self {
        Self::ExecuteRule(value)
    }
}
impl ::std::convert::From<RulesetsForceConnectionCloseRule> for RulesetsRequestRule {
    fn from(value: RulesetsForceConnectionCloseRule) -> Self {
        Self::ForceConnectionCloseRule(value)
    }
}
impl ::std::convert::From<RulesetsJsChallengeRule> for RulesetsRequestRule {
    fn from(value: RulesetsJsChallengeRule) -> Self {
        Self::JsChallengeRule(value)
    }
}
impl ::std::convert::From<RulesetsLogRule> for RulesetsRequestRule {
    fn from(value: RulesetsLogRule) -> Self {
        Self::LogRule(value)
    }
}
impl ::std::convert::From<RulesetsLogCustomFieldRule> for RulesetsRequestRule {
    fn from(value: RulesetsLogCustomFieldRule) -> Self {
        Self::LogCustomFieldRule(value)
    }
}
impl ::std::convert::From<RulesetsManagedChallengeRule> for RulesetsRequestRule {
    fn from(value: RulesetsManagedChallengeRule) -> Self {
        Self::ManagedChallengeRule(value)
    }
}
impl ::std::convert::From<RulesetsRedirectRule> for RulesetsRequestRule {
    fn from(value: RulesetsRedirectRule) -> Self {
        Self::RedirectRule(value)
    }
}
impl ::std::convert::From<RulesetsRewriteRule> for RulesetsRequestRule {
    fn from(value: RulesetsRewriteRule) -> Self {
        Self::RewriteRule(value)
    }
}
impl ::std::convert::From<RulesetsRouteRule> for RulesetsRequestRule {
    fn from(value: RulesetsRouteRule) -> Self {
        Self::RouteRule(value)
    }
}
impl ::std::convert::From<RulesetsScoreRule> for RulesetsRequestRule {
    fn from(value: RulesetsScoreRule) -> Self {
        Self::ScoreRule(value)
    }
}
impl ::std::convert::From<RulesetsServeErrorRule> for RulesetsRequestRule {
    fn from(value: RulesetsServeErrorRule) -> Self {
        Self::ServeErrorRule(value)
    }
}
impl ::std::convert::From<RulesetsSetCacheControlRule> for RulesetsRequestRule {
    fn from(value: RulesetsSetCacheControlRule) -> Self {
        Self::SetCacheControlRule(value)
    }
}
impl ::std::convert::From<RulesetsSetCacheSettingsRule> for RulesetsRequestRule {
    fn from(value: RulesetsSetCacheSettingsRule) -> Self {
        Self::SetCacheSettingsRule(value)
    }
}
impl ::std::convert::From<RulesetsSetCacheTagsRule> for RulesetsRequestRule {
    fn from(value: RulesetsSetCacheTagsRule) -> Self {
        Self::SetCacheTagsRule(value)
    }
}
impl ::std::convert::From<RulesetsSetConfigRule> for RulesetsRequestRule {
    fn from(value: RulesetsSetConfigRule) -> Self {
        Self::SetConfigRule(value)
    }
}
impl ::std::convert::From<RulesetsSkipRule> for RulesetsRequestRule {
    fn from(value: RulesetsSkipRule) -> Self {
        Self::SkipRule(value)
    }
}
impl ::std::convert::From<RulesetsTransformResponseHtmlRule> for RulesetsRequestRule {
    fn from(value: RulesetsTransformResponseHtmlRule) -> Self {
        Self::TransformResponseHtmlRule(value)
    }
}
///The list of rules in the ruleset.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsRequestRules(pub ::std::vec::Vec<RulesetsRequestRule>);
impl ::std::ops::Deref for RulesetsRequestRules {
    type Target = ::std::vec::Vec<RulesetsRequestRule>;
    fn deref(&self) -> &::std::vec::Vec<RulesetsRequestRule> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRequestRules>
for ::std::vec::Vec<RulesetsRequestRule> {
    fn from(value: RulesetsRequestRules) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<RulesetsRequestRule>>
for RulesetsRequestRules {
    fn from(value: ::std::vec::Vec<RulesetsRequestRule>) -> Self {
        Self(value)
    }
}
///A response object.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponse {
    pub errors: RulesetsErrors,
    pub messages: RulesetsMessages,
    ///A result.
    pub result: ::serde_json::Value,
    ///Whether the API call was successful.
    pub success: bool,
}
///`RulesetsResponseRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum RulesetsResponseRule {
    Variant0 {
        action: RulesetsResponseRuleVariant0Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant0ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant1 {
        action: RulesetsResponseRuleVariant1Action,
        ///The parameters configuring the rule's action.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant2 {
        action: RulesetsResponseRuleVariant2Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant2ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant3 {
        action: RulesetsResponseRuleVariant3Action,
        ///The parameters configuring the rule's action.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant4 {
        action: RulesetsResponseRuleVariant4Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant4ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant5 {
        action: RulesetsResponseRuleVariant5Action,
        ///The parameters configuring the rule's action.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant6 {
        action: RulesetsResponseRuleVariant6Action,
        ///The parameters configuring the rule's action.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant7 {
        action: RulesetsResponseRuleVariant7Action,
        ///The parameters configuring the rule's action.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant8 {
        action: RulesetsResponseRuleVariant8Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant8ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant9 {
        action: RulesetsResponseRuleVariant9Action,
        ///The parameters configuring the rule's action.
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant10 {
        action: RulesetsResponseRuleVariant10Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant10ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant11 {
        action: RulesetsResponseRuleVariant11Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant11ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant12 {
        action: RulesetsResponseRuleVariant12Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant12ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant13 {
        action: RulesetsResponseRuleVariant13Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant13ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant14 {
        action: RulesetsResponseRuleVariant14Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant14ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant15 {
        action: RulesetsResponseRuleVariant15Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant15ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant16 {
        action: RulesetsResponseRuleVariant16Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant16ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant17 {
        action: RulesetsResponseRuleVariant17Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant17ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant18 {
        action: RulesetsResponseRuleVariant18Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant18ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant19 {
        action: RulesetsResponseRuleVariant19Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant19ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
    Variant20 {
        action: RulesetsResponseRuleVariant20Action,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        action_parameters: ::std::option::Option<
            RulesetsResponseRuleVariant20ActionParameters,
        >,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        categories: ::std::option::Option<RulesetsRuleCategories>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        enabled: RulesetsRuleEnabled,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        exposed_credential_check: ::std::option::Option<
            RulesetsRuleExposedCredentialCheck,
        >,
        ///The expression defining which traffic will match the rule.
        expression: Expression,
        id: RulesetsRuleId,
        ///The timestamp of when the rule was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        logging: ::std::option::Option<RulesetsRuleLogging>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
        ///The reference of the rule (the rule's ID by default).
        #[serde(rename = "ref")]
        ref_: Ref,
        ///The version of the rule.
        version: Version,
    },
}
///`RulesetsResponseRuleVariant0Action`
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
pub enum RulesetsResponseRuleVariant0Action {
    #[serde(rename = "block")]
    Block,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant0Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Block => f.write_str("block"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant0Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant0Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant0Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant0ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant0ActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response: ::std::option::Option<Response>,
}
///`RulesetsResponseRuleVariant10Action`
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
pub enum RulesetsResponseRuleVariant10Action {
    #[serde(rename = "redirect")]
    Redirect,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant10Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Redirect => f.write_str("redirect"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant10Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "redirect" => Ok(Self::Redirect),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant10Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant10Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant10ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant10ActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub from_list: ::std::option::Option<RulesetsRedirectFromList>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub from_value: ::std::option::Option<RulesetsRedirectFromValue>,
}
///`RulesetsResponseRuleVariant11Action`
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
pub enum RulesetsResponseRuleVariant11Action {
    #[serde(rename = "rewrite")]
    Rewrite,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant11Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Rewrite => f.write_str("rewrite"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant11Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "rewrite" => Ok(Self::Rewrite),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant11Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant11Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant11ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant11ActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub headers: ::std::option::Option<RulesetsRewriteHeaders>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub uri: ::std::option::Option<RulesetsRewriteUri>,
}
///`RulesetsResponseRuleVariant12Action`
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
pub enum RulesetsResponseRuleVariant12Action {
    #[serde(rename = "route")]
    Route,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant12Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Route => f.write_str("route"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant12Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "route" => Ok(Self::Route),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant12Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant12Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant12ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant12ActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub host_header: ::std::option::Option<RulesetsRouteHostHeader>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub origin: ::std::option::Option<RulesetsRouteOrigin>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sni: ::std::option::Option<RulesetsRouteSni>,
}
///`RulesetsResponseRuleVariant13Action`
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
pub enum RulesetsResponseRuleVariant13Action {
    #[serde(rename = "score")]
    Score,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant13Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Score => f.write_str("score"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant13Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "score" => Ok(Self::Score),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant13Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant13Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant13ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant13ActionParameters {
    pub increment: RulesetsScoreIncrement,
}
///`RulesetsResponseRuleVariant14Action`
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
pub enum RulesetsResponseRuleVariant14Action {
    #[serde(rename = "serve_error")]
    ServeError,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant14Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ServeError => f.write_str("serve_error"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant14Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "serve_error" => Ok(Self::ServeError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant14Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant14Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant14ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum RulesetsResponseRuleVariant14ActionParameters {
    Variant0 {
        content: RulesetsServeErrorContent,
        content_type: RulesetsServeErrorContentType,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        status_code: ::std::option::Option<RulesetsServeErrorStatusCode>,
    },
    Variant1 {
        asset_name: RulesetsServeErrorAssetName,
        content_type: RulesetsServeErrorContentType,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        status_code: ::std::option::Option<RulesetsServeErrorStatusCode>,
    },
}
///`RulesetsResponseRuleVariant15Action`
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
pub enum RulesetsResponseRuleVariant15Action {
    #[serde(rename = "set_cache_control")]
    SetCacheControl,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant15Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetCacheControl => f.write_str("set_cache_control"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant15Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set_cache_control" => Ok(Self::SetCacheControl),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant15Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant15Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant15ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant15ActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub immutable: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "max-age", skip_serializing_if = "::std::option::Option::is_none")]
    pub max_age: ::std::option::Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(
        rename = "must-revalidate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub must_revalidate: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(
        rename = "must-understand",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub must_understand: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "no-cache", skip_serializing_if = "::std::option::Option::is_none")]
    pub no_cache: ::std::option::Option<RulesetsSetCacheControlDirectiveWithQualifiers>,
    #[serde(rename = "no-store", skip_serializing_if = "::std::option::Option::is_none")]
    pub no_store: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(
        rename = "no-transform",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub no_transform: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub private: ::std::option::Option<RulesetsSetCacheControlDirectiveWithQualifiers>,
    #[serde(
        rename = "proxy-revalidate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub proxy_revalidate: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub public: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "s-maxage", skip_serializing_if = "::std::option::Option::is_none")]
    pub s_maxage: ::std::option::Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(
        rename = "stale-if-error",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stale_if_error: ::std::option::Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(
        rename = "stale-while-revalidate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stale_while_revalidate: ::std::option::Option<
        RulesetsSetCacheControlDirectiveWithValue,
    >,
}
///`RulesetsResponseRuleVariant16Action`
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
pub enum RulesetsResponseRuleVariant16Action {
    #[serde(rename = "set_cache_settings")]
    SetCacheSettings,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant16Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetCacheSettings => f.write_str("set_cache_settings"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant16Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set_cache_settings" => Ok(Self::SetCacheSettings),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant16Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant16Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant16ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant16ActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub additional_cacheable_ports: ::std::option::Option<
        RulesetsSetCacheSettingsAdditionalCacheablePorts,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub browser_ttl: ::std::option::Option<RulesetsSetCacheSettingsBrowserTtl>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache: ::std::option::Option<RulesetsSetCacheSettingsCache>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache_key: ::std::option::Option<RulesetsSetCacheSettingsCacheKey>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache_reserve: ::std::option::Option<RulesetsSetCacheSettingsCacheReserve>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub edge_ttl: ::std::option::Option<RulesetsSetCacheSettingsEdgeTtl>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub origin_cache_control: ::std::option::Option<
        RulesetsSetCacheSettingsOriginCacheControl,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub origin_error_page_passthru: ::std::option::Option<
        RulesetsSetCacheSettingsOriginErrorPagePassthru,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub origin_range_requests: ::std::option::Option<
        RulesetsSetCacheSettingsOriginRangeRequests,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub read_timeout: ::std::option::Option<RulesetsSetCacheSettingsReadTimeout>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub respect_strong_etags: ::std::option::Option<
        RulesetsSetCacheSettingsRespectStrongEtags,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub serve_stale: ::std::option::Option<RulesetsSetCacheSettingsServeStale>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shared_dictionary: ::std::option::Option<
        RulesetsSetCacheSettingsSharedDictionary,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strip_etags: ::std::option::Option<RulesetsSetCacheSettingsStripETags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strip_last_modified: ::std::option::Option<
        RulesetsSetCacheSettingsStripLastModified,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strip_set_cookie: ::std::option::Option<RulesetsSetCacheSettingsStripSetCookie>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vary: ::std::option::Option<RulesetsSetCacheSettingsVary>,
}
///`RulesetsResponseRuleVariant17Action`
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
pub enum RulesetsResponseRuleVariant17Action {
    #[serde(rename = "set_cache_tags")]
    SetCacheTags,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant17Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetCacheTags => f.write_str("set_cache_tags"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant17Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set_cache_tags" => Ok(Self::SetCacheTags),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant17Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant17Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant17ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum RulesetsResponseRuleVariant17ActionParameters {
    AddCacheTagsValues {
        operation: AddCacheTagsValuesOperation,
        values: RulesetsSetCacheTagsValues,
    },
    AddCacheTagsExpression {
        expression: RulesetsSetCacheTagsExpression,
        operation: AddCacheTagsExpressionOperation,
    },
    RemoveCacheTagsValues {
        operation: RemoveCacheTagsValuesOperation,
        values: RulesetsSetCacheTagsValues,
    },
    RemoveCacheTagsExpression {
        expression: RulesetsSetCacheTagsExpression,
        operation: RemoveCacheTagsExpressionOperation,
    },
    SetCacheTagsValues {
        operation: SetCacheTagsValuesOperation,
        values: RulesetsSetCacheTagsValues,
    },
    SetCacheTagsExpression {
        expression: RulesetsSetCacheTagsExpression,
        operation: SetCacheTagsExpressionOperation,
    },
}
///`RulesetsResponseRuleVariant18Action`
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
pub enum RulesetsResponseRuleVariant18Action {
    #[serde(rename = "set_config")]
    SetConfig,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant18Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetConfig => f.write_str("set_config"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant18Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set_config" => Ok(Self::SetConfig),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant18Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant18Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant18ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant18ActionParameters {
    ///Whether to enable Automatic HTTPS Rewrites.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub automatic_https_rewrites: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub autominify: ::std::option::Option<RulesetsSetConfigAutominify>,
    ///Whether to enable Browser Integrity Check (BIC).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bic: ::std::option::Option<bool>,
    ///Whether to enable content conversion (e.g., HTML to Markdown).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content_converter: ::std::option::Option<bool>,
    ///Whether to disable Cloudflare Apps.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disable_apps: ::std::option::Option<bool>,
    ///Whether to disable Pay Per Crawl.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disable_pay_per_crawl: ::std::option::Option<bool>,
    ///Whether to disable Real User Monitoring (RUM).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disable_rum: ::std::option::Option<bool>,
    ///Whether to disable Zaraz.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disable_zaraz: ::std::option::Option<bool>,
    ///Whether to enable Email Obfuscation.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email_obfuscation: ::std::option::Option<bool>,
    ///Whether to enable Cloudflare Fonts.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fonts: ::std::option::Option<bool>,
    ///Whether to enable Hotlink Protection.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hotlink_protection: ::std::option::Option<bool>,
    ///Whether to enable Mirage.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mirage: ::std::option::Option<bool>,
    ///Whether to enable Opportunistic Encryption.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub opportunistic_encryption: ::std::option::Option<bool>,
    ///The Polish level to configure.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub polish: ::std::option::Option<Polish>,
    ///Whether to redirect verified AI training crawlers to canonical URLs found in the HTML response.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub redirects_for_ai_training: ::std::option::Option<bool>,
    ///The request body buffering mode.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub request_body_buffering: ::std::option::Option<RequestBodyBuffering>,
    ///The response body buffering mode.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_body_buffering: ::std::option::Option<ResponseBodyBuffering>,
    ///Whether to enable Rocket Loader.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rocket_loader: ::std::option::Option<bool>,
    ///The Security Level to configure.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub security_level: ::std::option::Option<SecurityLevel>,
    ///Whether to enable Server-Side Excludes.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub server_side_excludes: ::std::option::Option<bool>,
    ///The SSL level to configure.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ssl: ::std::option::Option<Ssl>,
    ///Whether to enable Signed Exchanges (SXG).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sxg: ::std::option::Option<bool>,
}
///`RulesetsResponseRuleVariant19Action`
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
pub enum RulesetsResponseRuleVariant19Action {
    #[serde(rename = "skip")]
    Skip,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant19Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Skip => f.write_str("skip"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant19Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "skip" => Ok(Self::Skip),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant19Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant19Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant19ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant19ActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub phase: ::std::option::Option<RulesetsSkipPhase>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub phases: ::std::option::Option<RulesetsSkipPhases>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub products: ::std::option::Option<RulesetsSkipProducts>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rules: ::std::option::Option<RulesetsSkipRules>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ruleset: ::std::option::Option<RulesetsSkipRuleset>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rulesets: ::std::option::Option<RulesetsSkipRulesets>,
}
///`RulesetsResponseRuleVariant1Action`
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
pub enum RulesetsResponseRuleVariant1Action {
    #[serde(rename = "challenge")]
    Challenge,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant1Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Challenge => f.write_str("challenge"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant1Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "challenge" => Ok(Self::Challenge),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant1Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant1Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant20Action`
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
pub enum RulesetsResponseRuleVariant20Action {
    #[serde(rename = "transform_response_html")]
    TransformResponseHtml,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant20Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TransformResponseHtml => f.write_str("transform_response_html"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant20Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transform_response_html" => Ok(Self::TransformResponseHtml),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant20Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant20Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant20ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant20ActionParameters {
    ///Enables the link maze transformation on the response.
    pub link_maze: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`RulesetsResponseRuleVariant2Action`
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
pub enum RulesetsResponseRuleVariant2Action {
    #[serde(rename = "compress_response")]
    CompressResponse,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant2Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CompressResponse => f.write_str("compress_response"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant2Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "compress_response" => Ok(Self::CompressResponse),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant2Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant2Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant2ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant2ActionParameters {
    ///Custom order for compression algorithms.
    pub algorithms: ::std::vec::Vec<Algorithm>,
}
///`RulesetsResponseRuleVariant3Action`
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
pub enum RulesetsResponseRuleVariant3Action {
    #[serde(rename = "ddos_dynamic")]
    DdosDynamic,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant3Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DdosDynamic => f.write_str("ddos_dynamic"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant3Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ddos_dynamic" => Ok(Self::DdosDynamic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant3Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant3Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant4Action`
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
pub enum RulesetsResponseRuleVariant4Action {
    #[serde(rename = "execute")]
    Execute,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant4Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Execute => f.write_str("execute"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant4Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "execute" => Ok(Self::Execute),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant4Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant4Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant4ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant4ActionParameters {
    pub id: RulesetsRulesetId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub matched_data: ::std::option::Option<RulesetsExecuteMatchedData>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub overrides: ::std::option::Option<RulesetsExecuteOverrides>,
}
///`RulesetsResponseRuleVariant5Action`
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
pub enum RulesetsResponseRuleVariant5Action {
    #[serde(rename = "force_connection_close")]
    ForceConnectionClose,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant5Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ForceConnectionClose => f.write_str("force_connection_close"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant5Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "force_connection_close" => Ok(Self::ForceConnectionClose),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant5Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant5Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant6Action`
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
pub enum RulesetsResponseRuleVariant6Action {
    #[serde(rename = "js_challenge")]
    JsChallenge,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant6Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::JsChallenge => f.write_str("js_challenge"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant6Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "js_challenge" => Ok(Self::JsChallenge),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant6Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant6Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant7Action`
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
pub enum RulesetsResponseRuleVariant7Action {
    #[serde(rename = "log")]
    Log,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant7Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Log => f.write_str("log"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant7Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "log" => Ok(Self::Log),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant7Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant7Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant8Action`
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
pub enum RulesetsResponseRuleVariant8Action {
    #[serde(rename = "log_custom_field")]
    LogCustomField,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant8Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LogCustomField => f.write_str("log_custom_field"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant8Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "log_custom_field" => Ok(Self::LogCustomField),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant8Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant8Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsResponseRuleVariant8ActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsResponseRuleVariant8ActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cookie_fields: ::std::option::Option<RulesetsLogCustomFieldCookieFields>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub raw_response_fields: ::std::option::Option<
        RulesetsLogCustomFieldRawResponseFields,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub request_fields: ::std::option::Option<RulesetsLogCustomFieldRequestFields>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_fields: ::std::option::Option<RulesetsLogCustomFieldResponseFields>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transformed_request_fields: ::std::option::Option<
        RulesetsLogCustomFieldTransformedRequestFields,
    >,
}
///`RulesetsResponseRuleVariant9Action`
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
pub enum RulesetsResponseRuleVariant9Action {
    #[serde(rename = "managed_challenge")]
    ManagedChallenge,
}
impl ::std::fmt::Display for RulesetsResponseRuleVariant9Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ManagedChallenge => f.write_str("managed_challenge"),
        }
    }
}
impl ::std::str::FromStr for RulesetsResponseRuleVariant9Action {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "managed_challenge" => Ok(Self::ManagedChallenge),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsResponseRuleVariant9Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsResponseRuleVariant9Action {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The list of rules in the ruleset.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsResponseRules(pub ::std::vec::Vec<RulesetsResponseRule>);
impl ::std::ops::Deref for RulesetsResponseRules {
    type Target = ::std::vec::Vec<RulesetsResponseRule>;
    fn deref(&self) -> &::std::vec::Vec<RulesetsResponseRule> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsResponseRules>
for ::std::vec::Vec<RulesetsResponseRule> {
    fn from(value: RulesetsResponseRules) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<RulesetsResponseRule>>
for RulesetsResponseRules {
    fn from(value: ::std::vec::Vec<RulesetsResponseRule>) -> Self {
        Self(value)
    }
}
///An expression that evaluates to a value for the header.
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
pub struct RulesetsRewriteHeaderExpression(::std::string::String);
impl ::std::ops::Deref for RulesetsRewriteHeaderExpression {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRewriteHeaderExpression> for ::std::string::String {
    fn from(value: RulesetsRewriteHeaderExpression) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsRewriteHeaderExpression {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRewriteHeaderExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRewriteHeaderExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsRewriteHeaderExpression {
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
///The operation to perform on the header.
#[derive(
    ::serde::Deserialize,
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
pub struct RulesetsRewriteHeaderOperation(pub ::std::string::String);
impl ::std::ops::Deref for RulesetsRewriteHeaderOperation {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRewriteHeaderOperation> for ::std::string::String {
    fn from(value: RulesetsRewriteHeaderOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for RulesetsRewriteHeaderOperation {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsRewriteHeaderOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsRewriteHeaderOperation {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///A static value for the header.
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
pub struct RulesetsRewriteHeaderValue(::std::string::String);
impl ::std::ops::Deref for RulesetsRewriteHeaderValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRewriteHeaderValue> for ::std::string::String {
    fn from(value: RulesetsRewriteHeaderValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsRewriteHeaderValue {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRewriteHeaderValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRewriteHeaderValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsRewriteHeaderValue {
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
///A map of headers to rewrite.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsRewriteHeaders(
    pub ::std::collections::HashMap<::std::string::String, RulesetsRewriteHeadersValue>,
);
impl ::std::ops::Deref for RulesetsRewriteHeaders {
    type Target = ::std::collections::HashMap<
        ::std::string::String,
        RulesetsRewriteHeadersValue,
    >;
    fn deref(
        &self,
    ) -> &::std::collections::HashMap<
        ::std::string::String,
        RulesetsRewriteHeadersValue,
    > {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRewriteHeaders>
for ::std::collections::HashMap<::std::string::String, RulesetsRewriteHeadersValue> {
    fn from(value: RulesetsRewriteHeaders) -> Self {
        value.0
    }
}
impl ::std::convert::From<
    ::std::collections::HashMap<::std::string::String, RulesetsRewriteHeadersValue>,
> for RulesetsRewriteHeaders {
    fn from(
        value: ::std::collections::HashMap<
            ::std::string::String,
            RulesetsRewriteHeadersValue,
        >,
    ) -> Self {
        Self(value)
    }
}
///`RulesetsRewriteHeadersValue`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum RulesetsRewriteHeadersValue {
    AddStaticHeader {
        operation: AddStaticHeaderOperation,
        value: RulesetsRewriteHeaderValue,
    },
    AddDynamicHeader {
        expression: RulesetsRewriteHeaderExpression,
        operation: AddDynamicHeaderOperation,
    },
    SetStaticHeader {
        operation: SetStaticHeaderOperation,
        value: RulesetsRewriteHeaderValue,
    },
    SetDynamicHeader {
        expression: RulesetsRewriteHeaderExpression,
        operation: SetDynamicHeaderOperation,
    },
    RemoveHeader { operation: RemoveHeaderOperation },
}
///`RulesetsRewriteRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRewriteRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsRewriteRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<RulesetsRewriteRuleActionParameters>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsRewriteRuleAction`
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
pub enum RulesetsRewriteRuleAction {
    #[serde(rename = "rewrite")]
    Rewrite,
}
impl ::std::fmt::Display for RulesetsRewriteRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Rewrite => f.write_str("rewrite"),
        }
    }
}
impl ::std::str::FromStr for RulesetsRewriteRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "rewrite" => Ok(Self::Rewrite),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRewriteRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRewriteRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsRewriteRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRewriteRuleActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub headers: ::std::option::Option<RulesetsRewriteHeaders>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub uri: ::std::option::Option<RulesetsRewriteUri>,
}
///`RulesetsRewriteUri`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum RulesetsRewriteUri {
    Variant0 {
        ///Whether to propagate the rewritten URI to origin.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        origin: ::std::option::Option<bool>,
        path: RulesetsRewriteUriPath,
    },
    Variant1 {
        ///Whether to propagate the rewritten URI to origin.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        origin: ::std::option::Option<bool>,
        query: RulesetsRewriteUriQuery,
    },
}
///A URI path rewrite.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRewriteUriPath {
    ///An expression that evaluates to a value to rewrite the URI path to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<PathExpression>,
    ///A value to rewrite the URI path to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<PathValue>,
}
///A URI query rewrite.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRewriteUriQuery {
    ///An expression that evaluates to a value to rewrite the URI query to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<QueryExpression>,
    ///A value to rewrite the URI query to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
///A value to rewrite the HTTP host header to.
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
pub struct RulesetsRouteHostHeader(::std::string::String);
impl ::std::ops::Deref for RulesetsRouteHostHeader {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRouteHostHeader> for ::std::string::String {
    fn from(value: RulesetsRouteHostHeader) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsRouteHostHeader {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRouteHostHeader {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRouteHostHeader {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsRouteHostHeader {
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
///An origin to route to.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRouteOrigin {
    ///A resolved host to route to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub host: ::std::option::Option<Host>,
    ///A destination port to route to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub port: ::std::option::Option<::std::num::NonZeroU64>,
}
///`RulesetsRouteRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRouteRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsRouteRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<RulesetsRouteRuleActionParameters>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsRouteRuleAction`
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
pub enum RulesetsRouteRuleAction {
    #[serde(rename = "route")]
    Route,
}
impl ::std::fmt::Display for RulesetsRouteRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Route => f.write_str("route"),
        }
    }
}
impl ::std::str::FromStr for RulesetsRouteRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "route" => Ok(Self::Route),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRouteRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRouteRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsRouteRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRouteRuleActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub host_header: ::std::option::Option<RulesetsRouteHostHeader>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub origin: ::std::option::Option<RulesetsRouteOrigin>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sni: ::std::option::Option<RulesetsRouteSni>,
}
///A Server Name Indication (SNI) override.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRouteSni {
    ///A value to override the SNI to.
    pub value: Value,
}
///`RulesetsRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsRuleAction>,
    ///The parameters configuring the rule's action.
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub action_parameters: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    ///An informative description of the rule.
    #[serde(default)]
    pub description: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///The action to perform when the rule matches.
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
pub struct RulesetsRuleAction(::std::string::String);
impl ::std::ops::Deref for RulesetsRuleAction {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRuleAction> for ::std::string::String {
    fn from(value: RulesetsRuleAction) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-z_]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z_]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsRuleAction {
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
///The categories of the rule.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsRuleCategories(pub ::std::vec::Vec<RulesetsRuleCategory>);
impl ::std::ops::Deref for RulesetsRuleCategories {
    type Target = ::std::vec::Vec<RulesetsRuleCategory>;
    fn deref(&self) -> &::std::vec::Vec<RulesetsRuleCategory> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRuleCategories>
for ::std::vec::Vec<RulesetsRuleCategory> {
    fn from(value: RulesetsRuleCategories) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<RulesetsRuleCategory>>
for RulesetsRuleCategories {
    fn from(value: ::std::vec::Vec<RulesetsRuleCategory>) -> Self {
        Self(value)
    }
}
///The category of a rule.
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
pub struct RulesetsRuleCategory(::std::string::String);
impl ::std::ops::Deref for RulesetsRuleCategory {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRuleCategory> for ::std::string::String {
    fn from(value: RulesetsRuleCategory) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsRuleCategory {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRuleCategory {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRuleCategory {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsRuleCategory {
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
///Whether the rule should be executed.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsRuleEnabled(pub bool);
impl ::std::ops::Deref for RulesetsRuleEnabled {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRuleEnabled> for bool {
    fn from(value: RulesetsRuleEnabled) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for RulesetsRuleEnabled {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsRuleEnabled {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsRuleEnabled {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRuleEnabled {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RulesetsRuleEnabled {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///Configuration for exposed credential checking.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRuleExposedCredentialCheck {
    ///An expression that selects the password used in the credentials check.
    pub password_expression: PasswordExpression,
    ///An expression that selects the user ID used in the credentials check.
    pub username_expression: UsernameExpression,
}
///The unique ID of the rule.
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
pub struct RulesetsRuleId(::std::string::String);
impl ::std::ops::Deref for RulesetsRuleId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRuleId> for ::std::string::String {
    fn from(value: RulesetsRuleId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsRuleId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[0-9a-f]{32}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9a-f]{32}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRuleId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRuleId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsRuleId {
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
///An object configuring the rule's logging behavior.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRuleLogging {
    ///Whether to generate a log when the rule matches.
    pub enabled: bool,
}
///An object configuring the rule's rate limit behavior.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRuleRatelimit {
    ///Characteristics of the request on which the rate limit counter will be incremented.
    pub characteristics: ::std::vec::Vec<Characteristic>,
    ///An expression that defines when the rate limit counter should be incremented. It defaults to the same as the rule's expression.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub counting_expression: ::std::option::Option<CountingExpression>,
    ///Period of time in seconds after which the action will be disabled following its first execution.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mitigation_timeout: ::std::option::Option<i64>,
    ///Period in seconds over which the counter is being incremented.
    pub period: u64,
    ///The threshold of requests per period after which the action will be executed for the first time.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub requests_per_period: ::std::option::Option<::std::num::NonZeroU64>,
    ///Whether counting is only performed when an origin is reached.
    #[serde(default)]
    pub requests_to_origin: bool,
    ///The score threshold per period for which the action will be executed the first time.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub score_per_period: ::std::option::Option<i64>,
    ///A response header name provided by the origin, which contains the score to increment rate limit counter with.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub score_response_header_name: ::std::option::Option<ScoreResponseHeaderName>,
}
///A ruleset object.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsRuleset {
    ///An informative description of the ruleset.
    #[serde(default)]
    pub description: ::std::string::String,
    pub id: RulesetsRulesetId,
    ///The timestamp of when the ruleset was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    ///The human-readable name of the ruleset.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
    pub version: RulesetsRulesetVersion,
}
///The unique ID of the ruleset.
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
pub struct RulesetsRulesetId(::std::string::String);
impl ::std::ops::Deref for RulesetsRulesetId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRulesetId> for ::std::string::String {
    fn from(value: RulesetsRulesetId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsRulesetId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[0-9a-f]{32}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9a-f]{32}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRulesetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRulesetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsRulesetId {
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
///The kind of the ruleset.
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
pub enum RulesetsRulesetKind {
    #[serde(rename = "managed")]
    Managed,
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "root")]
    Root,
    #[serde(rename = "zone")]
    Zone,
}
impl ::std::fmt::Display for RulesetsRulesetKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Managed => f.write_str("managed"),
            Self::Custom => f.write_str("custom"),
            Self::Root => f.write_str("root"),
            Self::Zone => f.write_str("zone"),
        }
    }
}
impl ::std::str::FromStr for RulesetsRulesetKind {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "managed" => Ok(Self::Managed),
            "custom" => Ok(Self::Custom),
            "root" => Ok(Self::Root),
            "zone" => Ok(Self::Zone),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRulesetKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRulesetKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The phase of the ruleset.
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
pub enum RulesetsRulesetPhase {
    #[serde(rename = "ddos_l4")]
    DdosL4,
    #[serde(rename = "ddos_l7")]
    DdosL7,
    #[serde(rename = "http_config_settings")]
    HttpConfigSettings,
    #[serde(rename = "http_custom_errors")]
    HttpCustomErrors,
    #[serde(rename = "http_log_custom_fields")]
    HttpLogCustomFields,
    #[serde(rename = "http_ratelimit")]
    HttpRatelimit,
    #[serde(rename = "http_request_cache_settings")]
    HttpRequestCacheSettings,
    #[serde(rename = "http_request_dynamic_redirect")]
    HttpRequestDynamicRedirect,
    #[serde(rename = "http_request_firewall_custom")]
    HttpRequestFirewallCustom,
    #[serde(rename = "http_request_firewall_managed")]
    HttpRequestFirewallManaged,
    #[serde(rename = "http_request_late_transform")]
    HttpRequestLateTransform,
    #[serde(rename = "http_request_origin")]
    HttpRequestOrigin,
    #[serde(rename = "http_request_redirect")]
    HttpRequestRedirect,
    #[serde(rename = "http_request_sanitize")]
    HttpRequestSanitize,
    #[serde(rename = "http_request_sbfm")]
    HttpRequestSbfm,
    #[serde(rename = "http_request_transform")]
    HttpRequestTransform,
    #[serde(rename = "http_response_cache_settings")]
    HttpResponseCacheSettings,
    #[serde(rename = "http_response_compression")]
    HttpResponseCompression,
    #[serde(rename = "http_response_firewall_managed")]
    HttpResponseFirewallManaged,
    #[serde(rename = "http_response_headers_transform")]
    HttpResponseHeadersTransform,
    #[serde(rename = "magic_transit")]
    MagicTransit,
    #[serde(rename = "magic_transit_ids_managed")]
    MagicTransitIdsManaged,
    #[serde(rename = "magic_transit_managed")]
    MagicTransitManaged,
    #[serde(rename = "magic_transit_ratelimit")]
    MagicTransitRatelimit,
}
impl ::std::fmt::Display for RulesetsRulesetPhase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DdosL4 => f.write_str("ddos_l4"),
            Self::DdosL7 => f.write_str("ddos_l7"),
            Self::HttpConfigSettings => f.write_str("http_config_settings"),
            Self::HttpCustomErrors => f.write_str("http_custom_errors"),
            Self::HttpLogCustomFields => f.write_str("http_log_custom_fields"),
            Self::HttpRatelimit => f.write_str("http_ratelimit"),
            Self::HttpRequestCacheSettings => f.write_str("http_request_cache_settings"),
            Self::HttpRequestDynamicRedirect => {
                f.write_str("http_request_dynamic_redirect")
            }
            Self::HttpRequestFirewallCustom => {
                f.write_str("http_request_firewall_custom")
            }
            Self::HttpRequestFirewallManaged => {
                f.write_str("http_request_firewall_managed")
            }
            Self::HttpRequestLateTransform => f.write_str("http_request_late_transform"),
            Self::HttpRequestOrigin => f.write_str("http_request_origin"),
            Self::HttpRequestRedirect => f.write_str("http_request_redirect"),
            Self::HttpRequestSanitize => f.write_str("http_request_sanitize"),
            Self::HttpRequestSbfm => f.write_str("http_request_sbfm"),
            Self::HttpRequestTransform => f.write_str("http_request_transform"),
            Self::HttpResponseCacheSettings => {
                f.write_str("http_response_cache_settings")
            }
            Self::HttpResponseCompression => f.write_str("http_response_compression"),
            Self::HttpResponseFirewallManaged => {
                f.write_str("http_response_firewall_managed")
            }
            Self::HttpResponseHeadersTransform => {
                f.write_str("http_response_headers_transform")
            }
            Self::MagicTransit => f.write_str("magic_transit"),
            Self::MagicTransitIdsManaged => f.write_str("magic_transit_ids_managed"),
            Self::MagicTransitManaged => f.write_str("magic_transit_managed"),
            Self::MagicTransitRatelimit => f.write_str("magic_transit_ratelimit"),
        }
    }
}
impl ::std::str::FromStr for RulesetsRulesetPhase {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ddos_l4" => Ok(Self::DdosL4),
            "ddos_l7" => Ok(Self::DdosL7),
            "http_config_settings" => Ok(Self::HttpConfigSettings),
            "http_custom_errors" => Ok(Self::HttpCustomErrors),
            "http_log_custom_fields" => Ok(Self::HttpLogCustomFields),
            "http_ratelimit" => Ok(Self::HttpRatelimit),
            "http_request_cache_settings" => Ok(Self::HttpRequestCacheSettings),
            "http_request_dynamic_redirect" => Ok(Self::HttpRequestDynamicRedirect),
            "http_request_firewall_custom" => Ok(Self::HttpRequestFirewallCustom),
            "http_request_firewall_managed" => Ok(Self::HttpRequestFirewallManaged),
            "http_request_late_transform" => Ok(Self::HttpRequestLateTransform),
            "http_request_origin" => Ok(Self::HttpRequestOrigin),
            "http_request_redirect" => Ok(Self::HttpRequestRedirect),
            "http_request_sanitize" => Ok(Self::HttpRequestSanitize),
            "http_request_sbfm" => Ok(Self::HttpRequestSbfm),
            "http_request_transform" => Ok(Self::HttpRequestTransform),
            "http_response_cache_settings" => Ok(Self::HttpResponseCacheSettings),
            "http_response_compression" => Ok(Self::HttpResponseCompression),
            "http_response_firewall_managed" => Ok(Self::HttpResponseFirewallManaged),
            "http_response_headers_transform" => Ok(Self::HttpResponseHeadersTransform),
            "magic_transit" => Ok(Self::MagicTransit),
            "magic_transit_ids_managed" => Ok(Self::MagicTransitIdsManaged),
            "magic_transit_managed" => Ok(Self::MagicTransitManaged),
            "magic_transit_ratelimit" => Ok(Self::MagicTransitRatelimit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRulesetPhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRulesetPhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The version of the ruleset.
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
pub struct RulesetsRulesetVersion(::std::string::String);
impl ::std::ops::Deref for RulesetsRulesetVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsRulesetVersion> for ::std::string::String {
    fn from(value: RulesetsRulesetVersion) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsRulesetVersion {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[0-9]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsRulesetVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsRulesetVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsRulesetVersion {
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
///A delta to change the score by, which can be either positive or negative.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsScoreIncrement(pub i64);
impl ::std::ops::Deref for RulesetsScoreIncrement {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RulesetsScoreIncrement> for i64 {
    fn from(value: RulesetsScoreIncrement) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for RulesetsScoreIncrement {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsScoreIncrement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsScoreIncrement {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsScoreIncrement {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RulesetsScoreIncrement {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///`RulesetsScoreRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsScoreRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsScoreRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<RulesetsScoreRuleActionParameters>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsScoreRuleAction`
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
pub enum RulesetsScoreRuleAction {
    #[serde(rename = "score")]
    Score,
}
impl ::std::fmt::Display for RulesetsScoreRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Score => f.write_str("score"),
        }
    }
}
impl ::std::str::FromStr for RulesetsScoreRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "score" => Ok(Self::Score),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsScoreRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsScoreRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsScoreRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsScoreRuleActionParameters {
    pub increment: RulesetsScoreIncrement,
}
///The name of a custom asset to serve as the error response.
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
pub struct RulesetsServeErrorAssetName(::std::string::String);
impl ::std::ops::Deref for RulesetsServeErrorAssetName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsServeErrorAssetName> for ::std::string::String {
    fn from(value: RulesetsServeErrorAssetName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsServeErrorAssetName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsServeErrorAssetName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsServeErrorAssetName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsServeErrorAssetName {
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
///The response content.
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
pub struct RulesetsServeErrorContent(::std::string::String);
impl ::std::ops::Deref for RulesetsServeErrorContent {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsServeErrorContent> for ::std::string::String {
    fn from(value: RulesetsServeErrorContent) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsServeErrorContent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsServeErrorContent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsServeErrorContent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsServeErrorContent {
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
///The content type header to set with the error response.
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
pub enum RulesetsServeErrorContentType {
    #[serde(rename = "application/json")]
    ApplicationJson,
    #[serde(rename = "text/html")]
    TextHtml,
    #[serde(rename = "text/plain")]
    TextPlain,
    #[serde(rename = "text/xml")]
    TextXml,
}
impl ::std::fmt::Display for RulesetsServeErrorContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplicationJson => f.write_str("application/json"),
            Self::TextHtml => f.write_str("text/html"),
            Self::TextPlain => f.write_str("text/plain"),
            Self::TextXml => f.write_str("text/xml"),
        }
    }
}
impl ::std::str::FromStr for RulesetsServeErrorContentType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "application/json" => Ok(Self::ApplicationJson),
            "text/html" => Ok(Self::TextHtml),
            "text/plain" => Ok(Self::TextPlain),
            "text/xml" => Ok(Self::TextXml),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsServeErrorContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsServeErrorContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsServeErrorRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsServeErrorRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsServeErrorRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<RulesetsServeErrorRuleActionParameters>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsServeErrorRuleAction`
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
pub enum RulesetsServeErrorRuleAction {
    #[serde(rename = "serve_error")]
    ServeError,
}
impl ::std::fmt::Display for RulesetsServeErrorRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ServeError => f.write_str("serve_error"),
        }
    }
}
impl ::std::str::FromStr for RulesetsServeErrorRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "serve_error" => Ok(Self::ServeError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsServeErrorRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsServeErrorRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsServeErrorRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum RulesetsServeErrorRuleActionParameters {
    Variant0 {
        content: RulesetsServeErrorContent,
        content_type: RulesetsServeErrorContentType,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        status_code: ::std::option::Option<RulesetsServeErrorStatusCode>,
    },
    Variant1 {
        asset_name: RulesetsServeErrorAssetName,
        content_type: RulesetsServeErrorContentType,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        status_code: ::std::option::Option<RulesetsServeErrorStatusCode>,
    },
}
///The status code to use for the error.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsServeErrorStatusCode(pub i64);
impl ::std::ops::Deref for RulesetsServeErrorStatusCode {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RulesetsServeErrorStatusCode> for i64 {
    fn from(value: RulesetsServeErrorStatusCode) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for RulesetsServeErrorStatusCode {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsServeErrorStatusCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsServeErrorStatusCode {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsServeErrorStatusCode {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RulesetsServeErrorStatusCode {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///Whether the directive should only be applied to the Cloudflare CDN cache.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheControlCloudflareOnly(pub bool);
impl ::std::ops::Deref for RulesetsSetCacheControlCloudflareOnly {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheControlCloudflareOnly> for bool {
    fn from(value: RulesetsSetCacheControlCloudflareOnly) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for RulesetsSetCacheControlCloudflareOnly {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsSetCacheControlCloudflareOnly {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsSetCacheControlCloudflareOnly {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheControlCloudflareOnly {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RulesetsSetCacheControlCloudflareOnly {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///A cache-control directive configuration.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum RulesetsSetCacheControlDirective {
    SetDirective {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        cloudflare_only: ::std::option::Option<RulesetsSetCacheControlCloudflareOnly>,
        operation: SetDirectiveOperation,
    },
    RemoveDirective {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        cloudflare_only: ::std::option::Option<RulesetsSetCacheControlCloudflareOnly>,
        operation: RemoveDirectiveOperation,
    },
}
///A cache-control directive configuration that accepts optional qualifiers (header names).
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum RulesetsSetCacheControlDirectiveWithQualifiers {
    SetDirective {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        cloudflare_only: ::std::option::Option<RulesetsSetCacheControlCloudflareOnly>,
        operation: SetDirectiveOperation,
        ///Optional list of header names to qualify the directive (e.g., for "private" or "no-cache" directives).
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        qualifiers: ::std::option::Option<::std::vec::Vec<Qualifier>>,
    },
    RemoveDirective {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        cloudflare_only: ::std::option::Option<RulesetsSetCacheControlCloudflareOnly>,
        operation: RemoveDirectiveOperation,
    },
}
///A cache-control directive configuration that accepts a duration value in seconds.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum RulesetsSetCacheControlDirectiveWithValue {
    SetDirective {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        cloudflare_only: ::std::option::Option<RulesetsSetCacheControlCloudflareOnly>,
        operation: SetDirectiveOperation,
        ///The duration value in seconds for the directive.
        value: u64,
    },
    RemoveDirective {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        cloudflare_only: ::std::option::Option<RulesetsSetCacheControlCloudflareOnly>,
        operation: RemoveDirectiveOperation,
    },
}
///The operation to perform on the cache-control directive.
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
pub enum RulesetsSetCacheControlOperation {
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "remove")]
    Remove,
}
impl ::std::fmt::Display for RulesetsSetCacheControlOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Set => f.write_str("set"),
            Self::Remove => f.write_str("remove"),
        }
    }
}
impl ::std::str::FromStr for RulesetsSetCacheControlOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set" => Ok(Self::Set),
            "remove" => Ok(Self::Remove),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheControlOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsSetCacheControlOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsSetCacheControlRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheControlRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsSetCacheControlRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<
        RulesetsSetCacheControlRuleActionParameters,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsSetCacheControlRuleAction`
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
pub enum RulesetsSetCacheControlRuleAction {
    #[serde(rename = "set_cache_control")]
    SetCacheControl,
}
impl ::std::fmt::Display for RulesetsSetCacheControlRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetCacheControl => f.write_str("set_cache_control"),
        }
    }
}
impl ::std::str::FromStr for RulesetsSetCacheControlRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set_cache_control" => Ok(Self::SetCacheControl),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheControlRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsSetCacheControlRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsSetCacheControlRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheControlRuleActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub immutable: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "max-age", skip_serializing_if = "::std::option::Option::is_none")]
    pub max_age: ::std::option::Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(
        rename = "must-revalidate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub must_revalidate: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(
        rename = "must-understand",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub must_understand: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "no-cache", skip_serializing_if = "::std::option::Option::is_none")]
    pub no_cache: ::std::option::Option<RulesetsSetCacheControlDirectiveWithQualifiers>,
    #[serde(rename = "no-store", skip_serializing_if = "::std::option::Option::is_none")]
    pub no_store: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(
        rename = "no-transform",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub no_transform: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub private: ::std::option::Option<RulesetsSetCacheControlDirectiveWithQualifiers>,
    #[serde(
        rename = "proxy-revalidate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub proxy_revalidate: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub public: ::std::option::Option<RulesetsSetCacheControlDirective>,
    #[serde(rename = "s-maxage", skip_serializing_if = "::std::option::Option::is_none")]
    pub s_maxage: ::std::option::Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(
        rename = "stale-if-error",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stale_if_error: ::std::option::Option<RulesetsSetCacheControlDirectiveWithValue>,
    #[serde(
        rename = "stale-while-revalidate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stale_while_revalidate: ::std::option::Option<
        RulesetsSetCacheControlDirectiveWithValue,
    >,
}
///A list of additional ports that caching should be enabled on.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheSettingsAdditionalCacheablePorts(
    pub ::std::vec::Vec<::std::num::NonZeroU64>,
);
impl ::std::ops::Deref for RulesetsSetCacheSettingsAdditionalCacheablePorts {
    type Target = ::std::vec::Vec<::std::num::NonZeroU64>;
    fn deref(&self) -> &::std::vec::Vec<::std::num::NonZeroU64> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheSettingsAdditionalCacheablePorts>
for ::std::vec::Vec<::std::num::NonZeroU64> {
    fn from(value: RulesetsSetCacheSettingsAdditionalCacheablePorts) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::num::NonZeroU64>>
for RulesetsSetCacheSettingsAdditionalCacheablePorts {
    fn from(value: ::std::vec::Vec<::std::num::NonZeroU64>) -> Self {
        Self(value)
    }
}
///How long client browsers should cache the response. Cloudflare cache purge will not purge content cached on client browsers, so high browser TTLs may lead to stale content.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsBrowserTtl {
    ///The browser TTL (in seconds) if you choose the "override_origin" mode.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<u64>,
    ///The browser TTL mode.
    pub mode: TtlMode,
}
///Whether the request's response from the origin is eligible for caching. Caching itself will still depend on the cache control header and your other caching configurations.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheSettingsCache(pub bool);
impl ::std::ops::Deref for RulesetsSetCacheSettingsCache {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheSettingsCache> for bool {
    fn from(value: RulesetsSetCacheSettingsCache) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for RulesetsSetCacheSettingsCache {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsSetCacheSettingsCache {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsSetCacheSettingsCache {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheSettingsCache {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RulesetsSetCacheSettingsCache {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///Which components of the request are included in or excluded from the cache key Cloudflare uses to store the response in cache.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsCacheKey {
    ///Whether to separate cached content based on the visitor's device type.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache_by_device_type: ::std::option::Option<bool>,
    ///Whether to protect from web cache deception attacks, while allowing static assets to be cached.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache_deception_armor: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_key: ::std::option::Option<RulesetsSetCacheSettingsCustomCacheKey>,
    ///Whether to treat requests with the same query parameters the same, regardless of the order those query parameters are in.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ignore_query_strings_order: ::std::option::Option<bool>,
}
///Settings to determine whether the request's response from origin is eligible for Cache Reserve (requires a Cache Reserve add-on plan).
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsCacheReserve {
    ///Whether Cache Reserve is enabled. If this is true and a request meets eligibility criteria, Cloudflare will write the resource to Cache Reserve.
    pub eligible: bool,
    ///The minimum file size eligible for storage in Cache Reserve.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum_file_size: ::std::option::Option<u64>,
}
///Which components of the request are included or excluded from the cache key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsCustomCacheKey {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cookie: ::std::option::Option<RulesetsSetCacheSettingsCustomCacheKeyCookie>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<RulesetsSetCacheSettingsCustomCacheKeyHeader>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub host: ::std::option::Option<RulesetsSetCacheSettingsCustomCacheKeyHost>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub query_string: ::std::option::Option<
        RulesetsSetCacheSettingsCustomCacheKeyQueryString,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<RulesetsSetCacheSettingsCustomCacheKeyUser>,
}
///Which cookies to include in the cache key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsCustomCacheKeyCookie {
    ///A list of cookies to check for the presence of. The presence of these cookies is included in the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub check_presence: ::std::option::Option<::std::vec::Vec<CookieName>>,
    ///A list of cookies to include in the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include: ::std::option::Option<::std::vec::Vec<CookieName>>,
}
///Which headers to include in the cache key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsCustomCacheKeyHeader {
    ///A list of headers to check for the presence of. The presence of these headers is included in the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub check_presence: ::std::option::Option<::std::vec::Vec<Header>>,
    ///A mapping of header names to a list of values. If a header is present in the request and contains any of the values provided, its value is included in the cache key.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub contains: ::std::collections::HashMap<
        ::std::string::String,
        ::std::vec::Vec<HeaderValue>,
    >,
    ///Whether to exclude the origin header in the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exclude_origin: ::std::option::Option<bool>,
    ///A list of headers to include in the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include: ::std::option::Option<::std::vec::Vec<Header>>,
}
///How to use the host in the cache key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsCustomCacheKeyHost {
    ///Whether to use the resolved host in the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resolved: ::std::option::Option<bool>,
}
///Which query string parameters to include in or exclude from the cache key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsCustomCacheKeyQueryString {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exclude: ::std::option::Option<Exclude>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include: ::std::option::Option<Include>,
}
///How to use characteristics of the request user agent in the cache key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsCustomCacheKeyUser {
    ///Whether to use the user agent's device type in the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub device_type: ::std::option::Option<bool>,
    ///Whether to use the user agents's country in the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub geo: ::std::option::Option<bool>,
    ///Whether to use the user agent's language in the cache key.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lang: ::std::option::Option<bool>,
}
///How long the Cloudflare edge network should cache the response.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsEdgeTtl {
    ///The edge TTL (in seconds) if you choose the "override_origin" mode.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<u64>,
    ///The edge TTL mode.
    pub mode: TtlMode,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status_code_ttl: ::std::option::Option<RulesetsSetCacheSettingsStatusCodeTtl>,
}
///Whether Cloudflare will aim to strictly adhere to RFC 7234.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheSettingsOriginCacheControl(pub bool);
impl ::std::ops::Deref for RulesetsSetCacheSettingsOriginCacheControl {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheSettingsOriginCacheControl> for bool {
    fn from(value: RulesetsSetCacheSettingsOriginCacheControl) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for RulesetsSetCacheSettingsOriginCacheControl {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsSetCacheSettingsOriginCacheControl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsSetCacheSettingsOriginCacheControl {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheSettingsOriginCacheControl {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RulesetsSetCacheSettingsOriginCacheControl {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///Whether to generate Cloudflare error pages for issues from the origin server.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheSettingsOriginErrorPagePassthru(pub bool);
impl ::std::ops::Deref for RulesetsSetCacheSettingsOriginErrorPagePassthru {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheSettingsOriginErrorPagePassthru> for bool {
    fn from(value: RulesetsSetCacheSettingsOriginErrorPagePassthru) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for RulesetsSetCacheSettingsOriginErrorPagePassthru {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsSetCacheSettingsOriginErrorPagePassthru {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsSetCacheSettingsOriginErrorPagePassthru {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheSettingsOriginErrorPagePassthru {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String>
for RulesetsSetCacheSettingsOriginErrorPagePassthru {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///Controls whether Cloudflare fetches a large asset from the origin as a series of range requests instead of one whole-body request.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsOriginRangeRequests {
    ///Whether to use range requests. `default` is the behaviour the zone gets without this rule.
    pub mode: Mode,
}
///A timeout value between two successive read operations to use for your origin server. Historically, the timeout value between two read options from Cloudflare to an origin server is 100 seconds. If you are attempting to reduce HTTP 524 errors because of timeouts from an origin server, try increasing this timeout value.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheSettingsReadTimeout(pub i64);
impl ::std::ops::Deref for RulesetsSetCacheSettingsReadTimeout {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheSettingsReadTimeout> for i64 {
    fn from(value: RulesetsSetCacheSettingsReadTimeout) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for RulesetsSetCacheSettingsReadTimeout {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsSetCacheSettingsReadTimeout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsSetCacheSettingsReadTimeout {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheSettingsReadTimeout {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RulesetsSetCacheSettingsReadTimeout {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///Whether Cloudflare should respect strong ETag (entity tag) headers. If false, Cloudflare converts strong ETag headers to weak ETag headers.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheSettingsRespectStrongEtags(pub bool);
impl ::std::ops::Deref for RulesetsSetCacheSettingsRespectStrongEtags {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheSettingsRespectStrongEtags> for bool {
    fn from(value: RulesetsSetCacheSettingsRespectStrongEtags) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for RulesetsSetCacheSettingsRespectStrongEtags {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsSetCacheSettingsRespectStrongEtags {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsSetCacheSettingsRespectStrongEtags {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheSettingsRespectStrongEtags {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RulesetsSetCacheSettingsRespectStrongEtags {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///`RulesetsSetCacheSettingsRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsSetCacheSettingsRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<
        RulesetsSetCacheSettingsRuleActionParameters,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsSetCacheSettingsRuleAction`
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
pub enum RulesetsSetCacheSettingsRuleAction {
    #[serde(rename = "set_cache_settings")]
    SetCacheSettings,
}
impl ::std::fmt::Display for RulesetsSetCacheSettingsRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetCacheSettings => f.write_str("set_cache_settings"),
        }
    }
}
impl ::std::str::FromStr for RulesetsSetCacheSettingsRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set_cache_settings" => Ok(Self::SetCacheSettings),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheSettingsRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsSetCacheSettingsRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsSetCacheSettingsRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsRuleActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub additional_cacheable_ports: ::std::option::Option<
        RulesetsSetCacheSettingsAdditionalCacheablePorts,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub browser_ttl: ::std::option::Option<RulesetsSetCacheSettingsBrowserTtl>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache: ::std::option::Option<RulesetsSetCacheSettingsCache>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache_key: ::std::option::Option<RulesetsSetCacheSettingsCacheKey>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache_reserve: ::std::option::Option<RulesetsSetCacheSettingsCacheReserve>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub edge_ttl: ::std::option::Option<RulesetsSetCacheSettingsEdgeTtl>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub origin_cache_control: ::std::option::Option<
        RulesetsSetCacheSettingsOriginCacheControl,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub origin_error_page_passthru: ::std::option::Option<
        RulesetsSetCacheSettingsOriginErrorPagePassthru,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub origin_range_requests: ::std::option::Option<
        RulesetsSetCacheSettingsOriginRangeRequests,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub read_timeout: ::std::option::Option<RulesetsSetCacheSettingsReadTimeout>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub respect_strong_etags: ::std::option::Option<
        RulesetsSetCacheSettingsRespectStrongEtags,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub serve_stale: ::std::option::Option<RulesetsSetCacheSettingsServeStale>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shared_dictionary: ::std::option::Option<
        RulesetsSetCacheSettingsSharedDictionary,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strip_etags: ::std::option::Option<RulesetsSetCacheSettingsStripETags>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strip_last_modified: ::std::option::Option<
        RulesetsSetCacheSettingsStripLastModified,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strip_set_cookie: ::std::option::Option<RulesetsSetCacheSettingsStripSetCookie>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vary: ::std::option::Option<RulesetsSetCacheSettingsVary>,
}
///When to serve stale content from cache.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsServeStale {
    ///Whether Cloudflare should disable serving stale content while getting the latest content from the origin.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disable_stale_while_updating: ::std::option::Option<bool>,
}
///Configuration for shared dictionary compression. When set, Cloudflare injects Use-As-Dictionary headers on matching cacheable responses.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsSharedDictionary {
    ///URL pattern for the Use-As-Dictionary match field. This pattern specifies which URLs can use this response as a dictionary.
    pub match_pattern: MatchPattern,
}
///A list of TTLs to apply to specific status codes or status code ranges.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheSettingsStatusCodeTtl(
    pub ::std::vec::Vec<RulesetsSetCacheSettingsStatusCodeTtlItem>,
);
impl ::std::ops::Deref for RulesetsSetCacheSettingsStatusCodeTtl {
    type Target = ::std::vec::Vec<RulesetsSetCacheSettingsStatusCodeTtlItem>;
    fn deref(&self) -> &::std::vec::Vec<RulesetsSetCacheSettingsStatusCodeTtlItem> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheSettingsStatusCodeTtl>
for ::std::vec::Vec<RulesetsSetCacheSettingsStatusCodeTtlItem> {
    fn from(value: RulesetsSetCacheSettingsStatusCodeTtl) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<RulesetsSetCacheSettingsStatusCodeTtlItem>>
for RulesetsSetCacheSettingsStatusCodeTtl {
    fn from(value: ::std::vec::Vec<RulesetsSetCacheSettingsStatusCodeTtlItem>) -> Self {
        Self(value)
    }
}
///`RulesetsSetCacheSettingsStatusCodeTtlItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsStatusCodeTtlItem {
    ///A single status code to apply the TTL to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status_code: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status_code_range: ::std::option::Option<StatusCodeRange>,
    ///The time to cache the response for (in seconds). A value of 0 is equivalent to setting the cache control header with the value "no-cache". A value of -1 is equivalent to setting the cache control header with the value of "no-store".
    pub value: i64,
}
///Whether to strip ETag headers from the origin response before caching.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheSettingsStripETags(pub bool);
impl ::std::ops::Deref for RulesetsSetCacheSettingsStripETags {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheSettingsStripETags> for bool {
    fn from(value: RulesetsSetCacheSettingsStripETags) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for RulesetsSetCacheSettingsStripETags {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsSetCacheSettingsStripETags {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsSetCacheSettingsStripETags {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheSettingsStripETags {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RulesetsSetCacheSettingsStripETags {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///Whether to strip Last-Modified headers from the origin response before caching.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheSettingsStripLastModified(pub bool);
impl ::std::ops::Deref for RulesetsSetCacheSettingsStripLastModified {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheSettingsStripLastModified> for bool {
    fn from(value: RulesetsSetCacheSettingsStripLastModified) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for RulesetsSetCacheSettingsStripLastModified {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsSetCacheSettingsStripLastModified {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsSetCacheSettingsStripLastModified {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheSettingsStripLastModified {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RulesetsSetCacheSettingsStripLastModified {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///Whether to strip Set-Cookie headers from the origin response before caching.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheSettingsStripSetCookie(pub bool);
impl ::std::ops::Deref for RulesetsSetCacheSettingsStripSetCookie {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheSettingsStripSetCookie> for bool {
    fn from(value: RulesetsSetCacheSettingsStripSetCookie) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for RulesetsSetCacheSettingsStripSetCookie {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RulesetsSetCacheSettingsStripSetCookie {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RulesetsSetCacheSettingsStripSetCookie {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheSettingsStripSetCookie {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RulesetsSetCacheSettingsStripSetCookie {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
///Controls how cached responses vary based on request headers. `default` is required by the API and applies to any Vary response header that does not have a per-header override.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsVary {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<RulesetsSetCacheSettingsVaryDefault>,
    ///A mapping of lowercase request header names to their vary configuration.
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub headers: ::std::collections::HashMap<
        ::std::string::String,
        RulesetsSetCacheSettingsVaryHeader,
    >,
}
///Controls how response Vary headers without a per-header override contribute to the cache key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsVaryDefault {
    ///How the header value is treated when building the cache key.
    pub action: Action,
}
///Controls how a single request header contributes to the cache key.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheSettingsVaryHeader {
    ///How the header value is treated when building the cache key.
    pub action: Action,
    ///The set of languages to normalize against. Only valid for the `accept-language` header.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<Language>,
    ///The set of media types to normalize against. Only valid for the `accept` header.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub media_types: ::std::vec::Vec<MediaType>,
}
///An expression that evaluates to an array of cache tag values.
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
pub struct RulesetsSetCacheTagsExpression(::std::string::String);
impl ::std::ops::Deref for RulesetsSetCacheTagsExpression {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheTagsExpression> for ::std::string::String {
    fn from(value: RulesetsSetCacheTagsExpression) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsSetCacheTagsExpression {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheTagsExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsSetCacheTagsExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsSetCacheTagsExpression {
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
///The operation to perform on the cache tags.
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
pub enum RulesetsSetCacheTagsOperation {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "set")]
    Set,
}
impl ::std::fmt::Display for RulesetsSetCacheTagsOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Add => f.write_str("add"),
            Self::Remove => f.write_str("remove"),
            Self::Set => f.write_str("set"),
        }
    }
}
impl ::std::str::FromStr for RulesetsSetCacheTagsOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "add" => Ok(Self::Add),
            "remove" => Ok(Self::Remove),
            "set" => Ok(Self::Set),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheTagsOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsSetCacheTagsOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsSetCacheTagsRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetCacheTagsRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsSetCacheTagsRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<
        RulesetsSetCacheTagsRuleActionParameters,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsSetCacheTagsRuleAction`
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
pub enum RulesetsSetCacheTagsRuleAction {
    #[serde(rename = "set_cache_tags")]
    SetCacheTags,
}
impl ::std::fmt::Display for RulesetsSetCacheTagsRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetCacheTags => f.write_str("set_cache_tags"),
        }
    }
}
impl ::std::str::FromStr for RulesetsSetCacheTagsRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set_cache_tags" => Ok(Self::SetCacheTags),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetCacheTagsRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsSetCacheTagsRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsSetCacheTagsRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum RulesetsSetCacheTagsRuleActionParameters {
    AddCacheTagsValues {
        operation: AddCacheTagsValuesOperation,
        values: RulesetsSetCacheTagsValues,
    },
    AddCacheTagsExpression {
        expression: RulesetsSetCacheTagsExpression,
        operation: AddCacheTagsExpressionOperation,
    },
    RemoveCacheTagsValues {
        operation: RemoveCacheTagsValuesOperation,
        values: RulesetsSetCacheTagsValues,
    },
    RemoveCacheTagsExpression {
        expression: RulesetsSetCacheTagsExpression,
        operation: RemoveCacheTagsExpressionOperation,
    },
    SetCacheTagsValues {
        operation: SetCacheTagsValuesOperation,
        values: RulesetsSetCacheTagsValues,
    },
    SetCacheTagsExpression {
        expression: RulesetsSetCacheTagsExpression,
        operation: SetCacheTagsExpressionOperation,
    },
}
///A list of cache tag values.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSetCacheTagsValues(pub ::std::vec::Vec<CacheTag>);
impl ::std::ops::Deref for RulesetsSetCacheTagsValues {
    type Target = ::std::vec::Vec<CacheTag>;
    fn deref(&self) -> &::std::vec::Vec<CacheTag> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSetCacheTagsValues> for ::std::vec::Vec<CacheTag> {
    fn from(value: RulesetsSetCacheTagsValues) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<CacheTag>> for RulesetsSetCacheTagsValues {
    fn from(value: ::std::vec::Vec<CacheTag>) -> Self {
        Self(value)
    }
}
///Which file extensions to minify automatically.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetConfigAutominify {
    ///Whether to minify CSS files.
    #[serde(default)]
    pub css: bool,
    ///Whether to minify HTML files.
    #[serde(default)]
    pub html: bool,
    ///Whether to minify JavaScript files.
    #[serde(default)]
    pub js: bool,
}
///`RulesetsSetConfigRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetConfigRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsSetConfigRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<RulesetsSetConfigRuleActionParameters>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsSetConfigRuleAction`
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
pub enum RulesetsSetConfigRuleAction {
    #[serde(rename = "set_config")]
    SetConfig,
}
impl ::std::fmt::Display for RulesetsSetConfigRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetConfig => f.write_str("set_config"),
        }
    }
}
impl ::std::str::FromStr for RulesetsSetConfigRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set_config" => Ok(Self::SetConfig),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSetConfigRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsSetConfigRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsSetConfigRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSetConfigRuleActionParameters {
    ///Whether to enable Automatic HTTPS Rewrites.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub automatic_https_rewrites: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub autominify: ::std::option::Option<RulesetsSetConfigAutominify>,
    ///Whether to enable Browser Integrity Check (BIC).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bic: ::std::option::Option<bool>,
    ///Whether to enable content conversion (e.g., HTML to Markdown).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub content_converter: ::std::option::Option<bool>,
    ///Whether to disable Cloudflare Apps.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disable_apps: ::std::option::Option<bool>,
    ///Whether to disable Pay Per Crawl.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disable_pay_per_crawl: ::std::option::Option<bool>,
    ///Whether to disable Real User Monitoring (RUM).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disable_rum: ::std::option::Option<bool>,
    ///Whether to disable Zaraz.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disable_zaraz: ::std::option::Option<bool>,
    ///Whether to enable Email Obfuscation.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub email_obfuscation: ::std::option::Option<bool>,
    ///Whether to enable Cloudflare Fonts.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fonts: ::std::option::Option<bool>,
    ///Whether to enable Hotlink Protection.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hotlink_protection: ::std::option::Option<bool>,
    ///Whether to enable Mirage.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mirage: ::std::option::Option<bool>,
    ///Whether to enable Opportunistic Encryption.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub opportunistic_encryption: ::std::option::Option<bool>,
    ///The Polish level to configure.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub polish: ::std::option::Option<Polish>,
    ///Whether to redirect verified AI training crawlers to canonical URLs found in the HTML response.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub redirects_for_ai_training: ::std::option::Option<bool>,
    ///The request body buffering mode.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub request_body_buffering: ::std::option::Option<RequestBodyBuffering>,
    ///The response body buffering mode.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response_body_buffering: ::std::option::Option<ResponseBodyBuffering>,
    ///Whether to enable Rocket Loader.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rocket_loader: ::std::option::Option<bool>,
    ///The Security Level to configure.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub security_level: ::std::option::Option<SecurityLevel>,
    ///Whether to enable Server-Side Excludes.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub server_side_excludes: ::std::option::Option<bool>,
    ///The SSL level to configure.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ssl: ::std::option::Option<Ssl>,
    ///Whether to enable Signed Exchanges (SXG).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sxg: ::std::option::Option<bool>,
}
///A phase to skip the execution of. This option is only compatible with the products option.
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
pub enum RulesetsSkipPhase {
    #[serde(rename = "current")]
    Current,
}
impl ::std::fmt::Display for RulesetsSkipPhase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Current => f.write_str("current"),
        }
    }
}
impl ::std::str::FromStr for RulesetsSkipPhase {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "current" => Ok(Self::Current),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSkipPhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsSkipPhase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A list of phases to skip the execution of. This option is incompatible with the rulesets option.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSkipPhases(pub ::std::vec::Vec<RulesetsRulesetPhase>);
impl ::std::ops::Deref for RulesetsSkipPhases {
    type Target = ::std::vec::Vec<RulesetsRulesetPhase>;
    fn deref(&self) -> &::std::vec::Vec<RulesetsRulesetPhase> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSkipPhases> for ::std::vec::Vec<RulesetsRulesetPhase> {
    fn from(value: RulesetsSkipPhases) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<RulesetsRulesetPhase>> for RulesetsSkipPhases {
    fn from(value: ::std::vec::Vec<RulesetsRulesetPhase>) -> Self {
        Self(value)
    }
}
///A list of legacy security products to skip the execution of.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSkipProducts(pub ::std::vec::Vec<Product>);
impl ::std::ops::Deref for RulesetsSkipProducts {
    type Target = ::std::vec::Vec<Product>;
    fn deref(&self) -> &::std::vec::Vec<Product> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSkipProducts> for ::std::vec::Vec<Product> {
    fn from(value: RulesetsSkipProducts) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Product>> for RulesetsSkipProducts {
    fn from(value: ::std::vec::Vec<Product>) -> Self {
        Self(value)
    }
}
///`RulesetsSkipRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSkipRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsSkipRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<RulesetsSkipRuleActionParameters>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsSkipRuleAction`
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
pub enum RulesetsSkipRuleAction {
    #[serde(rename = "skip")]
    Skip,
}
impl ::std::fmt::Display for RulesetsSkipRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Skip => f.write_str("skip"),
        }
    }
}
impl ::std::str::FromStr for RulesetsSkipRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "skip" => Ok(Self::Skip),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSkipRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsSkipRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsSkipRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsSkipRuleActionParameters {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub phase: ::std::option::Option<RulesetsSkipPhase>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub phases: ::std::option::Option<RulesetsSkipPhases>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub products: ::std::option::Option<RulesetsSkipProducts>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rules: ::std::option::Option<RulesetsSkipRules>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ruleset: ::std::option::Option<RulesetsSkipRuleset>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rulesets: ::std::option::Option<RulesetsSkipRulesets>,
}
///A mapping of ruleset IDs to a list of rule IDs in that ruleset to skip the execution of. This option is incompatible with the ruleset option.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSkipRules(
    pub ::std::collections::HashMap<
        ::std::string::String,
        ::std::vec::Vec<RulesetsRuleId>,
    >,
);
impl ::std::ops::Deref for RulesetsSkipRules {
    type Target = ::std::collections::HashMap<
        ::std::string::String,
        ::std::vec::Vec<RulesetsRuleId>,
    >;
    fn deref(
        &self,
    ) -> &::std::collections::HashMap<
        ::std::string::String,
        ::std::vec::Vec<RulesetsRuleId>,
    > {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSkipRules>
for ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<RulesetsRuleId>> {
    fn from(value: RulesetsSkipRules) -> Self {
        value.0
    }
}
impl ::std::convert::From<
    ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<RulesetsRuleId>>,
> for RulesetsSkipRules {
    fn from(
        value: ::std::collections::HashMap<
            ::std::string::String,
            ::std::vec::Vec<RulesetsRuleId>,
        >,
    ) -> Self {
        Self(value)
    }
}
///A ruleset to skip the execution of. This option is incompatible with the rulesets option.
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
pub enum RulesetsSkipRuleset {
    #[serde(rename = "current")]
    Current,
}
impl ::std::fmt::Display for RulesetsSkipRuleset {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Current => f.write_str("current"),
        }
    }
}
impl ::std::str::FromStr for RulesetsSkipRuleset {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "current" => Ok(Self::Current),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsSkipRuleset {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsSkipRuleset {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A list of ruleset IDs to skip the execution of. This option is incompatible with the ruleset and phases options.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct RulesetsSkipRulesets(pub ::std::vec::Vec<RulesetsRulesetId>);
impl ::std::ops::Deref for RulesetsSkipRulesets {
    type Target = ::std::vec::Vec<RulesetsRulesetId>;
    fn deref(&self) -> &::std::vec::Vec<RulesetsRulesetId> {
        &self.0
    }
}
impl ::std::convert::From<RulesetsSkipRulesets> for ::std::vec::Vec<RulesetsRulesetId> {
    fn from(value: RulesetsSkipRulesets) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<RulesetsRulesetId>> for RulesetsSkipRulesets {
    fn from(value: ::std::vec::Vec<RulesetsRulesetId>) -> Self {
        Self(value)
    }
}
///`RulesetsTransformResponseHtmlRule`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsTransformResponseHtmlRule {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<RulesetsTransformResponseHtmlRuleAction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action_parameters: ::std::option::Option<
        RulesetsTransformResponseHtmlRuleActionParameters,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<RulesetsRuleCategories>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<RulesetsRuleEnabled>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exposed_credential_check: ::std::option::Option<
        RulesetsRuleExposedCredentialCheck,
    >,
    ///The expression defining which traffic will match the rule.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<Expression>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<RulesetsRuleId>,
    ///The timestamp of when the rule was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: ::std::option::Option<RulesetsRuleLogging>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ratelimit: ::std::option::Option<RulesetsRuleRatelimit>,
    ///The reference of the rule (the rule's ID by default).
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<Ref>,
    ///The version of the rule.
    pub version: Version,
}
///`RulesetsTransformResponseHtmlRuleAction`
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
pub enum RulesetsTransformResponseHtmlRuleAction {
    #[serde(rename = "transform_response_html")]
    TransformResponseHtml,
}
impl ::std::fmt::Display for RulesetsTransformResponseHtmlRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TransformResponseHtml => f.write_str("transform_response_html"),
        }
    }
}
impl ::std::str::FromStr for RulesetsTransformResponseHtmlRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transform_response_html" => Ok(Self::TransformResponseHtml),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsTransformResponseHtmlRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for RulesetsTransformResponseHtmlRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RulesetsTransformResponseHtmlRuleActionParameters`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct RulesetsTransformResponseHtmlRuleActionParameters {
    ///Enables the link maze transformation on the response.
    pub link_maze: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///The unique ID of the zone.
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
pub struct RulesetsZoneId(::std::string::String);
impl ::std::ops::Deref for RulesetsZoneId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetsZoneId> for ::std::string::String {
    fn from(value: RulesetsZoneId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetsZoneId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[0-9a-f]{32}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9a-f]{32}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetsZoneId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetsZoneId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetsZoneId {
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
///A response header name provided by the origin, which contains the score to increment rate limit counter with.
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
pub struct ScoreResponseHeaderName(::std::string::String);
impl ::std::ops::Deref for ScoreResponseHeaderName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ScoreResponseHeaderName> for ::std::string::String {
    fn from(value: ScoreResponseHeaderName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ScoreResponseHeaderName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ScoreResponseHeaderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScoreResponseHeaderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ScoreResponseHeaderName {
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
///The Security Level to configure.
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
pub enum SecurityLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "essentially_off")]
    EssentiallyOff,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "under_attack")]
    UnderAttack,
}
impl ::std::fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Off => f.write_str("off"),
            Self::EssentiallyOff => f.write_str("essentially_off"),
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::UnderAttack => f.write_str("under_attack"),
        }
    }
}
impl ::std::str::FromStr for SecurityLevel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "off" => Ok(Self::Off),
            "essentially_off" => Ok(Self::EssentiallyOff),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "under_attack" => Ok(Self::UnderAttack),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SecurityLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SecurityLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SetCacheTagsExpressionOperation`
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
pub enum SetCacheTagsExpressionOperation {
    #[serde(rename = "set")]
    Set,
}
impl ::std::fmt::Display for SetCacheTagsExpressionOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Set => f.write_str("set"),
        }
    }
}
impl ::std::str::FromStr for SetCacheTagsExpressionOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set" => Ok(Self::Set),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SetCacheTagsExpressionOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetCacheTagsExpressionOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SetCacheTagsValuesOperation`
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
pub enum SetCacheTagsValuesOperation {
    #[serde(rename = "set")]
    Set,
}
impl ::std::fmt::Display for SetCacheTagsValuesOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Set => f.write_str("set"),
        }
    }
}
impl ::std::str::FromStr for SetCacheTagsValuesOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set" => Ok(Self::Set),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SetCacheTagsValuesOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetCacheTagsValuesOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SetDirectiveOperation`
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
pub enum SetDirectiveOperation {
    #[serde(rename = "set")]
    Set,
}
impl ::std::fmt::Display for SetDirectiveOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Set => f.write_str("set"),
        }
    }
}
impl ::std::str::FromStr for SetDirectiveOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set" => Ok(Self::Set),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SetDirectiveOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetDirectiveOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SetDynamicHeaderOperation`
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
pub enum SetDynamicHeaderOperation {
    #[serde(rename = "set")]
    Set,
}
impl ::std::fmt::Display for SetDynamicHeaderOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Set => f.write_str("set"),
        }
    }
}
impl ::std::str::FromStr for SetDynamicHeaderOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set" => Ok(Self::Set),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SetDynamicHeaderOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetDynamicHeaderOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SetStaticHeaderOperation`
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
pub enum SetStaticHeaderOperation {
    #[serde(rename = "set")]
    Set,
}
impl ::std::fmt::Display for SetStaticHeaderOperation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Set => f.write_str("set"),
        }
    }
}
impl ::std::str::FromStr for SetStaticHeaderOperation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "set" => Ok(Self::Set),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SetStaticHeaderOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetStaticHeaderOperation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The source of this message.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct Source {
    ///A JSON pointer to the field that is the source of the message.
    pub pointer: Pointer,
}
///The SSL level to configure.
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
pub enum Ssl {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "flexible")]
    Flexible,
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "origin_pull")]
    OriginPull,
}
impl ::std::fmt::Display for Ssl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Off => f.write_str("off"),
            Self::Flexible => f.write_str("flexible"),
            Self::Full => f.write_str("full"),
            Self::Strict => f.write_str("strict"),
            Self::OriginPull => f.write_str("origin_pull"),
        }
    }
}
impl ::std::str::FromStr for Ssl {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "off" => Ok(Self::Off),
            "flexible" => Ok(Self::Flexible),
            "full" => Ok(Self::Full),
            "strict" => Ok(Self::Strict),
            "origin_pull" => Ok(Self::OriginPull),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Ssl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Ssl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A range of status codes to apply the TTL to.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct StatusCodeRange {
    ///The lower bound of the range.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub from: ::std::option::Option<i64>,
    ///The upper bound of the range.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub to: ::std::option::Option<i64>,
}
///A URL to redirect the request to.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TargetUrl {
    ///An expression that evaluates to a URL to redirect the request to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub expression: ::std::option::Option<RedirectExpression>,
    ///A URL to redirect the request to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<RedirectValue>,
}
///The transformed request field to log.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TransformedRequestField {
    ///The name of the header.
    pub name: HeaderName,
}
///The transformed response field to log.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct TransformedResponseField {
    ///The name of the response header.
    pub name: HeaderName,
    ///Whether to log duplicate values of the same header.
    #[serde(default)]
    pub preserve_duplicates: bool,
}
///The browser TTL mode.
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
pub enum TtlMode {
    #[serde(rename = "respect_origin")]
    RespectOrigin,
    #[serde(rename = "bypass_by_default")]
    BypassByDefault,
    #[serde(rename = "override_origin")]
    OverrideOrigin,
    #[serde(rename = "bypass")]
    Bypass,
}
impl ::std::fmt::Display for TtlMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RespectOrigin => f.write_str("respect_origin"),
            Self::BypassByDefault => f.write_str("bypass_by_default"),
            Self::OverrideOrigin => f.write_str("override_origin"),
            Self::Bypass => f.write_str("bypass"),
        }
    }
}
impl ::std::str::FromStr for TtlMode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "respect_origin" => Ok(Self::RespectOrigin),
            "bypass_by_default" => Ok(Self::BypassByDefault),
            "override_origin" => Ok(Self::OverrideOrigin),
            "bypass" => Ok(Self::Bypass),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TtlMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TtlMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///An expression that selects the user ID used in the credentials check.
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
pub struct UsernameExpression(::std::string::String);
impl ::std::ops::Deref for UsernameExpression {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<UsernameExpression> for ::std::string::String {
    fn from(value: UsernameExpression) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for UsernameExpression {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for UsernameExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UsernameExpression {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UsernameExpression {
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
///A value to override the SNI to.
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
pub struct Value(::std::string::String);
impl ::std::ops::Deref for Value {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Value> for ::std::string::String {
    fn from(value: Value) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Value {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Value {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Value {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Value {
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
///The version of the rule.
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
pub struct Version(::std::string::String);
impl ::std::ops::Deref for Version {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Version> for ::std::string::String {
    fn from(value: Version) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Version {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[0-9]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Version {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Version {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Version {
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
///`ZoneAtProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneAtProvider {
    /**The last time proof of ownership was detected and the zone was made
active.*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub activated_on: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///When the zone was created.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub created_on: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /**The interval (in seconds) from when development mode expires
(positive integer) or last expired (negative integer) for the
domain. If development mode has never been enabled, this value is 0.*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub development_mode: ::std::option::Option<f64>,
    ///Identifier
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<ZoneAtProviderId>,
    ///When the zone was last modified.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub modified_on: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///The domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment ("label") not exceeding 63 characters.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<ZoneAtProviderName>,
    ///The name servers Cloudflare assigns to a zone.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub name_servers: ::std::vec::Vec<::std::string::String>,
    ///DNS host at the time of switching to Cloudflare.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub original_dnshost: ::std::option::Option<ZoneAtProviderOriginalDnshost>,
    ///Original name servers before moving to Cloudflare.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub original_name_servers: ::std::option::Option<
        ::std::vec::Vec<::std::string::String>,
    >,
    ///Registrar for the domain at the time of switching to Cloudflare.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub original_registrar: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub paused: ::std::option::Option<ZonesPaused>,
    ///The zone status on Cloudflare.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<ZoneAtProviderStatus>,
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<ZonesType>,
    ///An array of domains used for custom name servers. This is only available for Business and Enterprise plans.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vanity_name_servers: ::std::vec::Vec<ZoneAtProviderVanityNameServersItem>,
    ///Verification key for partial zone setup.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub verification_key: ::std::option::Option<::std::string::String>,
}
///Identifier
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
pub struct ZoneAtProviderId(::std::string::String);
impl ::std::ops::Deref for ZoneAtProviderId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZoneAtProviderId> for ::std::string::String {
    fn from(value: ZoneAtProviderId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZoneAtProviderId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZoneAtProviderId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZoneAtProviderId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZoneAtProviderId {
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
///The domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment ("label") not exceeding 63 characters.
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
pub struct ZoneAtProviderName(::std::string::String);
impl ::std::ops::Deref for ZoneAtProviderName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZoneAtProviderName> for ::std::string::String {
    fn from(value: ZoneAtProviderName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZoneAtProviderName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 253usize {
            return Err("longer than 253 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^([a-zA-Z0-9][\\-a-zA-Z0-9]*\\.)+[\\-a-zA-Z0-9]{2,20}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^([a-zA-Z0-9][\\-a-zA-Z0-9]*\\.)+[\\-a-zA-Z0-9]{2,20}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZoneAtProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZoneAtProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZoneAtProviderName {
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
///DNS host at the time of switching to Cloudflare.
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
pub struct ZoneAtProviderOriginalDnshost(::std::string::String);
impl ::std::ops::Deref for ZoneAtProviderOriginalDnshost {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZoneAtProviderOriginalDnshost> for ::std::string::String {
    fn from(value: ZoneAtProviderOriginalDnshost) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZoneAtProviderOriginalDnshost {
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
impl ::std::convert::TryFrom<&str> for ZoneAtProviderOriginalDnshost {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZoneAtProviderOriginalDnshost {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZoneAtProviderOriginalDnshost {
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
///The zone status on Cloudflare.
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
pub enum ZoneAtProviderStatus {
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "moved")]
    Moved,
}
impl ::std::fmt::Display for ZoneAtProviderStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Initializing => f.write_str("initializing"),
            Self::Pending => f.write_str("pending"),
            Self::Active => f.write_str("active"),
            Self::Moved => f.write_str("moved"),
        }
    }
}
impl ::std::str::FromStr for ZoneAtProviderStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "initializing" => Ok(Self::Initializing),
            "pending" => Ok(Self::Pending),
            "active" => Ok(Self::Active),
            "moved" => Ok(Self::Moved),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ZoneAtProviderStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZoneAtProviderStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ZoneAtProviderVanityNameServersItem`
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
pub struct ZoneAtProviderVanityNameServersItem(::std::string::String);
impl ::std::ops::Deref for ZoneAtProviderVanityNameServersItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZoneAtProviderVanityNameServersItem>
for ::std::string::String {
    fn from(value: ZoneAtProviderVanityNameServersItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZoneAtProviderVanityNameServersItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 253usize {
            return Err("longer than 253 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZoneAtProviderVanityNameServersItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ZoneAtProviderVanityNameServersItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZoneAtProviderVanityNameServersItem {
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
///`ZoneCreateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneCreateRequest {
    pub account: ZoneCreateRequestAccount,
    pub name: ZonesName,
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<ZonesType>,
}
///`ZoneCreateRequestAccount`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneCreateRequestAccount {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<ZonesIdentifier>,
}
///`ZoneCreateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneCreateResponse {
    pub errors: ZonesMessages,
    pub messages: ZonesMessages,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<ZonesZone>,
    ///Whether the API call was successful.
    pub success: bool,
}
///`ZoneForProvider`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneForProvider {
    ///The domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment ("label") not exceeding 63 characters.
    pub name: ZoneForProviderName,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub paused: ::std::option::Option<ZonesPaused>,
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<ZonesType>,
    ///An array of domains used for custom name servers. This is only available for Business and Enterprise plans.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vanity_name_servers: ::std::vec::Vec<ZoneForProviderVanityNameServersItem>,
}
///The domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment ("label") not exceeding 63 characters.
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
pub struct ZoneForProviderName(::std::string::String);
impl ::std::ops::Deref for ZoneForProviderName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZoneForProviderName> for ::std::string::String {
    fn from(value: ZoneForProviderName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZoneForProviderName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 253usize {
            return Err("longer than 253 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^([a-zA-Z0-9][\\-a-zA-Z0-9]*\\.)+[\\-a-zA-Z0-9]{2,20}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^([a-zA-Z0-9][\\-a-zA-Z0-9]*\\.)+[\\-a-zA-Z0-9]{2,20}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZoneForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZoneForProviderName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZoneForProviderName {
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
///`ZoneForProviderVanityNameServersItem`
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
pub struct ZoneForProviderVanityNameServersItem(::std::string::String);
impl ::std::ops::Deref for ZoneForProviderVanityNameServersItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZoneForProviderVanityNameServersItem>
for ::std::string::String {
    fn from(value: ZoneForProviderVanityNameServersItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZoneForProviderVanityNameServersItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 253usize {
            return Err("longer than 253 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZoneForProviderVanityNameServersItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ZoneForProviderVanityNameServersItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZoneForProviderVanityNameServersItem {
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
///`ZoneObserveResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneObserveResponse {
    pub errors: ZonesMessages,
    pub messages: ZonesMessages,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<ZonesZone>,
    ///Whether the API call was successful.
    pub success: bool,
}
///`ZoneRulesetCreateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneRulesetCreateRequest {
    ///An informative description of the ruleset.
    #[serde(default)]
    pub description: ::std::string::String,
    pub id: RulesetsRulesetId,
    pub kind: RulesetsRulesetKind,
    ///The timestamp of when the ruleset was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    ///The human-readable name of the ruleset.
    pub name: Name,
    pub phase: RulesetsRulesetPhase,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rules: ::std::option::Option<RulesetsRequestRules>,
    pub version: RulesetsRulesetVersion,
}
///`ZoneRulesetCreateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneRulesetCreateResponse {
    pub errors: RulesetsErrors,
    pub messages: RulesetsMessages,
    pub result: ZoneRulesetCreateResponseResult,
    pub success: bool,
}
///`ZoneRulesetCreateResponseResult`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum ZoneRulesetCreateResponseResult {
    Variant0 {
        ///An informative description of the ruleset.
        #[serde(default)]
        description: ::std::string::String,
        id: RulesetsRulesetId,
        kind: RulesetsRulesetKind,
        ///The timestamp of when the ruleset was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        ///The human-readable name of the ruleset.
        name: Name,
        phase: RulesetsRulesetPhase,
        rules: RulesetsResponseRules,
        version: RulesetsRulesetVersion,
    },
    Variant1,
}
///`ZoneRulesetObserveResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneRulesetObserveResponse {
    pub errors: RulesetsErrors,
    pub messages: RulesetsMessages,
    pub result: ZoneRulesetObserveResponseResult,
    pub success: bool,
}
///`ZoneRulesetObserveResponseResult`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneRulesetObserveResponseResult {
    ///An informative description of the ruleset.
    #[serde(default)]
    pub description: ::std::string::String,
    pub id: RulesetsRulesetId,
    pub kind: RulesetsRulesetKind,
    ///The timestamp of when the ruleset was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    ///The human-readable name of the ruleset.
    pub name: Name,
    pub phase: RulesetsRulesetPhase,
    pub rules: RulesetsResponseRules,
    pub version: RulesetsRulesetVersion,
}
///`ZoneRulesetUpdateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneRulesetUpdateRequest {
    ///An informative description of the ruleset.
    #[serde(default)]
    pub description: ::std::string::String,
    pub id: RulesetsRulesetId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub kind: ::std::option::Option<RulesetsRulesetKind>,
    ///The timestamp of when the ruleset was last modified.
    pub last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
    ///The human-readable name of the ruleset.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub phase: ::std::option::Option<RulesetsRulesetPhase>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rules: ::std::option::Option<RulesetsRequestRules>,
    pub version: RulesetsRulesetVersion,
}
///`ZoneRulesetUpdateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneRulesetUpdateResponse {
    pub errors: RulesetsErrors,
    pub messages: RulesetsMessages,
    pub result: ZoneRulesetUpdateResponseResult,
    pub success: bool,
}
///`ZoneRulesetUpdateResponseResult`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(untagged)]
pub enum ZoneRulesetUpdateResponseResult {
    Variant0 {
        ///An informative description of the ruleset.
        #[serde(default)]
        description: ::std::string::String,
        id: RulesetsRulesetId,
        kind: RulesetsRulesetKind,
        ///The timestamp of when the ruleset was last modified.
        last_updated: ::chrono::DateTime<::chrono::offset::Utc>,
        ///The human-readable name of the ruleset.
        name: Name,
        phase: RulesetsRulesetPhase,
        rules: RulesetsResponseRules,
        version: RulesetsRulesetVersion,
    },
    Variant1,
}
///`ZoneUpdateRequest`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneUpdateRequest {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub paused: ::std::option::Option<ZonesPaused>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub plan: ::std::option::Option<ZoneUpdateRequestPlan>,
    /**A full zone implies that DNS is hosted with Cloudflare. A partial
zone is typically a partner-hosted zone or a CNAME setup. This
parameter is only available to Enterprise customers or if it has
been explicitly enabled on a zone.
*/
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<ZoneUpdateRequestType>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vanity_name_servers: ::std::option::Option<ZonesVanityNameServers>,
}
/**(Deprecated) Please use the `/zones/{zone_id}/subscription` API
to update a zone's plan. Changing this value will create/cancel
associated subscriptions. To view available plans for this zone,
see Zone Plans.
*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneUpdateRequestPlan {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<ZonesIdentifier>,
}
/**A full zone implies that DNS is hosted with Cloudflare. A partial
zone is typically a partner-hosted zone or a CNAME setup. This
parameter is only available to Enterprise customers or if it has
been explicitly enabled on a zone.
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
pub enum ZoneUpdateRequestType {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "internal")]
    Internal,
}
impl ::std::fmt::Display for ZoneUpdateRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Full => f.write_str("full"),
            Self::Partial => f.write_str("partial"),
            Self::Secondary => f.write_str("secondary"),
            Self::Internal => f.write_str("internal"),
        }
    }
}
impl ::std::str::FromStr for ZoneUpdateRequestType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "full" => Ok(Self::Full),
            "partial" => Ok(Self::Partial),
            "secondary" => Ok(Self::Secondary),
            "internal" => Ok(Self::Internal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ZoneUpdateRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZoneUpdateRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ZoneUpdateResponse`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZoneUpdateResponse {
    pub errors: ZonesMessages,
    pub messages: ZonesMessages,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<ZonesZone>,
    ///Whether the API call was successful.
    pub success: bool,
}
///`ZonesApiResponseCommon`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesApiResponseCommon {
    pub errors: ZonesMessages,
    pub messages: ZonesMessages,
    ///Whether the API call was successful.
    pub success: bool,
}
///`ZonesApiResponseCommonFailure`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesApiResponseCommonFailure {
    pub errors: ::std::vec::Vec<ZonesApiResponseCommonFailureErrorsItem>,
    pub messages: ZonesMessages,
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub result: ::std::option::Option<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    ///Whether the API call was successful.
    pub success: bool,
}
///`ZonesApiResponseCommonFailureErrorsItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesApiResponseCommonFailureErrorsItem {
    pub code: i64,
    pub message: ::std::string::String,
}
///`ZonesApiResponseSingleId`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesApiResponseSingleId {
    pub errors: ZonesMessages,
    pub messages: ZonesMessages,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<ZonesApiResponseSingleIdResult>,
    ///Whether the API call was successful.
    pub success: bool,
}
///`ZonesApiResponseSingleIdResult`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesApiResponseSingleIdResult {
    pub id: ZonesIdentifier,
}
///Identifier
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
pub struct ZonesIdentifier(::std::string::String);
impl ::std::ops::Deref for ZonesIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZonesIdentifier> for ::std::string::String {
    fn from(value: ZonesIdentifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZonesIdentifier {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZonesIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZonesIdentifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZonesIdentifier {
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
///`ZonesMessages`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ZonesMessages(pub ::std::vec::Vec<ZonesMessagesItem>);
impl ::std::ops::Deref for ZonesMessages {
    type Target = ::std::vec::Vec<ZonesMessagesItem>;
    fn deref(&self) -> &::std::vec::Vec<ZonesMessagesItem> {
        &self.0
    }
}
impl ::std::convert::From<ZonesMessages> for ::std::vec::Vec<ZonesMessagesItem> {
    fn from(value: ZonesMessages) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<ZonesMessagesItem>> for ZonesMessages {
    fn from(value: ::std::vec::Vec<ZonesMessagesItem>) -> Self {
        Self(value)
    }
}
///`ZonesMessagesItem`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesMessagesItem {
    pub code: i64,
    pub message: ::std::string::String,
}
///The domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment ("label") not exceeding 63 characters.
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
pub struct ZonesName(::std::string::String);
impl ::std::ops::Deref for ZonesName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZonesName> for ::std::string::String {
    fn from(value: ZonesName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZonesName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 253usize {
            return Err("longer than 253 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^([a-zA-Z0-9][\\-a-zA-Z0-9]*\\.)+[\\-a-zA-Z0-9]{2,20}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^([a-zA-Z0-9][\\-a-zA-Z0-9]*\\.)+[\\-a-zA-Z0-9]{2,20}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZonesName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZonesName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZonesName {
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
/**Indicates whether the zone is only using Cloudflare DNS services. A
true value means the zone will not receive security or performance
benefits.
*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ZonesPaused(pub bool);
impl ::std::ops::Deref for ZonesPaused {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<ZonesPaused> for bool {
    fn from(value: ZonesPaused) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for ZonesPaused {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for ZonesPaused {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for ZonesPaused {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for ZonesPaused {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ZonesPaused {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
/**A full zone implies that DNS is hosted with Cloudflare. A partial zone is
typically a partner-hosted zone or a CNAME setup.
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
pub enum ZonesType {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "internal")]
    Internal,
}
impl ::std::fmt::Display for ZonesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Full => f.write_str("full"),
            Self::Partial => f.write_str("partial"),
            Self::Secondary => f.write_str("secondary"),
            Self::Internal => f.write_str("internal"),
        }
    }
}
impl ::std::str::FromStr for ZonesType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "full" => Ok(Self::Full),
            "partial" => Ok(Self::Partial),
            "secondary" => Ok(Self::Secondary),
            "internal" => Ok(Self::Internal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ZonesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZonesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for ZonesType {
    fn default() -> Self {
        ZonesType::Full
    }
}
/**An array of domains used for custom name servers. This is only
available for Business and Enterprise plans.*/
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
#[serde(transparent)]
pub struct ZonesVanityNameServers(pub ::std::vec::Vec<ZonesVanityNameServersItem>);
impl ::std::ops::Deref for ZonesVanityNameServers {
    type Target = ::std::vec::Vec<ZonesVanityNameServersItem>;
    fn deref(&self) -> &::std::vec::Vec<ZonesVanityNameServersItem> {
        &self.0
    }
}
impl ::std::convert::From<ZonesVanityNameServers>
for ::std::vec::Vec<ZonesVanityNameServersItem> {
    fn from(value: ZonesVanityNameServers) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<ZonesVanityNameServersItem>>
for ZonesVanityNameServers {
    fn from(value: ::std::vec::Vec<ZonesVanityNameServersItem>) -> Self {
        Self(value)
    }
}
///`ZonesVanityNameServersItem`
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
pub struct ZonesVanityNameServersItem(::std::string::String);
impl ::std::ops::Deref for ZonesVanityNameServersItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZonesVanityNameServersItem> for ::std::string::String {
    fn from(value: ZonesVanityNameServersItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZonesVanityNameServersItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 253usize {
            return Err("longer than 253 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZonesVanityNameServersItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZonesVanityNameServersItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZonesVanityNameServersItem {
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
///`ZonesZone`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesZone {
    pub account: ZonesZoneAccount,
    /**The last time proof of ownership was detected and the zone was made
active.*/
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub activated_on: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /**Allows the customer to use a custom apex.
*Tenants Only Configuration*.*/
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cname_suffix: ::std::option::Option<::std::string::String>,
    ///When the zone was created.
    pub created_on: ::chrono::DateTime<::chrono::offset::Utc>,
    /**The interval (in seconds) from when development mode expires
(positive integer) or last expired (negative integer) for the
domain. If development mode has never been enabled, this value is 0.*/
    pub development_mode: f64,
    ///Identifier
    pub id: ZonesZoneId,
    pub meta: ZonesZoneMeta,
    ///When the zone was last modified.
    pub modified_on: ::chrono::DateTime<::chrono::offset::Utc>,
    ///The domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment ("label") not exceeding 63 characters.
    pub name: ZonesZoneName,
    ///The name servers Cloudflare assigns to a zone.
    pub name_servers: ::std::vec::Vec<::std::string::String>,
    ///DNS host at the time of switching to Cloudflare.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub original_dnshost: ::std::option::Option<ZonesZoneOriginalDnshost>,
    ///Original name servers before moving to Cloudflare.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub original_name_servers: ::std::option::Option<
        ::std::vec::Vec<::std::string::String>,
    >,
    ///Registrar for the domain at the time of switching to Cloudflare.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub original_registrar: ::std::option::Option<::std::string::String>,
    pub owner: ZonesZoneOwner,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub paused: ::std::option::Option<ZonesPaused>,
    ///Legacy permissions based on legacy user membership information.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub permissions: ::std::vec::Vec<::std::string::String>,
    pub plan: ZonesZonePlan,
    ///The zone status on Cloudflare.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<ZonesZoneStatus>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<ZonesZoneTenant>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant_unit: ::std::option::Option<ZonesZoneTenantUnit>,
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<ZonesType>,
    ///An array of domains used for custom name servers. This is only available for Business and Enterprise plans.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vanity_name_servers: ::std::vec::Vec<ZonesZoneVanityNameServersItem>,
    ///Verification key for partial zone setup.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub verification_key: ::std::option::Option<::std::string::String>,
}
///The account the zone belongs to.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesZoneAccount {
    ///Identifier
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<ZonesZoneAccountId>,
    ///The name of the account.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
///Identifier
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
pub struct ZonesZoneAccountId(::std::string::String);
impl ::std::ops::Deref for ZonesZoneAccountId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZonesZoneAccountId> for ::std::string::String {
    fn from(value: ZonesZoneAccountId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZonesZoneAccountId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZonesZoneAccountId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZonesZoneAccountId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZonesZoneAccountId {
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
///Identifier
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
pub struct ZonesZoneId(::std::string::String);
impl ::std::ops::Deref for ZonesZoneId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZonesZoneId> for ::std::string::String {
    fn from(value: ZonesZoneId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZonesZoneId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZonesZoneId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZonesZoneId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZonesZoneId {
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
///Metadata about the zone.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesZoneMeta {
    ///The zone is only configured for CDN.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cdn_only: ::std::option::Option<bool>,
    ///Number of Custom Certificates the zone can have.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_certificate_quota: ::std::option::Option<i64>,
    ///The zone is only configured for DNS.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dns_only: ::std::option::Option<bool>,
    ///The zone is setup with Foundation DNS.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub foundation_dns: ::std::option::Option<bool>,
    ///Number of Page Rules a zone can have.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub page_rule_quota: ::std::option::Option<i64>,
    ///The zone has been flagged for phishing.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub phishing_detected: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub step: ::std::option::Option<i64>,
}
///The domain name. Per [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4) the overall zone name can be up to 253 characters, with each segment ("label") not exceeding 63 characters.
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
pub struct ZonesZoneName(::std::string::String);
impl ::std::ops::Deref for ZonesZoneName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZonesZoneName> for ::std::string::String {
    fn from(value: ZonesZoneName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZonesZoneName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 253usize {
            return Err("longer than 253 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^([a-zA-Z0-9][\\-a-zA-Z0-9]*\\.)+[\\-a-zA-Z0-9]{2,20}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^([a-zA-Z0-9][\\-a-zA-Z0-9]*\\.)+[\\-a-zA-Z0-9]{2,20}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZonesZoneName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZonesZoneName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZonesZoneName {
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
///DNS host at the time of switching to Cloudflare.
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
pub struct ZonesZoneOriginalDnshost(::std::string::String);
impl ::std::ops::Deref for ZonesZoneOriginalDnshost {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZonesZoneOriginalDnshost> for ::std::string::String {
    fn from(value: ZonesZoneOriginalDnshost) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZonesZoneOriginalDnshost {
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
impl ::std::convert::TryFrom<&str> for ZonesZoneOriginalDnshost {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZonesZoneOriginalDnshost {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZonesZoneOriginalDnshost {
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
///The owner of the zone.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesZoneOwner {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<ZonesIdentifier>,
    ///Name of the owner.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///The type of owner.
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<::std::string::String>,
}
///A Zones subscription information.
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesZonePlan {
    ///States if the subscription can be activated.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub can_subscribe: ::std::option::Option<bool>,
    ///The denomination of the customer.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<::std::string::String>,
    ///If this Zone is managed by another company.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub externally_managed: ::std::option::Option<bool>,
    ///How often the customer is billed.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub frequency: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<ZonesIdentifier>,
    ///States if the subscription active.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub is_subscribed: ::std::option::Option<bool>,
    ///If the legacy discount applies to this Zone.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub legacy_discount: ::std::option::Option<bool>,
    ///The legacy name of the plan.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub legacy_id: ::std::option::Option<::std::string::String>,
    ///Name of the owner.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///How much the customer is paying.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub price: ::std::option::Option<f64>,
}
///The zone status on Cloudflare.
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
pub enum ZonesZoneStatus {
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "moved")]
    Moved,
}
impl ::std::fmt::Display for ZonesZoneStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Initializing => f.write_str("initializing"),
            Self::Pending => f.write_str("pending"),
            Self::Active => f.write_str("active"),
            Self::Moved => f.write_str("moved"),
        }
    }
}
impl ::std::str::FromStr for ZonesZoneStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "initializing" => Ok(Self::Initializing),
            "pending" => Ok(Self::Pending),
            "active" => Ok(Self::Active),
            "moved" => Ok(Self::Moved),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ZonesZoneStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZonesZoneStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The root organizational unit that this zone belongs to (such as a tenant or organization).
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesZoneTenant {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<ZonesIdentifier>,
    ///The name of the Tenant account.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
///The immediate parent organizational unit that this zone belongs to (such as under a tenant or sub-organization).
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    schemars::JsonSchema
)]
pub struct ZonesZoneTenantUnit {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<ZonesIdentifier>,
}
///`ZonesZoneVanityNameServersItem`
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
pub struct ZonesZoneVanityNameServersItem(::std::string::String);
impl ::std::ops::Deref for ZonesZoneVanityNameServersItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ZonesZoneVanityNameServersItem> for ::std::string::String {
    fn from(value: ZonesZoneVanityNameServersItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ZonesZoneVanityNameServersItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 253usize {
            return Err("longer than 253 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ZonesZoneVanityNameServersItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ZonesZoneVanityNameServersItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ZonesZoneVanityNameServersItem {
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
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<u64>,
        <T as ::std::convert::TryFrom<u64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn pages_deployment_config_values_request_usage_model() -> super::PagesDeploymentConfigValuesRequestUsageModel {
        super::PagesDeploymentConfigValuesRequestUsageModel::Standard
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
