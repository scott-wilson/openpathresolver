import collections.abc
import enum
import os
import pathlib
import typing

import typing_extensions

class Error(Exception): ...

Resolvers: typing_extensions.TypeAlias = (
    IntegerResolver | StringResolver | EntityResolver
)
PathValue: typing_extensions.TypeAlias = int | str
TemplateValue: typing_extensions.TypeAlias = (
    None | bool | int | float | str | list[TemplateValue] | dict[str, TemplateValue]
)
MetadataValue: typing_extensions.TypeAlias = (
    None | bool | int | float | str | list[MetadataValue] | dict[str, MetadataValue]
)

class Config:
    def __init__(
        self,
        resolvers: collections.abc.Mapping[str, Resolvers],
        path_items: collections.abc.Iterable[PathItem],
    ) -> None: ...

class FieldKey:
    def __init__(self, key: str) -> None: ...

class EntityResolver:
    def __init__(self, key: str) -> None: ...

class IntegerResolver:
    def __init__(self, padding: int) -> None: ...

class StringResolver:
    def __init__(self, pattern: str | None) -> None: ...

class Owner(enum.Enum):
    Inherit = enum.auto()
    Root = enum.auto()
    Project = enum.auto()
    User = enum.auto()

class PathItem:
    def __init__(
        self,
        key: str,
        path: os.PathLike | str,
        parent: str | None,
        permission: Permission,
        owner: Owner,
        path_type: PathType,
        deferred: bool,
        metadata: collections.abc.Mapping[str, MetadataValue],
    ) -> None: ...

class ResolvedPathItem:
    def key(self) -> str | None: ...
    def value(self) -> pathlib.Path: ...
    def permission(self) -> Permission: ...
    def owner(self) -> Owner: ...
    def path_type(self) -> PathType: ...
    def deferred(self) -> bool: ...
    def metadata(self) -> MetadataValue: ...

class PathType(enum.Enum):
    Directory = enum.auto()
    File = enum.auto()
    FileTemplate = enum.auto()

class Permission(enum.Enum):
    Inherit = enum.auto()
    ReadOnly = enum.auto()
    ReadWrite = enum.auto()

async def create_workspace(
    config: Config,
    path_fields: collections.abc.Mapping[str, PathValue],
    template_fields: collections.abc.Mapping[str, TemplateValue],
    io_function: typing.Callable[
        [Config, collections.abc.Mapping[str, TemplateValue], ResolvedPathItem],
        collections.abc.Coroutine[typing.Any, typing.Any, None],
    ],
) -> None: ...
def get_workspace(
    config: Config, path_fields: collections.abc.Mapping[str, PathValue]
) -> list[ResolvedPathItem]: ...
def get_path(
    config: Config, key: str, fields: collections.abc.Mapping[str, PathValue]
) -> pathlib.Path: ...
def get_fields(
    config: Config, key: str, path: os.PathLike | str
) -> dict[str, PathValue]: ...
def get_key(
    config: Config,
    path: os.PathLike | str,
    fields: collections.abc.Mapping[str, PathValue],
) -> str | None: ...
def find_paths(
    config: Config, key: str, fields: collections.abc.Mapping[str, PathValue]
) -> list[pathlib.Path]: ...
