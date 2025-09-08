use pyo3::prelude::*;

mod errors;
mod path_resolver;
mod types;
mod workspace_resolver;

pub(crate) use errors::to_py_result;
pub use errors::{
    FieldError, FormatError, IOError, InfiniteRecursionError, IntegerConvertTypeError, JoinError,
    MismatchedFieldError, MissingItemError, MissingParentError, ParseError, ParseIntegerError,
    RegexError, ResolverTypeMismatchError, TemplateError, VariableRootPathError,
};
pub use path_resolver::{find_paths, get_fields, get_key, get_path};
pub(crate) use types::{path_value_to_py_object, template_value_to_py_object};
pub use types::{
    Config, CopyFile, EntityResolver, FieldKey, IntegerResolver, Owner, PathEntity, PathItem,
    PathValue, Permission, ResolvedPathItem, StringResolver, TemplateEntity, TemplateValue,
};
pub use workspace_resolver::{create_workspace, get_workspace};

#[pymodule]
pub mod openpathresolver {
    // Errors
    #[pymodule_export]
    use super::{
        FieldError, FormatError, IOError, InfiniteRecursionError, IntegerConvertTypeError,
        MismatchedFieldError, MissingItemError, MissingParentError, ParseError, ParseIntegerError,
        RegexError, ResolverTypeMismatchError, TemplateError, VariableRootPathError,
    };

    // Types
    #[pymodule_export]
    use super::{
        Config, CopyFile, EntityResolver, FieldKey, IntegerResolver, Owner, PathEntity, PathItem,
        PathValue, Permission, ResolvedPathItem, StringResolver, TemplateEntity, TemplateValue,
    };

    // Functions
    #[pymodule_export]
    use super::{create_workspace, find_paths, get_fields, get_key, get_path, get_workspace};
}
