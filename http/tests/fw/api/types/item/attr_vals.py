import dataclasses
import typing


@dataclasses.dataclass
class AttrVals:

    base: float
    dogma: float
    extra: float

    def __init__(self, *, data: list | tuple) -> None:
        self.base, self.dogma, self.extra = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.base, self.dogma, self.extra] == other
