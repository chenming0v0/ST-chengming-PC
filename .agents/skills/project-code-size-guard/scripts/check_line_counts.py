#!/usr/bin/env python3
"""Check source file line counts for the ST Launcher project."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys


SOFT_LIMIT = 300
HARD_LIMIT = 500
DEFAULT_ROOTS = ("src", "src-tauri/src")
SOURCE_EXTENSIONS = {
    ".rs",
    ".js",
    ".jsx",
    ".ts",
    ".tsx",
    ".css",
    ".scss",
    ".html",
    ".toml",
}


def count_lines(path: Path) -> int:
    with path.open("rb") as handle:
        return sum(1 for _ in handle)


def iter_source_files(project_root: Path, roots: tuple[str, ...]):
    for root_name in roots:
        root = project_root / root_name
        if not root.exists():
            continue
        for path in root.rglob("*"):
            if path.is_file() and path.suffix.lower() in SOURCE_EXTENSIONS:
                yield path


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Check line counts under src and src-tauri/src."
    )
    parser.add_argument(
        "--root",
        default=".",
        help="Project root. Defaults to the current working directory.",
    )
    parser.add_argument(
        "--soft",
        type=int,
        default=SOFT_LIMIT,
        help=f"Soft warning threshold. Defaults to {SOFT_LIMIT}.",
    )
    parser.add_argument(
        "--hard",
        type=int,
        default=HARD_LIMIT,
        help=f"Hard warning threshold. Defaults to {HARD_LIMIT}.",
    )
    args = parser.parse_args()

    project_root = Path(args.root).resolve()
    files = sorted(iter_source_files(project_root, DEFAULT_ROOTS))
    warnings: list[tuple[int, str, Path]] = []

    for path in files:
        lines = count_lines(path)
        if lines > args.hard:
            warnings.append((lines, "HARD", path))
        elif lines > args.soft:
            warnings.append((lines, "SOFT", path))

    print(f"Checked {len(files)} source files under: {', '.join(DEFAULT_ROOTS)}")

    if not warnings:
        print(f"OK: no files exceed {args.soft} lines.")
        return 0

    hard_count = 0
    for lines, level, path in sorted(warnings, reverse=True):
        rel = path.relative_to(project_root)
        if level == "HARD":
            hard_count += 1
            print(f"HARD {lines:>4} lines  {rel}  must split before completion")
        else:
            print(f"SOFT {lines:>4} lines  {rel}  consider splitting if practical")

    if hard_count:
        print(f"FAIL: {hard_count} file(s) exceed {args.hard} lines.")
        return 1

    print(f"WARN: {len(warnings)} file(s) exceed {args.soft} lines.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
