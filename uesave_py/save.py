from __future__ import annotations  # noqa

import uuid
from dataclasses import dataclass
from typing import TYPE_CHECKING, TypedDict, Union

if TYPE_CHECKING:

    class PackageVersionOld(TypedDict):
        Old: int

    class PackageVersionNew(TypedDict):
        New: tuple[int, int]


@dataclass(frozen=True, slots=True)
class Header:
    magic: int
    save_game_version: int
    package_version: Union["PackageVersionOld", "PackageVersionNew"]
    engine_version_major: int
    engine_version_minor: int
    engine_version_patch: int
    engine_version_build: int
    engine_version: str
    custom_format_version: int
    custom_format: list[tuple[str | uuid.UUID, int]]


@dataclass(frozen=True, slots=True)
class Root:
    save_game_type: str
    properties: dict


@dataclass(frozen=True, slots=True)
class Save:
    header: Header
    root: Root
    extra: bytearray
