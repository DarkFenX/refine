from dataclasses import dataclass


@dataclass(kw_only=True)
class DpsProfile:

    em: float
    thermal: float
    kinetic: float
    explosive: float
    breacher: BreacherProfile | None

    def __init__(self, *, data: list | tuple) -> None:
        if len(data) == 4:
            self.em, self.thermal, self.kinetic, self.explosive = data
        else:
            self.em, self.thermal, self.kinetic, self.explosive, breacher = data
            self.breacher = BreacherProfile(data=breacher) if breacher is not None else None

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        self_value = [self.em, self.thermal, self.kinetic, self.explosive]
        if len(other) >= 5:
            self_value.append(self.breacher)
        return self_value == other


@dataclass(kw_only=True)
class BreacherProfile:

    abs_max: float
    rel_max: float

    def __init__(self, *, data: list | tuple) -> None:
        self.abs_max, self.rel_max = data

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.abs_max, self.rel_max] == other
