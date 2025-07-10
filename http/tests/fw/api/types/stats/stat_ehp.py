from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class StatEhp:

    shield: StatLayerEhp
    armor: StatLayerEhp
    hull: StatLayerEhp

    def __init__(self, *, data: tuple) -> None:
        self.shield = StatLayerEhp(data=data[0])
        self.armor = StatLayerEhp(data=data[1])
        self.hull = StatLayerEhp(data=data[2])

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.shield, self.armor, self.hull) == (other[0], other[1], other[2])


@dataclasses.dataclass
class StatLayerEhp:

    buffer: float
    ancil_local: float
    ancil_remote: float
    mult: float

    def __init__(self, *, data: tuple) -> None:
        self.buffer = data[0]
        self.ancil_local = data[1]
        self.ancil_remote = data[2]
        self.mult = data[3]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.buffer, self.ancil_local, self.ancil_remote, self.mult) == (other[0], other[1], other[2], other[3])
