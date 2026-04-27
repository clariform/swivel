from __future__ import annotations

import json
import os
import subprocess
from pathlib import Path
from typing import Any

from .errors import (
    SwivelCommandError,
    SwivelConfigError,
    SwivelJsonError,
)
from .models import (
    JsonValue,
    SwivelConfig,
    SwivelEntryKind,
    SwivelEntryPoint,
    SwivelOutputKind,
    SwivelSource,
)


class SwivelClient:
    def __init__(self, config: SwivelConfig | None = None) -> None:
        self.config = config or SwivelConfig()

    def retrieve(
        self,
        entry: SwivelEntryPoint,
        output: SwivelOutputKind = SwivelOutputKind.CHUNKS,
    ) -> JsonValue:
        if entry.source != SwivelSource.NOTION:
            raise ValueError(f"Unsupported source: {entry.source}")

        if entry.kind == SwivelEntryKind.WORKSPACE:
            raise NotImplementedError(
                "workspace-level retrieval is not implemented in swivel yet"
            )

        args = self._notion_args(
            kind=entry.kind,
            entry_id=entry.id,
            output=output,
        )

        return self._run_json(args)

    def retrieve_chunks(self, entry: SwivelEntryPoint) -> list[dict[str, Any]]:
        value = self.retrieve(entry, SwivelOutputKind.CHUNKS)

        if not isinstance(value, list):
            raise TypeError(f"Expected chunks list, got {type(value).__name__}")

        return value

    def retrieve_docs(self, entry: SwivelEntryPoint) -> list[dict[str, Any]]:
        value = self.retrieve(entry, SwivelOutputKind.DOCS)

        if entry.kind == SwivelEntryKind.PAGE:
            if not isinstance(value, dict):
                raise TypeError(f"Expected page doc object, got {type(value).__name__}")
            return [value]

        if not isinstance(value, list):
            raise TypeError(f"Expected docs list, got {type(value).__name__}")

        return value

    def retrieve_raw(self, entry: SwivelEntryPoint) -> dict[str, Any]:
        value = self.retrieve(entry, SwivelOutputKind.RAW)

        if not isinstance(value, dict):
            raise TypeError(f"Expected raw object, got {type(value).__name__}")

        return value

    def _notion_args(
        self,
        kind: SwivelEntryKind,
        entry_id: str,
        output: SwivelOutputKind,
    ) -> list[str]:
        if output == SwivelOutputKind.RAW:
            command_by_kind = {
                SwivelEntryKind.PAGE: "get-page",
                SwivelEntryKind.DATA_SOURCE: "get-data-source",
                SwivelEntryKind.DATABASE: "get-database",
            }
        elif output == SwivelOutputKind.DOC:
            command_by_kind = {
                SwivelEntryKind.PAGE: "get-page-doc",
                SwivelEntryKind.DATABASE: "get-database-doc",
            }
        elif output == SwivelOutputKind.DOCS:
            command_by_kind = {
                SwivelEntryKind.PAGE: "get-page-doc",
                SwivelEntryKind.DATA_SOURCE: "get-data-source-docs",
                SwivelEntryKind.DATABASE: "get-database-docs",
            }
        elif output == SwivelOutputKind.CHUNKS:
            command_by_kind = {
                SwivelEntryKind.PAGE: "get-page-chunks",
                SwivelEntryKind.DATA_SOURCE: "get-data-source-chunks",
                SwivelEntryKind.DATABASE: "get-database-chunks",
            }
        else:
            raise ValueError(f"Unsupported output kind: {output}")

        command = command_by_kind.get(kind)

        if command is None:
            raise ValueError(f"Unsupported combination: kind={kind}, output={output}")

        return ["notion", command, entry_id]

    def _command(self, args: list[str]) -> list[str]:
        if self.config.use_cargo:
            return [
                "cargo",
                "run",
                "-p",
                "swivelcli",
                "--",
                *args,
            ]

        return [self.config.binary, *args]

    def _run_json(self, args: list[str]) -> JsonValue:
        env = os.environ.copy()
        env.update(self.config.extra_env)

        if "NOTION_API_KEY" not in env:
            raise SwivelConfigError("NOTION_API_KEY is not set")

        cmd = self._command(args)

        cwd: str | None = None
        if self.config.project_root is not None:
            cwd = str(Path(self.config.project_root).expanduser())

        result = subprocess.run(
            cmd,
            cwd=cwd,
            env=env,
            text=True,
            capture_output=True,
            check=False,
        )

        if result.returncode != 0:
            raise SwivelCommandError(
                "swivel command failed\n"
                f"command: {' '.join(cmd)}\n"
                f"exit code: {result.returncode}\n"
                f"stderr:\n{result.stderr.strip()}\n"
                f"stdout:\n{result.stdout.strip()}"
            )

        try:
            return json.loads(result.stdout)
        except json.JSONDecodeError as exc:
            raise SwivelJsonError(
                "swivel returned invalid JSON\n"
                f"command: {' '.join(cmd)}\n"
                f"stdout preview:\n{result.stdout[:1000]}"
            ) from exc
