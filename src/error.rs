/// Error for the workspace or path resolvers.
#[derive(Debug, thiserror::Error)]
#[error("{msg}")]
pub struct Error {
    msg: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send>>,
}

macro_rules! impl_from {
    ($($e:ty: $t:ty => $msg:expr),+ $(,)?) => {
        $(impl From<$t> for $e {
            fn from(value: $t) -> Self {
                Self {
                    msg: $msg.into(),
                    source: Some(Box::new(value)),
                }
            }
        })+
    };
}

impl_from!(
    Error: std::fmt::Error => "Formatting error.",
    Error: regex::Error => "Error while creating regex.",
    Error: std::num::TryFromIntError => "Error while converting integer type.",
    Error: std::num::ParseIntError => "Error while parsing integer.",
    Error: std::io::Error => "IO Error.",
    Error: glob::GlobError => "Glob Error.",
    Error: glob::PatternError => "Glob Pattern Error.",
);

impl Error {
    /// Create a new error.
    pub fn new<T: Into<String>>(msg: T) -> Self {
        Self {
            msg: msg.into(),
            source: None,
        }
    }
}
