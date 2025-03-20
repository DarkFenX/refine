from dataclasses import dataclass


class ValFighterSquadSizeFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValFighterSquadSizeInfo(data=v) for k, v in data.items()})


@dataclass
class ValFighterSquadSizeInfo:

    size: int
    max_size: int

    def __init__(self, *, data: tuple) -> None:
        self.size = data[0]
        self.max_size = data[1]

    def __eq__(self, other: tuple) -> bool:
        return (self.size, self.max_size) == (other[0], other[1])
