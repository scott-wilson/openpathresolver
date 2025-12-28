use pyo3::{exceptions::PyRuntimeError, prelude::*};

// TODO: Replace this with something to include extra useful information such as
// the field key that raised the error. There might be a nice way to do this in
// a future pyo3. See https://github.com/PyO3/pyo3/issues/295 for more info.

pyo3::create_exception!(path_resolver, FormatError, pyo3::exceptions::PyException);
pyo3::create_exception!(path_resolver, FieldError, pyo3::exceptions::PyException);
pyo3::create_exception!(
    path_resolver,
    ResolverTypeMismatchError,
    pyo3::exceptions::PyException
);
pyo3::create_exception!(
    path_resolver,
    MissingParentError,
    pyo3::exceptions::PyException
);
pyo3::create_exception!(path_resolver, ParseError, pyo3::exceptions::PyException);
pyo3::create_exception!(path_resolver, RegexError, pyo3::exceptions::PyException);
pyo3::create_exception!(
    path_resolver,
    IntegerConvertTypeError,
    pyo3::exceptions::PyException
);
pyo3::create_exception!(
    path_resolver,
    ParseIntegerError,
    pyo3::exceptions::PyException
);
pyo3::create_exception!(
    path_resolver,
    InfiniteRecursionError,
    pyo3::exceptions::PyException
);
pyo3::create_exception!(
    path_resolver,
    MissingItemError,
    pyo3::exceptions::PyException
);
pyo3::create_exception!(
    path_resolver,
    MismatchedFieldError,
    pyo3::exceptions::PyException
);
pyo3::create_exception!(
    path_resolver,
    VariableRootPathError,
    pyo3::exceptions::PyException
);
pyo3::create_exception!(path_resolver, IOError, pyo3::exceptions::PyException);
pyo3::create_exception!(path_resolver, JoinError, pyo3::exceptions::PyException);

pub(crate) fn to_py_error(err: &base_openpathresolver::Error) -> PyErr {
    match err {
        base_openpathresolver::Error::FormatError(_) => FormatError::new_err(err.to_string()),
        base_openpathresolver::Error::FieldError(_) => FieldError::new_err(err.to_string()),
        base_openpathresolver::Error::ResolverTypeMismatchError { .. } => {
            ResolverTypeMismatchError::new_err(err.to_string())
        }
        base_openpathresolver::Error::MissingParentError(_) => {
            MissingParentError::new_err(err.to_string())
        }
        base_openpathresolver::Error::ParseError(_) => ParseError::new_err(err.to_string()),
        base_openpathresolver::Error::RegexError(_) => RegexError::new_err(err.to_string()),
        base_openpathresolver::Error::IntegerConvertTypeError(_) => {
            IntegerConvertTypeError::new_err(err.to_string())
        }
        base_openpathresolver::Error::ParseIntegerError(_) => {
            ParseIntegerError::new_err(err.to_string())
        }
        base_openpathresolver::Error::InfiniteRecursionError { .. } => {
            InfiniteRecursionError::new_err(err.to_string())
        }
        base_openpathresolver::Error::MissingItemError(_) => {
            MissingItemError::new_err(err.to_string())
        }
        base_openpathresolver::Error::MismatchedFieldError { .. } => {
            MismatchedFieldError::new_err(err.to_string())
        }
        base_openpathresolver::Error::VariableRootPathError => {
            VariableRootPathError::new_err(err.to_string())
        }
        base_openpathresolver::Error::IOError(_) => IOError::new_err(err.to_string()),
        base_openpathresolver::Error::GlobError(glob_error) => {
            IOError::new_err(glob_error.to_string())
        }
        base_openpathresolver::Error::GlobPatternError(pattern_error) => {
            IOError::new_err(pattern_error.to_string())
        }
        base_openpathresolver::Error::RuntimeError(_) => PyRuntimeError::new_err(err.to_string()),
    }
}

pub(crate) fn to_py_result<T>(result: Result<T, base_openpathresolver::Error>) -> PyResult<T> {
    match result {
        Ok(value) => Ok(value),
        Err(err) => Err(to_py_error(&err)),
    }
}
