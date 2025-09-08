use crate::types::{TemplateAttributes, TemplateEntity};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PathValue {
    Integer(u16),
    String(String),
}

impl From<&str> for PathValue {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for PathValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u16> for PathValue {
    fn from(value: u16) -> Self {
        Self::Integer(value)
    }
}

impl From<u8> for PathValue {
    fn from(value: u8) -> Self {
        Self::Integer(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TemplateValue {
    None,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<TemplateValue>),
    Object(TemplateAttributes),
    Entity(TemplateEntity),
}

impl From<bool> for TemplateValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl TryFrom<u64> for TemplateValue {
    type Error = crate::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self::Integer(value.try_into()?))
    }
}

impl From<i64> for TemplateValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<i32> for TemplateValue {
    fn from(value: i32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u32> for TemplateValue {
    fn from(value: u32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i16> for TemplateValue {
    fn from(value: i16) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u16> for TemplateValue {
    fn from(value: u16) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i8> for TemplateValue {
    fn from(value: i8) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u8> for TemplateValue {
    fn from(value: u8) -> Self {
        Self::Integer(value.into())
    }
}

impl From<f64> for TemplateValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<f32> for TemplateValue {
    fn from(value: f32) -> Self {
        Self::Float(value.into())
    }
}

impl From<String> for TemplateValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for TemplateValue {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<Vec<TemplateValue>> for TemplateValue {
    fn from(value: Vec<TemplateValue>) -> Self {
        Self::Array(value)
    }
}

impl From<TemplateAttributes> for TemplateValue {
    fn from(value: TemplateAttributes) -> Self {
        Self::Object(value)
    }
}

impl From<TemplateEntity> for TemplateValue {
    fn from(value: TemplateEntity) -> Self {
        Self::Entity(value)
    }
}
