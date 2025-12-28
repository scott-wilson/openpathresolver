use serde::Deserialize;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Resolver {
    Default,
    #[serde(
        serialize_with = "serialize_regex",
        deserialize_with = "deserialize_regex"
    )]
    String {
        pattern: Option<regex::Regex>,
    },
    Integer {
        padding: u8,
    },
}

impl Resolver {
    pub(crate) fn pattern(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Self::Default => ".+?".into(),
            Self::String { pattern } => match pattern {
                Some(pattern) => pattern.to_string().into(),
                None => ".+?".into(),
            },
            Self::Integer { padding } => format!("\\d{{{},}}?", padding.max(&1)).into(),
        }
    }

    pub(crate) fn to_path_value(&self, value: &str) -> Result<crate::PathValue, crate::Error> {
        match self {
            Self::Default => Ok(crate::PathValue::String(value.into())),
            Self::String { .. } => Ok(crate::PathValue::String(value.into())),
            Self::Integer { .. } => Ok(crate::PathValue::Integer(value.parse()?)),
        }
    }
}

fn serialize_regex<S: serde::Serializer>(
    regex: &Option<regex::Regex>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match regex {
        Some(regex) => serializer.serialize_some(regex.as_str()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_regex<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<regex::Regex>, D::Error> {
    let regex = match Option::<String>::deserialize(deserializer)? {
        // TODO: Cache the compiled regex
        Some(regex) => Some(regex::Regex::new(&regex).map_err(serde::de::Error::custom)?),
        None => None,
    };

    Ok(regex)
}
