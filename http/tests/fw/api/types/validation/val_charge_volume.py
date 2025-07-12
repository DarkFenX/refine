
import dataclasses
import typing


class ValChargeVolumeFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValChargeVolumeInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValChargeVolumeInfo:

    parent_item_id: str
    charge_volume: float
    max_volume: float

    def __init__(self, *, data: list | tuple) -> None:
        self.parent_item_id, self.charge_volume, self.max_volume = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.parent_item_id, self.charge_volume, self.max_volume] == other
