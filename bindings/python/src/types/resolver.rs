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
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct EntityResolver {
    pub(crate) key: String,
}

#[pymethods]
impl EntityResolver {
    #[new]
    fn new(key: String) -> Self {
        Self { key }
    }
}
