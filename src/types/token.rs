use crate::types::{FieldKey, PathAttributes, PathValue, Resolver, Resolvers};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Token {
    Literal(String),
    Variable(FieldKey),
}

impl Token {
    fn draw(
        &self,
        buf: &mut impl std::fmt::Write,
        fields: &PathAttributes,
        resolvers: &Resolvers,
    ) -> Result<(), crate::Error> {
        match self {
            Self::Literal(literal) => match buf.write_str(literal) {
                Ok(_) => Ok(()),
                Err(error) => Err(crate::Error::new(format!(
                    "Error while formatting token: {error}"
                ))),
            },
            Self::Variable(variable) => {
                let value = match fields.get(variable) {
                    Some(value) => value,
                    None => {
                        return Err(crate::Error::new(format!(
                            "Could not find {:?} in the fields.",
                            variable.as_str()
                        )));
                    }
                };
                let resolver = match resolvers.get(variable) {
                    Some(resolver) => resolver,
                    None => &Resolver::Default,
                };
                match value {
                    PathValue::Integer(v) => {
                        let padding = match resolver {
                            Resolver::Default => 0,
                            Resolver::Integer { padding } => *padding,
                            _ => {
                                return Err(crate::Error::new(format!(
                                    "Resolver type {resolver:?} is invalid for value {value:?}."
                                )));
                            }
                        };
                        match write!(buf, "{:0width$}", v, width = padding as usize) {
                            Ok(_) => Ok(()),
                            Err(error) => Err(crate::Error::new(format!(
                                "Error while formatting: {error}"
                            ))),
                        }
                    }
                    PathValue::String(v) => {
                        // Validate that the resolver type and the field type match
                        match resolver {
                            Resolver::Default | Resolver::String { .. } => (),
                            _ => {
                                return Err(crate::Error::new(format!(
                                    "Resolver type {resolver:?} is invalid for value {value:?}."
                                )));
                            }
                        };

                        match buf.write_str(v) {
                            Ok(_) => Ok(()),
                            Err(error) => Err(crate::Error::new(format!(
                                "Error while formatting: {error}"
                            ))),
                        }
                    }
                }
            }
        }
    }

    fn is_resolved_by(&self, fields: &PathAttributes) -> bool {
        match self {
            Self::Literal(_) => true,
            Self::Variable(variable) => fields.get(variable).is_some(),
        }
    }

    fn try_to_literal_token(
        &self,
        fields: &PathAttributes,
        resolvers: &Resolvers,
    ) -> Result<Self, crate::Error> {
        match self {
            Self::Literal(literal) => Ok(Self::Literal(literal.clone())),
            Self::Variable(variable) => {
                if fields.get(variable).is_none() {
                    Ok(Self::Variable(variable.clone()))
                } else {
                    let mut buf = String::new();
                    self.draw(&mut buf, fields, resolvers)?;
                    Ok(Self::Literal(buf))
                }
            }
        }
    }

    fn draw_regex_pattern(
        &self,
        buf: &mut impl std::fmt::Write,
        resolvers: &Resolvers,
    ) -> Result<(), crate::Error> {
        match self {
            Self::Literal(literal) => {
                let mut escape_buf = String::new();

                for character in literal.chars() {
                    if character == '\\' || character == '/' {
                        buf.write_str(&regex::escape(&escape_buf))?;
                        escape_buf.clear();
                        buf.write_str(r"[\\/]")?;
                    } else {
                        escape_buf.push(character);
                    }
                }

                buf.write_str(&regex::escape(&escape_buf))?;

                Ok(())
            }
            Self::Variable(variable) => {
                let resolver = match resolvers.get(variable) {
                    Some(resolver) => resolver,
                    None => &Resolver::Default,
                };
                buf.write_char('(')?;
                buf.write_str(&resolver.pattern())?;
                buf.write_char(')')?;
                Ok(())
            }
        }
    }

