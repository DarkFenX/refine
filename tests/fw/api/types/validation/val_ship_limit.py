import dataclasses
import typing

from fw.consts import ApiValItemStatus


@dataclasses.dataclass
class ValShipLimitFail:

    ship_type_id: int | None
    ship_group_id: int | None
    items: dict[str, ValShipLimitItemInfo]

    def __init__(self, *, data: list | tuple) -> None:
        self.ship_type_id, self.ship_group_id, items = data
        self.items = {k: ValShipLimitItemInfo(data=v) for k, v in items.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.ship_type_id, self.ship_group_id, self.items] == other


@dataclasses.dataclass
class ValShipLimitItemInfo:

    status: ApiValItemStatus
    allowed_type_ids: list[int]
    allowed_group_ids: list[int]

    def __init__(self, *, data: list | tuple) -> None:
        status, allowed_type_ids, allowed_group_ids = data
        self.status = ApiValItemStatus(status)
        self.allowed_type_ids = sorted(allowed_type_ids)
        self.allowed_group_ids = sorted(allowed_group_ids)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        if isinstance(other, list):
            if len(other) >= 2:
                other[1] = sorted(other[1])
            if len(other) >= 3:
                other[2] = sorted(other[2])
        return [self.status, self.allowed_type_ids, self.allowed_group_ids] == other
