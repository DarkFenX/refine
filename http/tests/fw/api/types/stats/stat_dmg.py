import dataclasses
import typing

from fw import ANY_VALUE


@dataclasses.dataclass
class StatDmg:

    em: float
    thermal: float
    kinetic: float
    explosive: float
    breacher: StatDmgBreacher | float | None

    def __init__(self, *, data: list | tuple) -> None:
        self.em, self.thermal, self.kinetic, self.explosive, breacher = data
        self.breacher = StatDmgBreacher(data=breacher) if isinstance(breacher, list | tuple) else breacher

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        # Assume breacher can be anything if no 5th element is specified
        if isinstance(other, list) and len(other) == 4:
            other = [*other, ANY_VALUE]
        return [self.em, self.thermal, self.kinetic, self.explosive, self.breacher] == other


@dataclasses.dataclass
class StatDmgBreacher:

    abs_max: float
    rel_max: float

    def __init__(self, *, data: list | tuple) -> None:
        self.abs_max, self.rel_max = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.abs_max, self.rel_max] == other