    fn draw_glob_pattern(&self, buf: &mut impl std::fmt::Write) -> Result<(), crate::Error> {
        match self {
            Token::Literal(literal) => {
                for character in literal.chars() {
                    if character == '/' || character == '\\' {
                        buf.write_char(std::path::MAIN_SEPARATOR)?;
                    } else {
                        buf.write_char(character)?;
                    }
                }
            }
            Token::Variable(_) => buf.write_char('*')?,
        };

        Ok(())
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Variable(variable) => write!(f, "{{{}}}", variable),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Tokens {
    pub(crate) tokens: Vec<Token>,
}

impl Tokens {
    pub fn new(value: &impl AsRef<str>) -> Result<Self, crate::Error> {
        let mut tokens = Vec::new();
        let value = value.as_ref();
        Self::recursive_to_tokens(value, &mut tokens)?;

        Ok(Self { tokens })
    }

    pub(crate) fn draw(
        &self,
        buf: &mut impl std::fmt::Write,
        fields: &PathAttributes,
        resolvers: &Resolvers,
    ) -> Result<(), crate::Error> {
        for token in self.tokens.iter() {
            token.draw(buf, fields, resolvers)?;
        }
        Ok(())
    }

    pub(crate) fn is_resolved_by(&self, fields: &PathAttributes) -> bool {
        for token in self.tokens.iter() {
            if !token.is_resolved_by(fields) {
                return false;
            }
        }

        true
    }

    pub(crate) fn try_to_literal_token(
        &self,
        fields: &PathAttributes,
        resolvers: &Resolvers,
    ) -> Result<Self, crate::Error> {
        let mut tokens = Vec::new();

        for token in self.tokens.iter() {
            tokens.push(token.try_to_literal_token(fields, resolvers)?);
        }

        Ok(Self { tokens })
    }

    pub(crate) fn draw_regex_pattern(
        &self,
        buf: &mut impl std::fmt::Write,
        resolvers: &Resolvers,
    ) -> Result<(), crate::Error> {
        for token in self.tokens.iter() {
            token.draw_regex_pattern(buf, resolvers)?;
        }
        Ok(())
    }

    pub(crate) fn draw_glob_pattern(
        &self,
        buf: &mut impl std::fmt::Write,
    ) -> Result<(), crate::Error> {
        for token in self.tokens.iter() {
            token.draw_glob_pattern(buf)?;
        }
        Ok(())
    }

    pub(crate) fn has_variable_tokens(&self) -> bool {
        for token in self.tokens.iter() {
            if let Token::Variable(_) = token {
                return true;
            }
        }

        false
    }

    fn recursive_to_tokens(text: &str, tokens: &mut Vec<Token>) -> Result<(), crate::Error> {
        let (literal, variable, after) = Self::parse(text)?;

        if !literal.is_empty() {
            tokens.push(Token::Literal(literal.to_string()));
        }

        if !variable.is_empty() {
            tokens.push(Token::Variable(variable.try_into()?));
        }

        if !after.is_empty() {
            Self::recursive_to_tokens(after, tokens)?;
        }

        Ok(())
    }

    fn parse(text: &str) -> Result<(&str, &str, &str), crate::Error> {
        let start_index = match text.find('{') {
            Some(start_index) => start_index,
            None => match text.find('}') {
                Some(_) => return Err(crate::Error::new("Parse Error: Missing opening '{'")),
                None => return Ok((text, "", "")),
            },
        };
        let (before, after) = text.split_at(start_index);

        if before.find('}').is_some() {
            return Err(crate::Error::new("Parse Error: Missing opening '{'"));
        }

        let end_index = match after.find('}') {
            Some(end_index) => end_index,
            None => return Err(crate::Error::new("Parse Error: Missing closing '}'")),
        };
        let (inside, after) = after.split_at(end_index + 1);
        let inside = &inside[1..inside.len() - 1].trim();

        if !FieldKey::validate(inside) {
            return Err(crate::Error::new("Parse Error: Invalid variable"));
        }

        Ok((before, inside, after))
    }
}

impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.tokens.iter() {
            write!(f, "{}", token)?;
        }

        Ok(())
    }
}

impl TryFrom<std::path::PathBuf> for Tokens {
    type Error = crate::Error;

    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
        Self::new(&value.to_string_lossy())
    }
}

impl TryFrom<&std::path::PathBuf> for Tokens {
    type Error = crate::Error;

    fn try_from(value: &std::path::PathBuf) -> Result<Self, Self::Error> {
        Self::new(&value.to_string_lossy())
    }
}

impl TryFrom<&std::path::Path> for Tokens {
    type Error = crate::Error;

