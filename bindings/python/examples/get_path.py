"""This is an example of getting a path.

The get_path function is extremely useful when trying to save a file to a location with
a specific naming structure, or getting a path to a file with that naming structure.
"""

# ruff: noqa: D103,S101,INP001

from __future__ import annotations

import pathlib

import openpathresolver


def main() -> None:
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
                "path/to/{int}/{str}_{other}",
                # This is the root most item.
                None,
                # The following fields can be ignored, since they are not useful for the
                # find_paths function.
                openpathresolver.Permission.Inherit,
                openpathresolver.Owner.Inherit,
                openpathresolver.PathType.Directory,
                deferred=False,
                metadata={},
            )
        ],
    )

    path = openpathresolver.get_path(
        config,
        "path",
        {
            "int": 3,
            "str": "test",
            "other": "other_test",
        },
    )
    assert path == pathlib.Path("path/to/003/test_other_test")


if __name__ == "__main__":
    main()
