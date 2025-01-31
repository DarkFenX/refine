from dataclasses import dataclass


class ValMaxGroupDetails(dict):

    def __init__(self, *, data: dict):
        super().__init__({int(k): ValMaxGroupGroup(data=v) for k, v in data.items()})


@dataclass
class ValMaxGroupGroup:

    count: int
    items: dict[str, int]

    def __init__(self, *, data):
        self.count = data[0]
        self.items = dict(data[1])

    def __eq__(self, other):
        return (self.count, self.items) == (other[0], other[1])