    fn try_from(value: &std::path::Path) -> Result<Self, Self::Error> {
        Self::new(&value.to_string_lossy())
    }
}

impl TryFrom<String> for Tokens {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl TryFrom<&String> for Tokens {
    type Error = crate::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl TryFrom<&str> for Tokens {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("", "")]
    #[case("test", "test")]
    #[case("123", "123")]
    fn test_token_draw_literal_success(#[case] input: &str, #[case] expected: &str) {
        let token = Token::Literal(input.to_string());

        let mut result = String::new();
        token
            .draw(&mut result, &PathAttributes::new(), &Resolvers::new())
            .unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_token_draw_literal_failure_cannot_write_into_buf() {
        struct TestWriter;

        impl std::fmt::Write for TestWriter {
            fn write_str(&mut self, _text: &str) -> std::fmt::Result {
                Err(std::fmt::Error)
            }
        }

        let token = Token::Literal("test".to_string());
        let mut writer = TestWriter;
        let err = token
            .draw(&mut writer, &PathAttributes::new(), &Resolvers::new())
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "Error while formatting token: an error occurred when formatting an argument"
        );
    }

    #[rstest::rstest]
    #[case("test_str", "test")]
    #[case("test_str_default", "test")]
    #[case("test_int_no_zpad", "1")]
    #[case("test_int_with_zpad", "001")]
    fn test_token_draw_variable_success(#[case] input: &str, #[case] expected: &str) {
        let token = Token::Variable(input.try_into().unwrap());

        let mut result = String::new();
        let mut fields = PathAttributes::new();
        fields.insert("test_str".try_into().unwrap(), "test".into());
        fields.insert("test_str_default".try_into().unwrap(), "test".into());
        fields.insert("test_int_no_zpad".try_into().unwrap(), 1u8.into());
        fields.insert("test_int_with_zpad".try_into().unwrap(), 1u8.into());
        let mut resolvers = Resolvers::new();
        resolvers.insert(
            "test_str".try_into().unwrap(),
            Resolver::String { pattern: None },
        );
        resolvers.insert(
            "test_int_no_zpad".try_into().unwrap(),
            Resolver::Integer { padding: 0 },
        );
        resolvers.insert(
            "test_int_with_zpad".try_into().unwrap(),
            Resolver::Integer { padding: 3 },
        );

        token.draw(&mut result, &fields, &resolvers).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_token_draw_variable_failure_missing_field() {
        let token = Token::Variable("test".try_into().unwrap());
        let mut writer = String::new();
        let err = token
            .draw(&mut writer, &PathAttributes::new(), &Resolvers::new())
            .unwrap_err();

        assert_eq!(err.to_string(), "Could not find \"test\" in the fields.");
    }

    #[test]
    fn test_token_draw_variable_failure_int_resolver_mismatch() {
        let token = Token::Variable("test".try_into().unwrap());
        let mut writer = String::new();
        let fields = {
            let mut fields = PathAttributes::new();
            fields.insert("test".try_into().unwrap(), 1u8.into());
            fields
        };
        let resolvers = {
            let mut resolvers = Resolvers::new();
            resolvers.insert(
                "test".try_into().unwrap(),
                Resolver::String { pattern: None },
            );
            resolvers
        };
        let err = token.draw(&mut writer, &fields, &resolvers).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Resolver type String { pattern: None } is invalid for value Integer(1)."
        );
    }

    #[test]
    fn test_token_draw_variable_failure_str_resolver_mismatch() {
        let token = Token::Variable("test".try_into().unwrap());
        let mut writer = String::new();
        let fields = {
            let mut fields = PathAttributes::new();
            fields.insert("test".try_into().unwrap(), "test".into());
            fields
        };
        let resolvers = {
            let mut resolvers = Resolvers::new();
            resolvers.insert("test".try_into().unwrap(), Resolver::Integer { padding: 1 });
            resolvers
        };
        let err = token.draw(&mut writer, &fields, &resolvers).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Resolver type Integer { padding: 1 } is invalid for value String(\"test\")."
        );
    }

