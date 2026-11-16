import dataclasses
import typing
from collections import UserDict

from fw.consts import ApiValItemStatus


class ValChargeSizeFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValChargeSizeInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValChargeSizeInfo:

    status: ApiValItemStatus
    parent_item_id: str
    charge_size: float | None
    allowed_size: float

    def __init__(self, *, data: list | tuple) -> None:
        status, self.parent_item_id, self.charge_size, self.allowed_size = data
        self.status = ApiValItemStatus(status)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.parent_item_id, self.charge_size, self.allowed_size] == other
