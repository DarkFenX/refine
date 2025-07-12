from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class StatResists:

    shield: StatLayerResists
    armor: StatLayerResists
    hull: StatLayerResists

    def __init__(self, *, data: list | tuple) -> None:
        shield, armor, hull = data
        self.shield = StatLayerResists(data=shield)
        self.armor = StatLayerResists(data=armor)
        self.hull = StatLayerResists(data=hull)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.shield, self.armor, self.hull] == other


@dataclasses.dataclass
class StatLayerResists:

    em: float
    thermal: float
    kinetic: float
    explosive: float

    def __init__(self, *, data: list | tuple) -> None:
        self.em, self.thermal, self.kinetic, self.explosive = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.em, self.thermal, self.kinetic, self.explosive] == other
