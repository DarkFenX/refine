from dataclasses import dataclass


class ValFighterCountFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValFighterCountInfo(data=v) for k, v in data.items()})


@dataclass
class ValFighterCountInfo:

    count: int
    max_count: int

    def __init__(self, *, data: tuple) -> None:
        self.count = data[0]
        self.max_count = data[1]

    def __eq__(self, other: tuple) -> bool:
        return (self.count, self.max_count) == (other[0], other[1])
