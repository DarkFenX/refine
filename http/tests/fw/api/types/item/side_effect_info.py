from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class SideEffectInfo:

    chance: float
    status: bool
    str: tuple[SideEffectStrInfo] | list[SideEffectStrInfo] | None

    def __init__(self, *, data: tuple) -> None:
        self.chance = data[0]
        self.status = data[1]
        self.str = None if data[2] is None else SideEffectStrInfo(data=data[2])

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.chance, self.status, self.str) == (other[0], other[1], other[2])


@dataclasses.dataclass
class SideEffectStrInfo:

    op: str
    val: float

    def __init__(self, *, data: tuple) -> None:
        self.op = data[0]
        self.val = data[1]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.op, self.val) == (other[0], other[1])
