"""This is an example of finding paths.

the fields to be specified, leaving a field out of the map tells find_paths to return
all paths that match the placeholder's shape. For example, getting all of the versions
of a specific asset would be as simple as filling the fields map with all of the fields
except the version.
"""

# ruff: noqa: D103,S101,INP001

from __future__ import annotations

import pathlib
import tempfile

import openpathresolver


def main() -> None:
    tmp_root = pathlib.Path(tempfile.mkdtemp())
    expected_paths = []

    for index in range(3):
        test_dir = tmp_root / "path" / "to" / f"{index:03d}" / "test_other_test"
        test_dir.mkdir(parents=True, exist_ok=True)
        expected_paths.append(test_dir)

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

    # The int field is intentionally left out to tell find_paths to find all of the
    # ints.
    paths = openpathresolver.find_paths(
        config,
        "path",
        {"root": tmp_root.as_posix(), "str": "test", "other": "other_test"},
    )

    assert sorted(paths) == sorted(expected_paths)


if __name__ == "__main__":
    main()
