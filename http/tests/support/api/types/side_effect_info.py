from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from typing import Union


@dataclass(kw_only=True)
class SideEffectInfo:

    chance: float
    status: bool
    str: Union[tuple[SideEffectStrInfo], list[SideEffectStrInfo], None]


@dataclass(kw_only=True)
class SideEffectStrInfo:

    op: str
    val: float
