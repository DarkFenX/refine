import dataclasses
import typing


@dataclasses.dataclass
class StatIncomingJam:

    chance: float
    uptime: float

    def __init__(self, *, data: list | tuple) -> None:
        self.chance, self.uptime = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.chance, self.uptime] == other
