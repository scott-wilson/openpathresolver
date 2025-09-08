use pyo3::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[pyclass]
pub struct FieldKey {
    pub(crate) inner: base_openpathresolver::FieldKey,
}

impl TryFrom<String> for FieldKey {
    type Error = PyErr;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl TryFrom<FieldKey> for base_openpathresolver::FieldKey {
    type Error = base_openpathresolver::Error;

    fn try_from(value: FieldKey) -> Result<Self, Self::Error> {
        Ok(value.inner)
    }
}

impl TryFrom<base_openpathresolver::FieldKey> for FieldKey {
    type Error = PyErr;

    fn try_from(value: base_openpathresolver::FieldKey) -> Result<Self, Self::Error> {
        Ok(Self { inner: value })
    }
}

#[pymethods]
impl FieldKey {
    #[new]
    fn new(key: &str) -> PyResult<Self> {
        Ok(Self {
            inner: crate::to_py_result(base_openpathresolver::FieldKey::try_from(key))?,
        })
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.inner.hash(&mut hasher);
        hasher.finish()
    }

    fn __str__(&self) -> &str {
        self.inner.as_str()
    }
}
