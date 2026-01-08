use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

/// A value for a path.
#[derive(Clone)]
pub struct PathValue {
    pub(crate) inner: base_openpathresolver::PathValue,
}

impl std::fmt::Debug for PathValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'py> FromPyObject<'_, 'py> for PathValue {
    type Error = PyErr;

    fn extract(value: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(value) = value.extract::<u16>() {
            let inner = base_openpathresolver::PathValue::Integer(value);
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<String>() {
            let inner = base_openpathresolver::PathValue::String(value);
            Ok(Self { inner })
        } else {
            let name = value.get_type().name()?;
            Err(PyTypeError::new_err(format!(
                "Type '{}' is not supported. Expected an integer or a string.",
                name
            )))
        }
    }
}

impl<'py> IntoPyObject<'py> for PathValue {
    type Target = PyAny;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        path_value_to_py_any(py, self.inner)
    }
}

impl From<base_openpathresolver::PathValue> for PathValue {
    fn from(value: base_openpathresolver::PathValue) -> Self {
        Self { inner: value }
    }
}

impl From<PathValue> for base_openpathresolver::PathValue {
    fn from(value: PathValue) -> Self {
        value.inner
    }
}

/// A value for a template.
///
/// This is similar to a JSON type.
#[derive(Clone)]
pub struct TemplateValue {
    pub(crate) inner: base_openpathresolver::TemplateValue,
}

