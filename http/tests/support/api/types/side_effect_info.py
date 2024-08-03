from __future__ import annotations

from typing import NamedTuple, TYPE_CHECKING

if TYPE_CHECKING:
    from typing import Union


class SideEffectInfo(NamedTuple):

    chance: float
    status: bool
    str: Union[tuple[SideEffectStrInfo], list[SideEffectStrInfo], None]


class SideEffectStrInfo(NamedTuple):

    op: str
    val: float
