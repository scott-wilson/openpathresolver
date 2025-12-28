use pyo3::prelude::*;

#[derive(Debug, FromPyObject)]
#[pyclass]
pub struct StringResolver {
    pub(crate) pattern: Option<String>,
}

#[pymethods]
impl StringResolver {
    #[new]
    fn new(pattern: Option<String>) -> Self {
        Self { pattern }
    }

    #[getter]
    fn pattern(&self) -> &Option<String> {
        &self.pattern
    }
}

#[derive(Debug, FromPyObject)]
#[pyclass]
pub struct IntegerResolver {
    pub(crate) padding: u8,
}

#[pymethods]
impl IntegerResolver {
    #[new]
    fn new(padding: u8) -> Self {
        Self { padding }
    }

    #[getter]
    fn padding(&self) -> u8 {
        self.padding
    }
}
