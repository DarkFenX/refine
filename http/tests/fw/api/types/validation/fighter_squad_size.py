
import dataclasses
import typing


class ValFighterSquadSizeFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValFighterSquadSizeInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValFighterSquadSizeInfo:

    size: int
    max_size: int

    def __init__(self, *, data: tuple) -> None:
        self.size = data[0]
        self.max_size = data[1]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.size, self.max_size) == (other[0], other[1])