    #[rstest::rstest]
    #[case("test_str")]
    #[case("test_int")]
    fn test_token_draw_variable_failure_write_err(#[case] input: &str) {
        struct TestWriter;

        impl std::fmt::Write for TestWriter {
            fn write_str(&mut self, _text: &str) -> std::fmt::Result {
                Err(std::fmt::Error)
            }
        }

        let mut fields = PathAttributes::new();
        fields.insert("test_str".try_into().unwrap(), "test".into());
        fields.insert("test_int".try_into().unwrap(), 1u8.into());
        let token = Token::Variable(input.try_into().unwrap());
        let mut writer = TestWriter;
        let err = token
            .draw(&mut writer, &fields, &Resolvers::new())
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "Error while formatting: an error occurred when formatting an argument"
        );
    }

    #[rstest::rstest]
    #[case("", ("", "", ""))]
    #[case("abc", ("abc", "", ""))]
    #[case("{abc}", ("", "abc", ""))]
    #[case("{abc123}", ("", "abc123", ""))]
    #[case("{abc.def}", ("", "abc.def", ""))]
    #[case("{ abc }", ("", "abc", ""))]
    #[case("abc{def}", ("abc", "def", ""))]
    #[case("abc {def}", ("abc ", "def", ""))]
    #[case("{abc}def", ("", "abc", "def"))]
    #[case("{abc}{def}", ("", "abc", "{def}"))]
    fn test_tokens_parse_success(#[case] input: &str, #[case] expected: (&str, &str, &str)) {
        let result = Tokens::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest::rstest]
    #[case("{", "Missing closing '}'")]
    #[case("}", "Missing opening '{'")]
    #[case("}{", "Missing opening '{'")]
    #[case("}{abc}", "Missing opening '{'")]
    #[case("{}", "Invalid variable")]
    #[case("{ }", "Invalid variable")]
    #[case("{123}", "Invalid variable")]
    #[case("{abc.123}", "Invalid variable")]
    #[case("{abc.}", "Invalid variable")]
    #[case("{abc..}", "Invalid variable")]
    #[case("{abc..def}", "Invalid variable")]
    #[case("{abc.def.}", "Invalid variable")]
    #[case("{abc.def..}", "Invalid variable")]
    #[case("{{abc}}", "Invalid variable")]
    fn test_tokens_parse_failure(#[case] input: &str, #[case] expected: &str) {
        let result = Tokens::parse(input).unwrap_err();

        assert_eq!(result.to_string(), format!("Parse Error: {expected}"));
    }

    #[rstest::rstest]
    #[case("", &[])]
    #[case("abc", &[Token::Literal("abc".to_string())])]
    #[case("{abc}", &[Token::Variable("abc".try_into().unwrap())])]
    #[case("{abc123}", &[Token::Variable("abc123".try_into().unwrap())])]
    #[case("{abc.def}", &[Token::Variable("abc.def".try_into().unwrap())])]
    #[case("{ abc }", &[Token::Variable("abc".try_into().unwrap())])]
    #[case("abc{def}", &[Token::Literal("abc".to_string()), Token::Variable("def".try_into().unwrap())])]
    #[case("abc {def}", &[Token::Literal("abc ".to_string()), Token::Variable("def".try_into().unwrap())])]
    #[case("{abc}def", &[Token::Variable("abc".try_into().unwrap()), Token::Literal("def".try_into().unwrap())])]
    #[case("{abc}{def}", &[Token::Variable("abc".try_into().unwrap()), Token::Variable("def".try_into().unwrap())])]
    fn test_tokens_new_success(#[case] input: &str, #[case] expected: &[Token]) {
        let result = Tokens::new(&input).unwrap();
        assert_eq!(result.tokens, expected);
    }

    #[rstest::rstest]
    #[case("{", "Missing closing '}'")]
    #[case("}", "Missing opening '{'")]
    #[case("}{", "Missing opening '{'")]
    #[case("}{abc}", "Missing opening '{'")]
    #[case("{}", "Invalid variable")]
    #[case("{ }", "Invalid variable")]
    #[case("{123}", "Invalid variable")]
    #[case("{abc.123}", "Invalid variable")]
    #[case("{abc.}", "Invalid variable")]
    #[case("{abc..}", "Invalid variable")]
    #[case("{abc..def}", "Invalid variable")]
    #[case("{abc.def.}", "Invalid variable")]
    #[case("{abc.def..}", "Invalid variable")]
    #[case("{{abc}}", "Invalid variable")]
    fn test_tokens_new_failure(#[case] input: &str, #[case] expected: &str) {
        let result = Tokens::new(&input).unwrap_err();

        assert_eq!(result.to_string(), format!("Parse Error: {expected}"));
    }

