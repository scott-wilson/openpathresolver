from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

import openpathresolver

if TYPE_CHECKING:
    import collections.abc


@pytest.mark.asyncio
async def test_create_workspace_success(
    tmp_path_factory: pytest.TempPathFactory,
) -> None:
    tmp_root = tmp_path_factory.mktemp("root")

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

    async def io_function(
        config: openpathresolver.Config,  # noqa: ARG001
        template_args: collections.abc.Mapping[str, openpathresolver.TemplateValue],  # noqa: ARG001
        resolved_path_item: openpathresolver.ResolvedPathItem,
    ) -> None:
        resolved_path_item.value().mkdir(exist_ok=True, parents=True)

    await openpathresolver.create_workspace(
        config,
        {"root": tmp_root.as_posix(), "int": 3, "str": "test", "other": "other_test"},
        {},
        io_function,
    )

    assert (tmp_root / "path" / "to" / "003" / "test_other_test").is_dir()


def test_get_workspace_success(
    tmp_path_factory: pytest.TempPathFactory,
) -> None:
    tmp_root = tmp_path_factory.mktemp("root")

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

    result = openpathresolver.get_workspace(
        config,
        {"root": tmp_root.as_posix()},
    )
    assert sorted(
        [
            tmp_root,
            tmp_root / "path",
            tmp_root / "path" / "to",
        ]
    ) == sorted([i.value() for i in result])

    result = openpathresolver.get_workspace(
        config,
        {"root": tmp_root.as_posix(), "int": 3},
    )
    assert sorted(
        [
            tmp_root,
            tmp_root / "path",
            tmp_root / "path" / "to",
            tmp_root / "path" / "to" / "003",
        ]
    ) == sorted([i.value() for i in result])

    result = openpathresolver.get_workspace(
        config,
        {"root": tmp_root.as_posix(), "int": 3, "str": "test", "other": "other_test"},
    )
    assert sorted(
        [
            tmp_root,
            tmp_root / "path",
            tmp_root / "path" / "to",
            tmp_root / "path" / "to" / "003",
            tmp_root / "path" / "to" / "003" / "test_other_test",
        ]
    ) == sorted([i.value() for i in result])
