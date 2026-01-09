//! Find paths based on a structured query or build out a tree in the filesystem.
//!
//! The open path resolver contains two parts. The path resolver is responsible for finding paths
//! or the elements used to find a path. This can be used either to get where to save a file or
//! where to try to find a file to load.
//!
//! The workspace resolver is responsible for building out a tree for a given query. For example,
//! if there is a location to save published elements and workspaces for building out the elements,
//! and a user needs to build out the "Widget" element, then the system can automatically build out
//! the file and folder structure needed for the user to do their work.

#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(unsafe_code)]

mod error;
mod path_resolver;
mod types;
mod workspace_resolver;

pub use error::Error;
pub use types::{
    Config, ConfigBuilder, FieldKey, MetadataValue, Owner, PathItemArgs, PathType, PathValue,
    Permission, ResolvedPathItem, Resolver, TemplateValue,
};

pub use path_resolver::{find_paths, get_fields, get_key, get_path};
pub use workspace_resolver::{CreateWorkspaceIoFunction, create_workspace, get_workspace};
