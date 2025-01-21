from tests.support.consts import ApiValType
from tests.support.util import AttrDict, AttrHookDef

from .ship_limit import ValShipLimitDetails
from .slot_index import ValSlotIndexDetails


class ValResult(AttrDict):

    def __init__(self, *, data: dict):
        super().__init__(data=data, hooks={'details': AttrHookDef(func=lambda d: ValResultDetails(data=d))})


class ValResultDetails(AttrDict):

    def __init__(self, *, data: dict):
        super().__init__(data=data, hooks={
            ApiValType.implant_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            ApiValType.booster_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            ApiValType.subsystem_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            ApiValType.ship_limit: AttrHookDef(func=lambda d: ValShipLimitDetails(data=d))})
