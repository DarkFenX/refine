from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class StatReps:

    shield: StatLayerReps
    armor: StatLayerReps
    hull: StatLayerReps

    def __init__(self, *, data: list | tuple) -> None:
        shield, armor, hull = data
        self.shield = StatLayerReps(data=shield)
        self.armor = StatLayerReps(data=armor)
        self.hull = StatLayerReps(data=hull)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.shield, self.armor, self.hull] == other


@dataclasses.dataclass
class StatLayerReps:

    local: float
    remote: float
    remote_penalized: float

    def __init__(self, *, data: list | tuple) -> None:
        self.local, self.remote, self.remote_penalized = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.local, self.remote, self.remote_penalized] == other
