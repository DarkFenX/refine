from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from typing import Union


@dataclass
class ValShipLimitDetails:

    ship_type_id: Union[int, None]
    ship_group_id: Union[int, None]
    mismatches: dict[str, ValShipLimitAllowed]

    def __init__(self, *, data):
        self.ship_type_id = data[0]
        self.ship_group_id = data[1]
        self.mismatches = {k: ValShipLimitAllowed(data=v) for k, v in data[2].items()}


@dataclass
class ValShipLimitAllowed:

    allowed_type_ids: list[int]
    allowed_group_ids: list[int]

    def __init__(self, *, data):
        self.allowed_type_ids = sorted(data[0])
        self.allowed_group_ids = sorted(data[1])

    def __eq__(self, other):
        return (self.allowed_type_ids, self.allowed_group_ids) == (sorted(other[0]), sorted(other[1]))
