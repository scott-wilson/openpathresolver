from __future__ import annotations

import pathlib
import argparse
import subprocess
from typing import TYPE_CHECKING

if TYPE_CHECKING:  # pragma: no cover
    pass

_CARGO_PATH = (
    pathlib.Path(__file__).parent.parent / "bindings" / "python" / "Cargo.toml"
)


def main(git_commit_id: str) -> None:
    with _CARGO_PATH.open(mode="r") as f_in:
        data = f_in.read()

    data = data.replace(
        'base_openpathresolver = { path = "../../", package = "openpathresolver" }',
        f'base_openpathresolver = {{ git = "https://github.com/scott-wilson/openpathresolver.git", package = "openpathresolver", rev="{git_commit_id}" }}',
        1,
    )

    with _CARGO_PATH.open(mode="w") as f_out:
        f_out.write(data)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("git_commit_id")
    args = parser.parse_args()

    main(args.git_commit_id)
