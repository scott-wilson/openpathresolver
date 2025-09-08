use pyo3::prelude::*;

use crate::errors::to_py_error;

type PathAttributes = std::collections::HashMap<String, crate::PathValue>;

#[pyfunction]
pub fn get_path(
    config: &crate::Config,
    key: &str,
    fields: PathAttributes,
) -> PyResult<std::path::PathBuf> {
    base_openpathresolver::get_path(&config.inner, key, &convert_fields_from_wrapper(fields)?)
        .map_err(|err| to_py_error(&err))
}

#[pyfunction]
pub fn get_fields(
    config: &crate::Config,
    key: &str,
    path: std::path::PathBuf,
) -> PyResult<Option<std::collections::HashMap<crate::FieldKey, crate::PathValue>>> {
    let result = base_openpathresolver::get_fields(&config.inner, key, path)
        .map_err(|err| to_py_error(&err))?;

    match result {
        Some(fields) => Ok(Some(convert_fields_from_base(fields)?)),
        None => Ok(None),
    }
}

#[pyfunction]
pub fn get_key(
    config: &crate::Config,
    path: std::path::PathBuf,
    fields: PathAttributes,
) -> PyResult<Option<crate::FieldKey>> {
    let result =
        base_openpathresolver::get_key(&config.inner, &path, &convert_fields_from_wrapper(fields)?)
            .map_err(|err| to_py_error(&err))?;

    match result {
        Some(key) => Ok(Some(crate::FieldKey::try_from(key.clone())?)),
        None => Ok(None),
    }
}

#[pyfunction]
pub fn find_paths(
    config: &crate::Config,
    key: &str,
    fields: PathAttributes,
) -> PyResult<Vec<std::path::PathBuf>> {
    base_openpathresolver::find_paths(&config.inner, key, &convert_fields_from_wrapper(fields)?)
        .map_err(|err| to_py_error(&err))
}

pub(crate) fn convert_fields_from_wrapper(
    fields: PathAttributes,
) -> PyResult<
    std::collections::HashMap<base_openpathresolver::FieldKey, base_openpathresolver::PathValue>,
> {
    let mut converted_fields = std::collections::HashMap::with_capacity(fields.len());

    for (key, value) in fields {
        let key =
            base_openpathresolver::FieldKey::try_from(key).map_err(|err| to_py_error(&err))?;
        let value =
            base_openpathresolver::PathValue::try_from(value).map_err(|err| to_py_error(&err))?;

        converted_fields.insert(key, value);
    }

    Ok(converted_fields)
}

pub(crate) fn convert_fields_from_base(
    fields: std::collections::HashMap<
        base_openpathresolver::FieldKey,
        base_openpathresolver::PathValue,
    >,
) -> PyResult<std::collections::HashMap<crate::FieldKey, crate::PathValue>> {
    let mut converted_fields = std::collections::HashMap::with_capacity(fields.len());

    for (key, value) in fields {
        let key = crate::FieldKey::try_from(key)?;
        let value = crate::PathValue::try_from(value)?;

        converted_fields.insert(key, value);
    }

    Ok(converted_fields)
}
