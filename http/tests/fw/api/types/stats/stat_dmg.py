from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class StatDmg:

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
