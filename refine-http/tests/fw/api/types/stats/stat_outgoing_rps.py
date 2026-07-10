import dataclasses
import typing


@dataclasses.dataclass
class StatOutRps:

    shield: float
    armor: float
    hull: float

    def __init__(self, *, data: list | tuple) -> None:
        self.shield, self.armor, self.hull = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.shield, self.armor, self.hull] == other
