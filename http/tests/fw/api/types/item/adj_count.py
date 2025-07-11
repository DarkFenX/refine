import dataclasses
import typing


@dataclasses.dataclass
class AdjustableCount:

    current: int
    max: int
    overridden: bool

    def __init__(self, *, data: tuple) -> None:
        self.current = data[0]
        self.max = data[1]
        self.overridden = data[2]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.current, self.max, self.overridden) == (other[0], other[1], other[2])
