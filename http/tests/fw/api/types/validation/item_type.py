from dataclasses import dataclass


class ValItemTypeDetails(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValItemTypeInfo(data=v) for k, v in data.items()})


@dataclass
class ValItemTypeInfo:

    item_type: str | None
    expected_type: str

    def __init__(self, *, data: tuple) -> None:
        self.item_type = data[0]
        self.expected_type = data[1]

    def __eq__(self, other: tuple) -> bool:
        return (self.item_type, self.expected_type) == (other[0], other[1])
