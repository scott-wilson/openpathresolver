"""This is an example of creating a workspace.

The create workspace function can be called at any point in the lifecycle of a project
and should not destroy paths that already exist.
"""

# ruff: noqa: D103,S101,INP001

from __future__ import annotations

import asyncio
import pathlib
import tempfile
from typing import TYPE_CHECKING

import openpathresolver

if TYPE_CHECKING:
    import collections.abc


async def main() -> None:
    tmp_root = pathlib.Path(tempfile.mkdtemp())

    # First, the config will need to be initialized. openpathresolver intentionally does
    # not include support for a config file such as yaml, json, etc because we assume
    # that the calling code has its own config format that we can use.
    config = openpathresolver.Config(
        {
            # This marks values such as "001", "012", "123", and "1234" as valid. But
            # "01" and "1" would not be valid.
            "int": openpathresolver.IntegerResolver(3),
            "str": openpathresolver.StringResolver(r"\w+"),
        },
        [
            openpathresolver.PathItem(
                "path",
                # This is the path element. Anything between { and } will be looked up
                # in the fields and use the placeholder resolvers to decide how to
                # serialize the value.
                "{root}/path/to/{int}/{str}_{other}",
                # This is the root most item.
                None,
                # The permission model of the openpathresolver is very simple. Paths can
                # be read only, read/write, or inherit. If it is inherit, then the path
                # will automatically pick the permission of its parent. If there are no
                # permissions, then the IO function will have to decide what inherit
                # means. Otherwise the IO function will decide what read only or
                # read/write means given the context that the calling code should know.
                openpathresolver.Permission.Inherit,
                # The owner model is like the permission model, except the possible
                # values are root, project, user, and inherit. These values are also
                # meaningless to openpathresolver, and is up to the calling code to
                # decide what these might mean.
                openpathresolver.Owner.Inherit,
                # The path type is either a directory, file, or file template. It is up
                # to the calling code and the IO function to create a directory or file.
                # If the type is file template, then the IO function and calling code
                # can use any templating engine it prefers to create a file with a given
                # template.
                openpathresolver.PathType.Directory,
                # If a path is deferred, then it will not be generated unless a child
                # path is not deferred and can be resolved.
                deferred=False,
                # Extra metadata that might be useful for the IO function such as the
                # path to copy the file from.
                metadata={},
            )
        ],
    )

    # A simple implementation of the IO function.
    async def io_function(
        config: openpathresolver.Config,  # noqa: ARG001
        template_args: collections.abc.Mapping[str, openpathresolver.TemplateValue],  # noqa: ARG001
        resolved_path_item: openpathresolver.ResolvedPathItem,
    ) -> None:
        # In this case, we are expecting all the paths to be directories, and are
        # ignoring permissions, ownership, etc. Just create the directories.
        resolved_path_item.value().mkdir(exist_ok=True, parents=True)

    await openpathresolver.create_workspace(
        config,
        {"root": tmp_root.as_posix(), "int": 3, "str": "test", "other": "other_test"},
        {},
        io_function,
    )

    assert (tmp_root / "path" / "to" / "003" / "test_other_test").is_dir()


if __name__ == "__main__":
    asyncio.run(main())
