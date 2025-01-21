from tests.support.consts import ApiValType
from tests.support.util import AttrDict, AttrHookDef


class ValResult(AttrDict):

    def __init__(self, *, data: dict):
        super().__init__(data=data, hooks={'details': AttrHookDef(func=lambda d: ValResultDetails(data=d))})


class ValResultDetails(AttrDict):

    def __init__(self, *, data: dict):
        super().__init__(data=data, hooks={
            ApiValType.implant_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            ApiValType.booster_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            ApiValType.subsystem_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d))})


class ValSlotIndexDetails(dict):

    def __init__(self, *, data: dict):
        super().__init__({int(k): sorted(v) for k, v in data.items()})
