from dataclasses import dataclass


class ValChargeVolumeFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValChargeVolumeInfo(data=v) for k, v in data.items()})


@dataclass
class ValChargeVolumeInfo:

    parent_item_id: str
    charge_volume: float
    max_volume: float

    def __init__(self, *, data: tuple) -> None:
        self.parent_item_id = data[0]
        self.charge_volume = data[1]
        self.max_volume = data[2]

    def __eq__(self, other: tuple) -> bool:
        return (self.parent_item_id, self.charge_volume, self.max_volume) == (other[0], other[1], other[2])
