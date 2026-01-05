use pyo3::prelude::*;
use std::hash::{Hash, Hasher};

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
#[derive(Clone, PartialEq, Eq, Hash)]
#[pyclass]
pub struct FieldKey {
    pub(crate) inner: base_openpathresolver::FieldKey,
}

impl std::fmt::Debug for FieldKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
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

    fn __repr__(&self) -> String {
        format!("FieldKey({:?})", self.inner.as_str())
    }
}
