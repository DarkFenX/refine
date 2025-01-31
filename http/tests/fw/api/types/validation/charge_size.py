from dataclasses import dataclass


class ValChargeSizeDetails(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValChargeSizeInfo(data=v) for k, v in data.items()})


@dataclass
class ValChargeSizeInfo:

    parent_item_id: str
    charge_size: float | None
    allowed_size: float

    def __init__(self, *, data: tuple) -> None:
        self.parent_item_id = data[0]
        self.charge_size = data[1]
        self.allowed_size = data[2]

    def __eq__(self, other: tuple) -> bool:
        return (self.parent_item_id, self.charge_size, self.allowed_size) == (other[0], other[1], other[2])
