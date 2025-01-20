from tests.support.util import AttrDict, AttrHookDef


class ValResult(AttrDict):

    def __init__(self, *, data: dict):
        super().__init__(data=data, hooks={'details': AttrHookDef(func=lambda d: ValResultDetails(data=d))})


class ValResultDetails(AttrDict):

    def __init__(self, *, data: dict):
        super().__init__(data=data, hooks={
            'implant_slot_index': AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            'booster_slot_index': AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            'subsystem_slot_index': AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d))})


class ValSlotIndexDetails(dict):

    def __init__(self, *, data: dict):
        super().__init__({int(k): sorted(v) for k, v in data.items()})
