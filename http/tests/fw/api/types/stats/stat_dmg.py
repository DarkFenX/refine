import dataclasses
import typing


@dataclasses.dataclass
class StatDmg:

    em: float
    thermal: float
    kinetic: float
    explosive: float
    breacher: StatDmgBreacher | None

    def __init__(self, *, data: list | tuple) -> None:
        self.em, self.thermal, self.kinetic, self.explosive, breacher = data
        self.breacher = StatDmgBreacher(data=breacher) if breacher is not None else None

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        # Assume breacher is None if no 5th element is specified
        if isinstance(other, list) and len(other) == 4:
            other = [*other, None]
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