    #[rstest::rstest]
    #[case("{test_str}", "test")]
    #[case("{test_int}", "001")]
    #[case("abc {test_str}", "abc test")]
    #[case("abc {test_int}", "abc 001")]
    #[case("{test_str} abc", "test abc")]
    #[case("{test_int} abc", "001 abc")]
    fn test_tokens_draw_success(#[case] input: &str, #[case] expected: &str) {
        let tokens = Tokens::new(&input).unwrap();

        let fields = {
            let mut fields = PathAttributes::new();
            fields.insert("test_str".try_into().unwrap(), "test".into());
            fields.insert("test_int".try_into().unwrap(), 1u8.into());
            fields
        };

        let resolvers = {
            let mut resolvers = Resolvers::new();
            resolvers.insert(
                "test_str".try_into().unwrap(),
                Resolver::String { pattern: None },
            );
            resolvers.insert(
                "test_int".try_into().unwrap(),
                Resolver::Integer { padding: 3 },
            );
            resolvers
        };

        let mut result = String::new();
        tokens.draw(&mut result, &fields, &resolvers).unwrap();

        assert_eq!(result, expected);
    }

    #[rstest::rstest]
    #[case("{test_str}", "test_str")]
    #[case("{test_int}", "test_int")]
    #[case("abc {test_str}", "test_str")]
    #[case("abc {test_int}", "test_int")]
    #[case("{test_str} abc", "test_str")]
    #[case("{test_int} abc", "test_int")]
    fn test_tokens_draw_failure(#[case] input: &str, #[case] expected: &str) {
        let tokens = Tokens::new(&input).unwrap();

        let mut writer = String::new();
        let result = tokens
            .draw(&mut writer, &PathAttributes::new(), &Resolvers::new())
            .unwrap_err();

        assert_eq!(
            result.to_string(),
            format!("Could not find {expected:?} in the fields.")
        );
    }

    #[rstest::rstest]
    #[case("", &[])]
    #[case("abc", &[Token::Literal("abc".to_string())])]
    #[case("{abc}", &[Token::Variable("abc".try_into().unwrap())])]
    #[case("{abc123}", &[Token::Variable("abc123".try_into().unwrap())])]
    #[case("{abc.def}", &[Token::Variable("abc.def".try_into().unwrap())])]
    #[case("{ abc }", &[Token::Variable("abc".try_into().unwrap())])]
    #[case("abc{def}", &[Token::Literal("abc".to_string()), Token::Variable("def".try_into().unwrap())])]
    #[case("abc {def}", &[Token::Literal("abc ".to_string()), Token::Variable("def".try_into().unwrap())])]
    #[case("{abc}def", &[Token::Variable("abc".try_into().unwrap()), Token::Literal("def".to_string())])]
    #[case("{abc}{def}", &[Token::Variable("abc".try_into().unwrap()), Token::Variable("def".try_into().unwrap())])]
    fn test_tokens_try_from_success(#[case] input: &str, #[case] expected: &[Token]) {
        // From<&str>
        let tokens = Tokens::try_from(input).unwrap();
        assert_eq!(tokens.tokens, expected);

        // From<String>
        let tokens = Tokens::try_from(input.to_string()).unwrap();
        assert_eq!(tokens.tokens, expected);

        // From<&String>
        let tokens = Tokens::try_from(&input.to_string()).unwrap();
        assert_eq!(tokens.tokens, expected);

        // From<PathBuf>
        let tokens = Tokens::try_from(std::path::PathBuf::from(input)).unwrap();
        assert_eq!(tokens.tokens, expected);

        // From<&PathBuf>
        let tokens = Tokens::try_from(&std::path::PathBuf::from(input)).unwrap();
        assert_eq!(tokens.tokens, expected);

        // From<&Path>
        let tokens = Tokens::try_from(std::path::PathBuf::from(input).as_path()).unwrap();
        assert_eq!(tokens.tokens, expected);
    }
}
