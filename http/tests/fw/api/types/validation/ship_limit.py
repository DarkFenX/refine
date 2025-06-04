from __future__ import annotations

import dataclasses
import typing


@dataclasses.dataclass
class ValShipLimitFail:

    ship_type_id: int | None
    ship_group_id: int | None
    items: dict[str, ValShipLimitItemInfo]

    def __init__(self, *, data: tuple) -> None:
        self.ship_type_id = data[0]
        self.ship_group_id = data[1]
        self.items = {k: ValShipLimitItemInfo(data=v) for k, v in data[2].items()}


@dataclasses.dataclass
class ValShipLimitItemInfo:

    allowed_type_ids: list[int]
    allowed_group_ids: list[int]

    def __init__(self, *, data: tuple) -> None:
        self.allowed_type_ids = sorted(data[0])
        self.allowed_group_ids = sorted(data[1])

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.allowed_type_ids, self.allowed_group_ids) == (sorted(other[0]), sorted(other[1]))
