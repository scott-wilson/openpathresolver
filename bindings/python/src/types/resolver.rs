use pyo3::prelude::*;

/// A string resolver marks a placeholder as a string with an optional shape.
#[derive(Debug, FromPyObject)]
#[pyclass]
pub struct StringResolver {
    pub(crate) pattern: Option<String>,
}

#[pymethods]
impl StringResolver {
    /// Create a new string resolver.
    ///
    /// Args:
    ///     pattern: The regex pattern to define the shape of the string.
    #[new]
    fn new(pattern: Option<String>) -> Self {
        Self { pattern }
    }

    /// The shape of the string.
    #[getter]
    fn pattern(&self) -> &Option<String> {
        &self.pattern
    }
}

/// An integer resolver marks a placeholder as an integer with zero padding.
#[derive(Debug, FromPyObject)]
#[pyclass]
pub struct IntegerResolver {
    pub(crate) padding: u8,
}

#[pymethods]
impl IntegerResolver {
    /// Create a new integer resolver.
    ///
    /// Args:
    ///     padding: The zero padding for the integer.
    #[new]
    fn new(padding: u8) -> Self {
        Self { padding }
    }

    /// The zero padding for the integer.
    #[getter]
    fn padding(&self) -> u8 {
        self.padding
    }
}
