
import dataclasses
import typing


@dataclasses.dataclass
class StatSlot:

    used: int
    total: int | None

    def __init__(self, *, data: list | tuple) -> None:
        self.used, self.total = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.used, self.total] == other
