#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Formatting error")]
    FormatError(#[from] std::fmt::Error),
    #[error("Error while accessing field {0}")]
    FieldError(String),
    #[error("Resolver {resolver:?} is incompatible with value {value:?}")]
    ResolverTypeMismatchError {
        resolver: crate::Resolver,
        value: crate::PathValue,
    },
    #[error("Error with template {0}")]
    TemplateError(#[from] minijinja::Error),
    #[error("Parent {0} does not exist")]
    MissingParentError(crate::FieldKey),
    #[error("Error while parsing: {0}")]
    ParseError(&'static str),
    #[error("Error while creating regex: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Error while converting integer type: {0}")]
    IntegerConvertTypeError(#[from] std::num::TryFromIntError),
    #[error("Error while parsing integer: {0}")]
    ParseIntegerError(#[from] std::num::ParseIntError),
    #[error("The path item {item} parent {parent} causes a infinite recursion")]
    InfiniteRecursionError {
        item: crate::FieldKey,
        parent: crate::FieldKey,
    },
    #[error("Could not find item {0}")]
    MissingItemError(crate::FieldKey),
    #[error("Field {key} exists in multiple variable parts in the path, but the values are different: {value:?} != {other_value:?}")]
    MismatchedFieldError {
        key: crate::FieldKey,
        value: crate::PathValue,
        other_value: crate::PathValue,
    },
    #[error("Cannot resolve a variable root path")]
    VariableRootPathError,
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Runtime Error: {0}")]
    RuntimeError(String),
}
