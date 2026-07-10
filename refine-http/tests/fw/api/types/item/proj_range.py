import dataclasses
import typing


@dataclasses.dataclass(kw_only=True)
class ProjRangeInfo:

    c2c: float
    s2s: float

    def __init__(self, *, data: list | tuple) -> None:
        self.c2c, self.s2s = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.c2c, self.s2s] == other
