from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class StatEhp:

    shield: StatLayerEhp
    armor: StatLayerEhp
    hull: StatLayerEhp

    def __init__(self, *, data: list | tuple) -> None:
        shield, armor, hull = data
        self.shield = StatLayerEhp(data=shield)
        self.armor = StatLayerEhp(data=armor)
        self.hull = StatLayerEhp(data=hull)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.shield, self.armor, self.hull] == other


@dataclasses.dataclass
class StatLayerEhp:

    buffer: float
    ancil_local: float
    ancil_remote: float
    mult: float

    def __init__(self, *, data: list | tuple) -> None:
        self.buffer, self.ancil_local, self.ancil_remote, self.mult = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.buffer, self.ancil_local, self.ancil_remote, self.mult] == other
