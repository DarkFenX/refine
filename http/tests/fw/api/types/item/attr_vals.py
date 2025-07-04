import dataclasses
import typing


@dataclasses.dataclass
class AttrVals:

    base: float
    dogma: float
    extra: float

    def __init__(self, *, data: tuple) -> None:
        self.base = data[0]
        self.dogma = data[1]
        self.extra = data[2]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.base, self.dogma, self.extra) == (other[0], other[1], other[2])
