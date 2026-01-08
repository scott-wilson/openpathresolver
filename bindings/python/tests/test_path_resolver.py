from __future__ import annotations

import pathlib
from typing import TYPE_CHECKING

import openpathresolver

if TYPE_CHECKING:
    import pytest


def test_get_path_success() -> None:
    config = openpathresolver.Config(
        {
            "int": openpathresolver.IntegerResolver(3),
            "str": openpathresolver.StringResolver(r"\w+"),
        },
        [
            openpathresolver.PathItem(
                "path",
                "path/to/{int}/{str}_{other}",
                None,
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


def test_get_fields_success() -> None:
    config = openpathresolver.Config(
        {
            "int": openpathresolver.IntegerResolver(3),
            "str": openpathresolver.StringResolver(r"\w+?"),
        },
        [
            openpathresolver.PathItem(
                "path",
                "path/to/{int}/{str}_{other}",
                None,
                openpathresolver.Permission.Inherit,
                openpathresolver.Owner.Inherit,
                openpathresolver.PathType.Directory,
                deferred=False,
                metadata={},
            )
        ],
    )

    fields = openpathresolver.get_fields(
        config, "path", pathlib.Path("path/to/004/test_other_test")
    )
    assert fields == {
        "int": 4,
        "str": "test",
        "other": "other_test",
    }


def test_get_key_success() -> None:
    config = openpathresolver.Config(
        {
            "int": openpathresolver.IntegerResolver(3),
            "str": openpathresolver.StringResolver(r"\w+"),
        },
        [
            openpathresolver.PathItem(
                "path",
                "path/to/{int}/{str}_{other}",
                None,
                openpathresolver.Permission.Inherit,
                openpathresolver.Owner.Inherit,
                openpathresolver.PathType.Directory,
                deferred=False,
                metadata={},
            )
        ],
    )

    key = openpathresolver.get_key(
        config,
        "path/to/003/test_other_test",
        {
            "int": 3,
            "str": "test",
            "other": "other_test",
        },
    )
    assert key == "path"


def test_find_paths_success(tmp_path_factory: pytest.TempPathFactory) -> None:
    tmp_root = tmp_path_factory.mktemp("root")
    expected_paths = []

    for index in range(3):
        test_dir = tmp_root / "path" / "to" / f"{index:03d}" / "test_other_test"
        test_dir.mkdir(parents=True, exist_ok=True)
        expected_paths.append(test_dir)

    config = openpathresolver.Config(
        {
            "int": openpathresolver.IntegerResolver(3),
            "str": openpathresolver.StringResolver(r"\w+"),
        },
        [
            openpathresolver.PathItem(
                "path",
                "{root}/path/to/{int}/{str}_{other}",
                None,
                openpathresolver.Permission.Inherit,
                openpathresolver.Owner.Inherit,
                openpathresolver.PathType.Directory,
                deferred=False,
                metadata={},
            )
        ],
    )

    paths = openpathresolver.find_paths(
        config,
        "path",
        {"root": tmp_root.as_posix(), "str": "test", "other": "other_test"},
    )

    assert sorted(paths) == sorted(expected_paths)
