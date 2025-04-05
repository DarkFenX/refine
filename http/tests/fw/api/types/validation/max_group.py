from dataclasses import dataclass


class ValMaxGroupFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({int(k): ValMaxGroupGroup(data=v) for k, v in data.items()})


@dataclass
class ValMaxGroupGroup:

    group_item_count: int
    items: dict[str, int]

    def __init__(self, *, data: tuple) -> None:
        self.group_item_count = data[0]
        self.items = dict(data[1])

    def __eq__(self, other: tuple) -> bool:
        return (self.group_item_count, self.items) == (other[0], other[1])
