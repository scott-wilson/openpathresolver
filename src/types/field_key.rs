struct FieldKeyVisitor;

impl<'de> serde::de::Visitor<'de> for FieldKeyVisitor {
    type Value = FieldKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid field key")
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        FieldKey::new(value).map_err(|err| E::custom(format!("{}", err)))
    }
}

/// A field key is a valid key to a field.
///
/// This can be used for path parts keys, the parent key, etc.
///
/// # Validation
///
/// - The key must not be empty
/// - The first character of the key must be any ASCII alphabetic character or `_`.
/// - The remainder characters must be any ASCII alphanumeric character or `_`.
/// - Sections can be split with `.`. The above rules then apply to each section.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldKey {
    key: String,
}

impl serde::Serialize for FieldKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.key)
    }
}

impl<'de> serde::Deserialize<'de> for FieldKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(FieldKeyVisitor)
    }
}

impl std::fmt::Display for FieldKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl FieldKey {
    /// Create a new field key.
    pub fn new(key: &str) -> Result<Self, crate::Error> {
        let key = key.to_lowercase();
        let mut parsed_key = String::new();

        if !Self::validate(&key) {
            return Err(crate::Error::new("Invalid field key"));
        }

        Self::parse(&key, &mut parsed_key)?;
        Ok(Self { key: parsed_key })
    }

    /// Access the internal key string.
    pub fn as_str(&self) -> &str {
        &self.key
    }

    fn parse(text: &str, writer: &mut impl std::fmt::Write) -> Result<(), crate::Error> {
        let split_index = match text.find('.') {
            Some(index) => index,
            None => {
                writer.write_str(text)?;

                return Ok(());
            }
        };
        let (before, after) = text.split_at(split_index);
        writer.write_str(before)?;

        let after = &after[1..];

        if !after.is_empty() {
            writer.write_char('.')?;
            Self::parse(after, writer)?;
        }

        Ok(())
    }

    pub(crate) fn validate(text: &str) -> bool {
        if text.is_empty() {
            return false;
        }

        let split_index = match text.find('.') {
            Some(index) => index,
            None => {
                if !Self::validate_part(text) {
                    return false;
                }

                return true;
            }
        };
        let (before, after) = text.split_at(split_index);

        if !Self::validate_part(before) {
            return false;
        }

        let after = &after[1..];

        if !Self::validate(after) {
            return false;
        }

        true
    }

    fn validate_part(text: &str) -> bool {
        if text.is_empty() {
            return false;
        }

        let first_char = text.chars().next().unwrap();

        if !(first_char.is_ascii_alphabetic() || first_char == '_') {
            return false;
        }

        for character in text.chars().skip(1) {
            if !(character.is_ascii_alphanumeric() || character == '_') {
                return false;
            }
        }

        true
    }
}

impl TryFrom<&str> for FieldKey {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&String> for FieldKey {
    type Error = crate::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for FieldKey {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl TryFrom<&FieldKey> for FieldKey {
    type Error = crate::Error;

    fn try_from(value: &FieldKey) -> Result<Self, Self::Error> {
        Ok(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("test", "test")]
    #[case("Test", "test")]
    #[case("_test", "_test")]
    #[case("test_", "test_")]
    #[case("_1test", "_1test")]
    #[case("abc.def", "abc.def")]
    #[case("abc.def.ghi", "abc.def.ghi")]
    #[case("abc123.def456", "abc123.def456")]
    #[case("_abc._def", "_abc._def")]
    fn test_field_key_new_success(#[case] input: &str, #[case] expected: &str) {
        // New
        let result = FieldKey::new(input).unwrap();
        assert_eq!(&result.key, expected);

        // From<&str>
        let result = FieldKey::try_from(input).unwrap();
        assert_eq!(&result.key, expected);

        // From<String>
        let result = FieldKey::try_from(input.to_string()).unwrap();
        assert_eq!(&result.key, expected);

        // From<&String>
        let result = FieldKey::try_from(&input.to_string()).unwrap();
        assert_eq!(&result.key, expected);
    }

    #[rstest::rstest]
    #[case("", "Invalid field key")]
    #[case(" abc ", "Invalid field key")]
    #[case("1", "Invalid field key")]
    #[case("abc.", "Invalid field key")]
    #[case("abc.123.", "Invalid field key")]
    #[case("abc.def.", "Invalid field key")]
    #[case("abc.def.123", "Invalid field key")]
    #[case("abc..def", "Invalid field key")]
    #[case(".abc", "Invalid field key")]
    #[case("1abc", "Invalid field key")]
    #[case("!", "Invalid field key")]
    #[case("a!", "Invalid field key")]
    #[case("abc.!", "Invalid field key")]
    #[case("abc.d!", "Invalid field key")]
    #[case(".", "Invalid field key")]
    #[case("..", "Invalid field key")]
    fn test_tokens_parse_failure(#[case] input: &str, #[case] expected: &str) {
        // New
        let result = FieldKey::new(input).unwrap_err();
        assert_eq!(result.to_string(), expected);

        // From<&str>
        let result = FieldKey::try_from(input).unwrap_err();

        assert_eq!(result.to_string(), expected);

        // From<String>
        let result = FieldKey::try_from(input.to_string()).unwrap_err();

        assert_eq!(result.to_string(), expected);

        // From<&String>
        let result = FieldKey::try_from(&input.to_string()).unwrap_err();

        assert_eq!(result.to_string(), expected);
    }

    #[rstest::rstest]
    #[case("test", "test")]
    #[case("Test", "test")]
    #[case("_test", "_test")]
    #[case("_1test", "_1test")]
    #[case("abc.def", "abc.def")]
    #[case("abc.def.ghi", "abc.def.ghi")]
    #[case("abc123.def456", "abc123.def456")]
    #[case("_abc._def", "_abc._def")]
    fn test_field_key_display_success(#[case] input: &str, #[case] expected: &str) {
        // New
        let result = FieldKey::new(input).unwrap();
        assert_eq!(format!("{}", result), expected);
    }

    #[rstest::rstest]
    #[case("test", "test")]
    #[case("test", "Test")]
    #[case("abc.def", "abc.def")]
    fn test_field_key_eq(#[case] input: &str, #[case] other: &str) {
        let input = FieldKey::new(input).unwrap();
        let other = FieldKey::new(other).unwrap();

        assert_eq!(input, other);
    }

    #[rstest::rstest]
    #[case("test", "test1")]
    #[case("test", "Test1")]
    #[case("abc.def", "abc")]
    #[case("abc", "abc.def")]
    #[case("abc.def", "abc.def1")]
    fn test_field_key_ne(#[case] input: &str, #[case] other: &str) {
        let input = FieldKey::new(input).unwrap();
        let other = FieldKey::new(other).unwrap();

        assert_ne!(input, other);
    }
}
