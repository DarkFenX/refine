from __future__ import annotations

from dataclasses import dataclass


@dataclass
class ValShipLimitFail:

    ship_type_id: int | None
    ship_group_id: int | None
    mismatches: dict[str, ValShipLimitAllowed]

    def __init__(self, *, data: tuple) -> None:
        self.ship_type_id = data[0]
        self.ship_group_id = data[1]
        self.mismatches = {k: ValShipLimitAllowed(data=v) for k, v in data[2].items()}


@dataclass
class ValShipLimitAllowed:

    allowed_type_ids: list[int]
    allowed_group_ids: list[int]

    def __init__(self, *, data: tuple) -> None:
        self.allowed_type_ids = sorted(data[0])
        self.allowed_group_ids = sorted(data[1])

    def __eq__(self, other: tuple) -> bool:
        return (self.allowed_type_ids, self.allowed_group_ids) == (sorted(other[0]), sorted(other[1]))
