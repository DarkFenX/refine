from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class StatEhp:

    shield: StatLayerEhp | None
    armor: StatLayerEhp | None
    hull: StatLayerEhp | None

    def __init__(self, *, data: list | tuple) -> None:
        shield, armor, hull = data
        self.shield = StatLayerEhp(data=shield) if shield is not None else None
        self.armor = StatLayerEhp(data=armor) if armor is not None else None
        self.hull = StatLayerEhp(data=hull) if hull is not None else None

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
