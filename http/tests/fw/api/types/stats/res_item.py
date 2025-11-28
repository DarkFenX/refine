from tests.fw.util import AttrDict, AttrHookDef, NttList
from .stat_dmg import StatDmg
from .stat_ehp import StatEhp
from .stat_erps import StatErps
from .stat_hp import StatHp
from .stat_jam import StatIncomingJam
from .stat_mining import StatMining
from .stat_outgoing_rps import StatOutRps
from .stat_resists import StatResists
from .stat_rps import StatRps
from .stat_sensor import StatSensor


class ItemStats(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'sensor': AttrHookDef(func=lambda d: StatSensor(data=d) if d is not None else None),
            'incoming_jam': AttrHookDef(func=lambda d: StatIncomingJam(data=d) if d is not None else None),
            'dps': AttrHookDef(func=lambda d: (
                NttList(StatDmg(data=e) if e is not None else None for e in d)
                if d is not None else None)),
            'volley': AttrHookDef(func=lambda d: (
                NttList(StatDmg(data=e) if e is not None else None for e in d)
                if d is not None else None)),
            'mps': AttrHookDef(func=lambda d: NttList(StatMining(data=e) for e in d) if d is not None else None),
            'hp': AttrHookDef(func=lambda d: StatHp(data=d) if d is not None else None),
            'ehp': AttrHookDef(func=lambda d: (NttList(StatEhp(data=e) for e in d) if d is not None else None)),
            'wc_ehp': AttrHookDef(func=lambda d: StatEhp(data=d) if d is not None else None),
            'rps': AttrHookDef(func=lambda d: NttList(StatRps(data=e) for e in d) if d is not None else None),
            'erps': AttrHookDef(func=lambda d: (NttList(StatErps(data=e) for e in d) if d is not None else None)),
            'resists': AttrHookDef(func=lambda d: StatResists(data=d) if d is not None else None),
            'outgoing_rps': AttrHookDef(func=lambda d: (
                NttList(StatOutRps(data=e) for e in d)
                if d is not None else None)),
            'outgoing_cps': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'outgoing_nps': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'cap_balance': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'cap_sim': AttrHookDef(func=lambda d: NttList(d) if d is not None else None)})
