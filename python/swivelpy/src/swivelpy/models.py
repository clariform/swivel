from __future__ import annotations

from dataclasses import dataclass, field
from enum import StrEnum
from typing import Any, Literal


class SwivelSource(StrEnum):
    NOTION = "notion"


class SwivelEntryKind(StrEnum):
    PAGE = "page"
    DATA_SOURCE = "data_source"
    DATABASE = "database"
    WORKSPACE = "workspace"


class SwivelOutputKind(StrEnum):
    RAW = "raw"
    DOC = "doc"
    DOCS = "docs"
    CHUNKS = "chunks"


@dataclass(frozen=True)
class SwivelEntryPoint:
    source: SwivelSource
    kind: SwivelEntryKind
    id: str


@dataclass(frozen=True)
class SwivelConfig:
    """
    Configuration for invoking the Rust swivel CLI.

    Typical development mode:

        SwivelConfig(
            use_cargo=True,
            project_root=Path("/path/to/swivel"),
        )

    Typical installed mode:

        SwivelConfig(binary="swivel")
    """

    binary: str = "swivel"
    use_cargo: bool = False
    project_root: Any | None = None
    extra_env: dict[str, str] = field(default_factory=dict)


JsonDict = dict[str, Any]
JsonList = list[Any]
JsonValue = JsonDict | JsonList | str | int | float | bool | None

RetrieveMode = Literal["raw", "doc", "docs", "chunks"]
