from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class StatResists:

    shield: StatLayerResists
    armor: StatLayerResists
    hull: StatLayerResists

    def __init__(self, *, data: tuple) -> None:
        self.shield = StatLayerResists(data=data[0])
        self.armor = StatLayerResists(data=data[1])
        self.hull = StatLayerResists(data=data[2])

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.shield, self.armor, self.hull) == (other[0], other[1], other[2])


@dataclasses.dataclass
class StatLayerResists:

    em: float
    thermal: float
    kinetic: float
    explosive: float

    def __init__(self, *, data: tuple) -> None:
        self.em = data[0]
        self.thermal = data[1]
        self.kinetic = data[2]
        self.explosive = data[3]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.em, self.thermal, self.kinetic, self.explosive) == (other[0], other[1], other[2], other[3])
