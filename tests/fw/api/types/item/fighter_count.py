import dataclasses
import typing


@dataclasses.dataclass
class FighterCountInfo:

    current: int
    max: int

    def __init__(self, *, data: list | tuple) -> None:
        self.current, self.max = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.current, self.max] == other
