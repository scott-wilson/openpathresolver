from __future__ import annotations

import asyncio
from typing import TYPE_CHECKING

import hypothesis
import pytest
from hypothesis import strategies

import openpathresolver

if TYPE_CHECKING:
    import collections.abc
    import typing


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
                metadata={"test": 123},
            ),
        ],
    )

    async def io_function(
        config: openpathresolver.Config,  # noqa: ARG001
        template_args: collections.abc.Mapping[str, openpathresolver.TemplateValue],  # noqa: ARG001
        resolved_path_item: openpathresolver.ResolvedPathItem,
    ) -> None:
        resolved_path_item.value().mkdir(exist_ok=True, parents=True)

        if resolved_path_item.key() == "path":
            assert resolved_path_item.metadata()["test"] == 123  # noqa: PLR2004

    await openpathresolver.create_workspace(
        config,
        {"root": tmp_root.as_posix(), "int": 3, "str": "test", "other": "other_test"},
        {},
        io_function,
    )

    assert (tmp_root / "path" / "to" / "003" / "test_other_test").is_dir()


def test_create_workspace_asyncio_run_success(
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

    async def main() -> None:
        await openpathresolver.create_workspace(
            config,
            {
                "root": tmp_root.as_posix(),
                "int": 3,
                "str": "test",
                "other": "other_test",
            },
            {},
            io_function,
        )

    asyncio.run(main())

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
    expected = []
    assert expected == sorted([i.value() for i in result])

    result = openpathresolver.get_workspace(
        config,
        {"root": tmp_root.as_posix(), "int": 3},
    )
    expected = []
    assert expected == sorted([i.value() for i in result])

    result = openpathresolver.get_workspace(
        config,
        {"root": tmp_root.as_posix(), "int": 3, "str": "test", "other": "other_test"},
    )
    expected = sorted(
        [
            tmp_root,
            tmp_root / "path",
            tmp_root / "path" / "to",
            tmp_root / "path" / "to" / "003",
            tmp_root / "path" / "to" / "003" / "test_other_test",
        ]
    )
    assert expected == sorted([i.value() for i in result])


def test_create_workspace_regression_segfault(
    tmp_path_factory: pytest.TempPathFactory,
) -> None:
    tmp_root = tmp_path_factory.mktemp("root")

    path_items = [
        openpathresolver.PathItem(
            "root",
            "{root_dir}",
            None,
            openpathresolver.Permission.Inherit,
            openpathresolver.Owner.Inherit,
            openpathresolver.PathType.Directory,
            False,  # noqa: FBT003
            {},
        ),
        openpathresolver.PathItem(
            "art_root",
            "{project_name}-art",
            "root",
            openpathresolver.Permission.Inherit,
            openpathresolver.Owner.Inherit,
            openpathresolver.PathType.Directory,
            False,  # noqa: FBT003
            {},
        ),
        openpathresolver.PathItem(
            "game_root",
            "{project_name}-game",
            "root",
            openpathresolver.Permission.Inherit,
            openpathresolver.Owner.Inherit,
            openpathresolver.PathType.Directory,
            False,  # noqa: FBT003
            {},
        ),
        openpathresolver.PathItem(
            "art_asset_workspace",
            "art_assets/{asset_type}/{asset_name}",
            "art_root",
            openpathresolver.Permission.Inherit,
            openpathresolver.Owner.Inherit,
            openpathresolver.PathType.Directory,
            False,  # noqa: FBT003
            {},
        ),
        openpathresolver.PathItem(
            "art_asset_blend",
            "{asset_name}.blend",
            "art_asset_workspace",
            openpathresolver.Permission.Inherit,
            openpathresolver.Owner.Inherit,
            openpathresolver.PathType.File,
            False,  # noqa: FBT003
            {"skip": True},
        ),
        openpathresolver.PathItem(
            "game_asset_dir",
            "art_assets/{asset_type}/{asset_name}",
            "game_root",
            openpathresolver.Permission.Inherit,
            openpathresolver.Owner.Inherit,
            openpathresolver.PathType.Directory,
            False,  # noqa: FBT003
            {},
        ),
    ]

    config = openpathresolver.Config({}, path_items)

    async def io_function(
        config: openpathresolver.Config,
        template_args: collections.abc.Mapping[str, openpathresolver.TemplateValue],
        resolved_path_item: openpathresolver.ResolvedPathItem,
    ) -> None:
        pass

    async def main() -> None:
        await openpathresolver.create_workspace(
            config,
            {
                "root_dir": tmp_root.as_posix(),
                "project_name": "project_name",
                "asset_type": "asset_type",
                "asset_name": "asset_name",
            },
            {},
            io_function,
        )

    asyncio.run(main())


@hypothesis.given(
    metadata=strategies.dictionaries(
        strategies.text(),
        strategies.recursive(
            strategies.none()
            | strategies.booleans()
            | strategies.integers(min_value=-1000, max_value=1000)
            | strategies.floats(allow_nan=False, allow_infinity=False),
            extend=lambda children: (
                strategies.lists(children)
                | strategies.dictionaries(strategies.text(), children)
            ),
        ),
    )
)
@pytest.mark.asyncio
async def test_create_workspace_regression_wrong_metadata_types(
    metadata: dict[str, typing.Any],
) -> None:
    config = openpathresolver.Config(
        {
            "int": openpathresolver.IntegerResolver(3),
            "str": openpathresolver.StringResolver(r"\w+"),
        },
        [
            openpathresolver.PathItem(
                "path",
                "",
                None,
                openpathresolver.Permission.Inherit,
                openpathresolver.Owner.Inherit,
                openpathresolver.PathType.Directory,
                deferred=False,
                metadata=metadata,
            ),
        ],
    )

    def validate(a: typing.Any, b: typing.Any) -> None:  # noqa: ANN401
        assert type(a) is type(b)

        if a is None or isinstance(a, (bool, int, float, str)):
            assert a == b
        elif isinstance(a, list):
            assert len(a) == len(b)

            for item_a, item_b in zip(a, b):
                validate(item_a, item_b)
        elif isinstance(a, dict):
            assert len(a) == len(b)
            assert set(a) == set(b)  # Validate the keys

            for key, value_a in a.items():
                value_b = b[key]
                validate(value_a, value_b)
        else:
            raise NotImplementedError

    async def io_function(
        config: openpathresolver.Config,  # noqa: ARG001
        template_args: collections.abc.Mapping[str, openpathresolver.TemplateValue],  # noqa: ARG001
        resolved_path_item: openpathresolver.ResolvedPathItem,
    ) -> None:
        validate(metadata, resolved_path_item.metadata())

    await openpathresolver.create_workspace(config, {}, {}, io_function)
