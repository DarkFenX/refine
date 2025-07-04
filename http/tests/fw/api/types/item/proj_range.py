import dataclasses
import typing


@dataclasses.dataclass(kw_only=True)
class ProjRangeInfo:

    c2c: float
    s2s: float

    def __init__(self, *, data: tuple) -> None:
        self.c2c = data[0]
        self.s2s = data[1]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.c2c, self.s2s) == (other[0], other[1])
