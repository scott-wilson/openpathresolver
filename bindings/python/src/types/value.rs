use pyo3::{exceptions::PyTypeError, prelude::*, IntoPyObjectExt};

#[derive(Debug, Clone)]
#[pyclass]
pub struct PathValue {
    pub(crate) inner: base_openpathresolver::PathValue,
}

impl<'py> TryFrom<Bound<'py, PyAny>> for PathValue {
    type Error = PyErr;

    fn try_from(value: Bound<'py, PyAny>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<PathValue> for base_openpathresolver::PathValue {
    type Error = base_openpathresolver::Error;

    fn try_from(value: PathValue) -> Result<Self, Self::Error> {
        Ok(value.inner)
    }
}

impl TryFrom<base_openpathresolver::PathValue> for PathValue {
    type Error = PyErr;

    fn try_from(value: base_openpathresolver::PathValue) -> Result<Self, Self::Error> {
        Ok(Self { inner: value })
    }
}

#[pymethods]
impl PathValue {
    #[new]
    fn new<'py>(value: Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(value) = value.extract::<Self>() {
            Ok(value)
        } else if let Ok(value) = value.extract::<u16>() {
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

    fn value<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
        path_value_to_py_object(py, &self.inner)
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct TemplateValue {
    pub(crate) inner: base_openpathresolver::TemplateValue,
}

impl<'py> TryFrom<Bound<'py, PyAny>> for TemplateValue {
    type Error = PyErr;

    fn try_from(value: Bound<'py, PyAny>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<base_openpathresolver::TemplateValue> for TemplateValue {
    type Error = PyErr;

    fn try_from(value: base_openpathresolver::TemplateValue) -> Result<Self, Self::Error> {
        Ok(Self { inner: value })
    }
}

impl TryFrom<TemplateValue> for base_openpathresolver::TemplateValue {
    type Error = base_openpathresolver::Error;

    fn try_from(value: TemplateValue) -> Result<Self, Self::Error> {
        Ok(value.inner)
    }
}

#[pymethods]
impl TemplateValue {
    #[new]
    fn new<'py>(value: Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(value) = value.extract::<Self>() {
            Ok(value)
        } else if value.is_none() {
            let inner = base_openpathresolver::TemplateValue::None;
            Ok(Self { inner })
        } else if let Ok(value) = value.extract::<bool>() {
            let inner = base_openpathresolver::TemplateValue::Bool(value);
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
                "Type '{}' is not supported. Expected an integer or a string.",
                name
            )))
        }
    }

    fn value<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
        template_value_to_py_object(py, &self.inner)
    }
}

pub(crate) fn path_value_to_py_object<'py>(
    py: Python<'py>,
    value: &base_openpathresolver::PathValue,
) -> PyResult<Py<PyAny>> {
    match value {
        base_openpathresolver::PathValue::Integer(value) => value.into_py_any(py),
        base_openpathresolver::PathValue::String(value) => value.into_py_any(py),
    }
}

pub(crate) fn template_value_to_py_object<'py>(
    py: Python<'py>,
    value: &base_openpathresolver::TemplateValue,
) -> PyResult<Py<PyAny>> {
    match value {
        base_openpathresolver::TemplateValue::None => Ok(py.None()),
        base_openpathresolver::TemplateValue::Bool(value) => value.into_py_any(py),
        base_openpathresolver::TemplateValue::Integer(value) => value.into_py_any(py),
        base_openpathresolver::TemplateValue::Float(value) => value.into_py_any(py),
        base_openpathresolver::TemplateValue::String(value) => value.into_py_any(py),
        base_openpathresolver::TemplateValue::Array(value) => {
            let mut new_value = Vec::new();
            for v in value {
                new_value.push(template_value_to_py_object(py, v)?);
            }
            new_value.into_py_any(py)
        }
        base_openpathresolver::TemplateValue::Object(value) => {
            let mut new_value = std::collections::HashMap::new();
            for (k, v) in value {
                new_value.insert(k.as_str(), template_value_to_py_object(py, v)?);
            }
            new_value.into_py_any(py)
        }
        base_openpathresolver::TemplateValue::Entity(value) => crate::TemplateEntity {
            inner: std::sync::Arc::new(value.clone()),
        }
        .into_py_any(py),
    }
}
