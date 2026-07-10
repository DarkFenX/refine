import dataclasses
import typing


@dataclasses.dataclass(kw_only=True)
class Coordinates:

    x: float
    y: float
    z: float

    def __init__(self, *, data: list | tuple) -> None:
        self.x, self.y, self.z = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.x, self.y, self.z] == other
