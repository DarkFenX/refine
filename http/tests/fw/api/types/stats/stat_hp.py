from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class StatHp:

    shield: StatLayerHp
    armor: StatLayerHp
    structure: StatLayerHp

    def __init__(self, *, data: tuple) -> None:
        self.shield = StatLayerHp(data=data[0])
        self.armor = StatLayerHp(data=data[1])
        self.structure = StatLayerHp(data=data[2])

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.shield, self.armor, self.structure) == (other[0], other[1], other[2])


@dataclasses.dataclass
class StatLayerHp:

    buffer: float
    ancil_local: float
    ancil_remote: float

    def __init__(self, *, data: tuple) -> None:
        self.buffer = data[0]
        self.ancil_local = data[1]
        self.ancil_remote = data[2]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.buffer, self.ancil_local, self.ancil_remote) == (other[0], other[1], other[2])
