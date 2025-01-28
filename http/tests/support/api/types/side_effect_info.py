from __future__ import annotations

from dataclasses import dataclass


@dataclass(kw_only=True)
class SideEffectInfo:

    chance: float
    status: bool
    str: tuple[SideEffectStrInfo] | list[SideEffectStrInfo] | None


@dataclass(kw_only=True)
class SideEffectStrInfo:

    op: str
    val: float
