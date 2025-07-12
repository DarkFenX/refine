from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class SideEffectInfo:

    chance: float
    status: bool
    str: tuple[SideEffectStrInfo] | list[SideEffectStrInfo] | None

    def __init__(self, *, data: list | tuple) -> None:
        self.chance, self.status, side_str = data
        self.str = None if side_str is None else SideEffectStrInfo(data=side_str)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.chance, self.status, self.str] == other


@dataclasses.dataclass
class SideEffectStrInfo:

    op: str
    val: float

    def __init__(self, *, data: list | tuple) -> None:
        self.op, self.val = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.op, self.val] == other
