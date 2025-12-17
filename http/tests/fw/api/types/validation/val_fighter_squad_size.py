import dataclasses
import typing
from collections import UserDict


class ValFighterSquadSizeFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValFighterSquadSizeInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValFighterSquadSizeInfo:

    size: int
    max_size: int

    def __init__(self, *, data: list | tuple) -> None:
        self.size, self.max_size = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.size, self.max_size] == other
