from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

from tests.support.consts import ApiValType
from tests.support.util import AttrDict, AttrHookDef

if TYPE_CHECKING:
    from typing import Union


class ValResult(AttrDict):

    def __init__(self, *, data: dict):
        super().__init__(data=data, hooks={'details': AttrHookDef(func=lambda d: ValResultDetails(data=d))})


class ValResultDetails(AttrDict):

    def __init__(self, *, data: dict):
        super().__init__(data=data, hooks={
            ApiValType.implant_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            ApiValType.booster_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            ApiValType.subsystem_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            ApiValType.ship_limit: AttrHookDef(func=lambda d: ValShipLimitDetails(
                ship_type_id=d[0],
                ship_group_id=d[1],
                mismatches={
                    k: ValShipLimitAllowed(allowed_type_ids=sorted(v[0]), allowed_group_ids=sorted(v[1]))
                    for k, v in d[2].items()}))})


class ValSlotIndexDetails(dict):

    def __init__(self, *, data: dict):
        super().__init__({int(k): sorted(v) for k, v in data.items()})


@dataclass(kw_only=True)
class ValShipLimitDetails(dict):

    ship_type_id: Union[int, None]
    ship_group_id: Union[int, None]
    mismatches: dict[str, ValShipLimitAllowed]


@dataclass(kw_only=True)
class ValShipLimitAllowed:

    allowed_type_ids: list[int]
    allowed_group_ids: list[int]
