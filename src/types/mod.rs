mod config;
mod field_key;
mod path_item;
mod resolver;
mod token;
mod value;

pub(crate) type PathAttributes = std::collections::HashMap<FieldKey, PathValue>;
pub(crate) type TemplateAttributes = std::collections::HashMap<FieldKey, TemplateValue>;
pub(crate) type Resolvers = std::collections::HashMap<FieldKey, Resolver>;

pub use config::{Config, ConfigBuilder};
pub use field_key::FieldKey;
pub(crate) use path_item::PathItem;
pub use path_item::{Owner, PathItemArgs, PathType, Permission, ResolvedPathItem};
pub use resolver::Resolver;
pub(crate) use token::{Token, Tokens};
pub use value::{MetadataValue, PathValue, TemplateValue};
