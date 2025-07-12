from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class StatHp:

    shield: StatLayerHp
    armor: StatLayerHp
    hull: StatLayerHp

    def __init__(self, *, data: list | tuple) -> None:
        shield, armor, hull = data
        self.shield = StatLayerHp(data=shield)
        self.armor = StatLayerHp(data=armor)
        self.hull = StatLayerHp(data=hull)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.shield, self.armor, self.hull] == other


@dataclasses.dataclass
class StatLayerHp:

    buffer: float
    ancil_local: float
    ancil_remote: float

    def __init__(self, *, data: list | tuple) -> None:
        self.buffer, self.ancil_local, self.ancil_remote = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.buffer, self.ancil_local, self.ancil_remote] == other
