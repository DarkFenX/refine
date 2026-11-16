import dataclasses
import typing
from collections import UserDict

from fw.consts import ApiValItemStatus


class ValChargeGroupFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValChargeGroupInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValChargeGroupInfo:

    status: ApiValItemStatus
    parent_item_id: str
    charge_group_id: int
    allowed_group_ids: list[int]

    def __init__(self, *, data: list | tuple) -> None:
        status, self.parent_item_id, self.charge_group_id, allowed_group_ids = data
        self.status = ApiValItemStatus(status)
        self.allowed_group_ids = sorted(allowed_group_ids)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        if isinstance(other, list) and len(other) >= 4:
            other[3] = sorted(other[3])
        return [self.status, self.parent_item_id, self.charge_group_id, self.allowed_group_ids] == other
