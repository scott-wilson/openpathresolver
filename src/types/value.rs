use crate::types::TemplateAttributes;

macro_rules! impl_from {
    ($($e:ty: $t:ty => $v:ident),+ $(,)?) => {
        $(impl From<$t> for $e {
            fn from(value: $t) -> Self {
                Self::$v(value.into())
            }
        })+
    };
}

/// A value for a path.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PathValue {
    /// An integer.
    Integer(u16),
    /// A string.
    String(String),
}

impl_from!(
    PathValue: &str => String,
    PathValue: String => String,
    PathValue: u8 => Integer,
    PathValue: u16 => Integer,
);

/// A value for a template.
///
/// This is similar to a JSON type.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TemplateValue {
    /// A null value.
    None,
    /// A boolean.
    Bool(bool),
    /// An integer.
    Integer(i64),
    /// A float.
    Float(f64),
    /// A string.
    String(String),
    /// An array of template values.
    Array(Vec<TemplateValue>),
    /// An object or mapping of template values.
    Object(TemplateAttributes),
}

impl_from!(
    TemplateValue: bool => Bool,
    TemplateValue: u8 => Integer,
    TemplateValue: i8 => Integer,
    TemplateValue: u16 => Integer,
    TemplateValue: i16 => Integer,
    TemplateValue: u32 => Integer,
    TemplateValue: i32 => Integer,
    TemplateValue: i64 => Integer,
    TemplateValue: f32 => Float,
    TemplateValue: f64 => Float,
    TemplateValue: &str => String,
    TemplateValue: String => String,
    TemplateValue: Vec<TemplateValue> => Array,
    TemplateValue: TemplateAttributes => Object,
);

impl TryFrom<u64> for TemplateValue {
    type Error = crate::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self::Integer(value.try_into()?))
    }
}

/// A value for metadata.
///
/// This is similar to a JSON type.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MetadataValue {
    /// A null value.
    None,
    /// A boolean.
    Bool(bool),
    /// An integer.
    Integer(i64),
    /// A float.
    Float(f64),
    /// A string.
    String(String),
    /// An array of metadata values.
    Array(Vec<MetadataValue>),
    /// An object or mapping of metadata values.
    Object(std::collections::HashMap<String, MetadataValue>),
}

impl_from!(
    MetadataValue: bool => Bool,
    MetadataValue: u8 => Integer,
    MetadataValue: i8 => Integer,
    MetadataValue: u16 => Integer,
    MetadataValue: i16 => Integer,
    MetadataValue: u32 => Integer,
    MetadataValue: i32 => Integer,
    MetadataValue: i64 => Integer,
    MetadataValue: f32 => Float,
    MetadataValue: f64 => Float,
    MetadataValue: &str => String,
    MetadataValue: String => String,
    MetadataValue: Vec<MetadataValue> => Array,
    MetadataValue: std::collections::HashMap<String, MetadataValue> => Object,
);

impl TryFrom<u64> for MetadataValue {
    type Error = crate::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self::Integer(value.try_into()?))
    }
}
