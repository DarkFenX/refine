from tests.fw.util import AttrDict, AttrHookDef, NttList
from .stat_dmg import StatDmg
from .stat_ehp import StatEhp
from .stat_erps import StatErps
from .stat_hp import StatHp
from .stat_remote_rps import StatRemoteRps
from .stat_resists import StatResists
from .stat_rps import StatRps


class ItemStats(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'dps': AttrHookDef(func=lambda d: (NttList(StatDmg(data=e) for e in d) if d is not None else None)),
            'volley': AttrHookDef(func=lambda d: (NttList(StatDmg(data=e) for e in d) if d is not None else None)),
            'hp': AttrHookDef(func=lambda d: StatHp(data=d) if d is not None else None),
            'ehp': AttrHookDef(func=lambda d: (NttList(StatEhp(data=e) for e in d) if d is not None else None)),
            'wc_ehp': AttrHookDef(func=lambda d: StatEhp(data=d) if d is not None else None),
            'rps': AttrHookDef(func=lambda d: NttList(StatRps(data=e) for e in d) if d is not None else None),
            'erps': AttrHookDef(func=lambda d: (NttList(StatErps(data=e) for e in d) if d is not None else None)),
            'resists': AttrHookDef(func=lambda d: StatResists(data=d) if d is not None else None),
            'remote_rps': AttrHookDef(func=lambda d: (
                NttList(StatRemoteRps(data=e) for e in d)
                if d is not None else None)),
            'remote_cps': AttrHookDef(func=lambda d: NttList(d) if d is not None else None)})
