import dataclasses
import typing


@dataclasses.dataclass
class StatSensor:

    kind: str
    strength: float

    def __init__(self, *, data: list | tuple) -> None:
        self.kind, self.strength = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.kind, self.strength] == other
