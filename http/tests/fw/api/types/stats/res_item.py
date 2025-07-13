from tests.fw.util import AttrDict, AttrHookDef
from .stat_ehp import StatEhp
from .stat_hp import StatHp
from .stat_reps import StatReps
from .stat_resists import StatResists


class ItemStats(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'hp': AttrHookDef(func=lambda d: StatHp(data=d) if d is not None else None),
            'ehp': AttrHookDef(
                func=lambda d: [StatEhp(data=e) if e is not None else None for e in d] if d is not None else None),
            'wc_ehp': AttrHookDef(func=lambda d: StatEhp(data=d) if d is not None else None),
            'reps': AttrHookDef(func=lambda d: [StatReps(data=e) for e in d] if d is not None else None),
            'resists': AttrHookDef(func=lambda d: StatResists(data=d) if d is not None else None)})
