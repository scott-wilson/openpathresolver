use pyo3::prelude::*;

use crate::errors::to_py_error;

type PathAttributes = std::collections::HashMap<String, crate::PathValue>;

/// Resolve a path from a key and fields.
///
/// This will get a path to find in the filesystem or save to based on the input key and fields.
///
/// Args:
///     config: The config to get the path from.
///     key: The path item's key to generate the path from.
///     fields: The fields used to fill the placeholders in the path.
#[pyfunction]
pub fn get_path(
    config: &crate::Config,
    key: &str,
    fields: PathAttributes,
) -> PyResult<std::path::PathBuf> {
    base_openpathresolver::get_path(&config.inner, key, &convert_fields_from_wrapper(fields)?)
        .map_err(|err| to_py_error(&err))
}

/// Try to extract the fields from a key and path.
///
/// Args:
///     config: The config to get the fields from.
///     key: The path item's key to get the fields from.
///     path: The path to pull the values from.
#[pyfunction]
pub fn get_fields(
    config: &crate::Config,
    key: &str,
    path: std::path::PathBuf,
) -> PyResult<Option<std::collections::HashMap<String, crate::PathValue>>> {
    let result = base_openpathresolver::get_fields(&config.inner, key, path)
        .map_err(|err| to_py_error(&err))?;

    match result {
        Some(fields) => Ok(Some(
            convert_fields_from_base(fields)?
                .into_iter()
                .map(|(k, v)| (k.inner.as_str().to_owned(), v))
                .collect(),
        )),
        None => Ok(None),
    }
}

/// Find a key from a path and fields.
///
/// Args:
///     config: The config to get the key from.
///     path: The path to use to find the key for.
///     fields: The fields used to fill the placeholders in the path.
#[pyfunction]
pub fn get_key(
    config: &crate::Config,
    path: std::path::PathBuf,
    fields: PathAttributes,
) -> PyResult<Option<String>> {
    let result =
        base_openpathresolver::get_key(&config.inner, &path, &convert_fields_from_wrapper(fields)?)
            .map_err(|err| to_py_error(&err))?;

    match result {
        Some(key) => Ok(Some(key.as_str().to_owned())),
        None => Ok(None),
    }
}

/// Find paths from a given key and fields.
///
/// This differs from the :code:`get_path` because it will search the filesystem for the paths and the
/// fields do not need to be a superset of the path variables. If a path variable is missing, then
/// that will signify to the system to return all paths that match that variable's shape. For
/// example, if a user needs to find all of the versions of the "Widget" publish, and the structure
/// of the path looks like `"{root}/publishes/{entity}/{version}"`, then the only required fields
/// will be `root` and `entity`.
///
/// Args:
///     config: The config to find the paths from.
///     key: The path item's key used to find the paths.
///     fields: The fields used to fill the placeholders. If a field is not included, then that
///         represents finding all of the paths of that placeholder type.
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
        let value = base_openpathresolver::PathValue::from(value);

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
        let value = crate::PathValue::from(value);

        converted_fields.insert(key, value);
    }

    Ok(converted_fields)
}
