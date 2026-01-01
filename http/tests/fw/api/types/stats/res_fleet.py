from fw.util import AttrDict, AttrHookDef, NttList
from .stat_dmg import StatDmg
from .stat_mining import StatMining
from .stat_outgoing_rps import StatOutRps


class FleetStats(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'dps': AttrHookDef(func=lambda d: (
                NttList(StatDmg(data=e) if e is not None else None for e in d)
                if d is not None else None)),
            'volley': AttrHookDef(func=lambda d: (
                NttList(StatDmg(data=e) if e is not None else None for e in d)
                if d is not None else None)),
            'mps': AttrHookDef(func=lambda d: NttList(StatMining(data=e) for e in d)),
            'outgoing_nps': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'outgoing_rps': AttrHookDef(func=lambda d: (
                NttList(StatOutRps(data=e) for e in d)
                if d is not None else None)),
            'outgoing_cps': AttrHookDef(func=lambda d: NttList(d) if d is not None else None)})