impl std::fmt::Debug for TemplateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'py> FromPyObject<'_, 'py> for TemplateValue {
    type Error = PyErr;

    fn extract(value: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(value) = value.extract::<Self>() {
            Ok(value)
        } else if value.is_none() {
            let inner = base_openpathresolver::TemplateValue::None;
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<bool>() {
            let inner = base_openpathresolver::TemplateValue::Bool(value);
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<f64>() {
            let inner = base_openpathresolver::TemplateValue::Float(value);
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<i64>() {
            let inner = base_openpathresolver::TemplateValue::Integer(value);
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<String>() {
            let inner = base_openpathresolver::TemplateValue::String(value);
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<Vec<TemplateValue>>() {
            let inner = base_openpathresolver::TemplateValue::Array(
                value.iter().map(|v| v.inner.clone()).collect(),
            );
            Ok(Self { inner })
        } else if let Ok(value) =
            value.extract::<std::collections::HashMap<String, TemplateValue>>()
        {
            let mut attributes = std::collections::HashMap::new();

            for (k, v) in value.iter() {
                attributes.insert(crate::to_py_result(k.try_into())?, v.inner.clone());
            }
            let inner = base_openpathresolver::TemplateValue::Object(attributes);
            Ok(Self { inner })
        } else {
            let name = value.get_type().name()?;
            Err(PyTypeError::new_err(format!(
                "Type '{}' is not supported.",
                name
            )))
        }
    }
}

impl<'py> IntoPyObject<'py> for TemplateValue {
    type Target = PyAny;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        template_value_to_py_any(py, self.inner)
    }
}

impl From<base_openpathresolver::TemplateValue> for TemplateValue {
    fn from(value: base_openpathresolver::TemplateValue) -> Self {
        Self { inner: value }
    }
}

impl From<TemplateValue> for base_openpathresolver::TemplateValue {
    fn from(value: TemplateValue) -> Self {
        value.inner
    }
}

/// A value for metadata.
///
/// This is similar to a JSON type.
#[derive(Clone)]
pub struct MetadataValue {
    pub(crate) inner: base_openpathresolver::MetadataValue,
}

impl std::fmt::Debug for MetadataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'py> FromPyObject<'_, 'py> for MetadataValue {
    type Error = PyErr;

    fn extract(value: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(value) = value.extract::<Self>() {
            Ok(value)
        } else if value.is_none() {
            let inner = base_openpathresolver::MetadataValue::None;
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<bool>() {
            let inner = base_openpathresolver::MetadataValue::Bool(value);
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<f64>() {
            let inner = base_openpathresolver::MetadataValue::Float(value);
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<i64>() {
            let inner = base_openpathresolver::MetadataValue::Integer(value);
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<String>() {
            let inner = base_openpathresolver::MetadataValue::String(value);
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<Vec<MetadataValue>>() {
            let inner = base_openpathresolver::MetadataValue::Array(
                value.iter().map(|v| v.inner.clone()).collect(),
            );
            Ok(Self { inner })
        } else if let Ok(value) =
            value.extract::<std::collections::HashMap<String, MetadataValue>>()
        {
            let mut attributes = std::collections::HashMap::new();

            for (k, v) in value.iter() {
                attributes.insert(k.to_string(), v.inner.clone());
            }
            let inner = base_openpathresolver::MetadataValue::Object(attributes);
            Ok(Self { inner })
        } else {
            let name = value.get_type().name()?;
            Err(PyTypeError::new_err(format!(
                "Type '{}' is not supported.",
                name
            )))
        }
    }
}

impl<'py> IntoPyObject<'py> for MetadataValue {
    type Target = PyAny;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        metadata_value_to_py_any(py, self.inner)
    }
}

impl From<base_openpathresolver::MetadataValue> for MetadataValue {
    fn from(value: base_openpathresolver::MetadataValue) -> Self {
        Self { inner: value }
    }
}

impl From<MetadataValue> for base_openpathresolver::MetadataValue {
    fn from(value: MetadataValue) -> Self {
        value.inner
    }
}

pub(crate) fn path_value_to_py_any<'py>(
    py: Python<'py>,
    value: base_openpathresolver::PathValue,
) -> PyResult<Bound<'py, PyAny>> {
    match value {
        base_openpathresolver::PathValue::Integer(value) => value.into_bound_py_any(py),
        base_openpathresolver::PathValue::String(value) => value.into_bound_py_any(py),
    }
}

pub(crate) fn template_value_to_py_any<'py>(
    py: Python<'py>,
    value: base_openpathresolver::TemplateValue,
) -> PyResult<Bound<'py, PyAny>> {
    match value {
        base_openpathresolver::TemplateValue::None => Ok(py.None().bind(py).clone()),
        base_openpathresolver::TemplateValue::Bool(value) => value.into_bound_py_any(py),
        base_openpathresolver::TemplateValue::Integer(value) => value.into_bound_py_any(py),
        base_openpathresolver::TemplateValue::Float(value) => value.into_bound_py_any(py),
        base_openpathresolver::TemplateValue::String(value) => value.into_bound_py_any(py),
        base_openpathresolver::TemplateValue::Array(value) => {
            let mut new_value = Vec::with_capacity(value.len());
            for v in value {
                new_value.push(template_value_to_py_any(py, v)?);
            }
            new_value.into_bound_py_any(py)
        }
        base_openpathresolver::TemplateValue::Object(value) => {
            let mut new_value = std::collections::HashMap::with_capacity(value.len());
            for (k, v) in value {
                new_value.insert(
                    crate::FieldKey { inner: k },
                    template_value_to_py_any(py, v)?,
                );
            }
            new_value.into_bound_py_any(py)
        }
    }
}

pub(crate) fn metadata_value_to_py_any<'py>(
    py: Python<'py>,
    value: base_openpathresolver::MetadataValue,
) -> PyResult<Bound<'py, PyAny>> {
    match value {
        base_openpathresolver::MetadataValue::None => Ok(py.None().bind(py).clone()),
        base_openpathresolver::MetadataValue::Bool(value) => value.into_bound_py_any(py),
        base_openpathresolver::MetadataValue::Integer(value) => value.into_bound_py_any(py),
        base_openpathresolver::MetadataValue::Float(value) => value.into_bound_py_any(py),
        base_openpathresolver::MetadataValue::String(value) => value.into_bound_py_any(py),
        base_openpathresolver::MetadataValue::Array(value) => {
            let mut new_value = Vec::with_capacity(value.len());
            for v in value {
                new_value.push(metadata_value_to_py_any(py, v)?);
            }
            new_value.into_bound_py_any(py)
        }
        base_openpathresolver::MetadataValue::Object(value) => {
            let mut new_value = std::collections::HashMap::with_capacity(value.len());
            for (k, v) in value {
                new_value.insert(k, metadata_value_to_py_any(py, v)?);
            }
            new_value.into_bound_py_any(py)
        }
    }
}
