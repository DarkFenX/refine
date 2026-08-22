from fw.util import AttrDict, AttrHookDef, NttList
from .res_fit import FitStats
from .res_item import ItemStats
from .stat_dmg import StatDmg
from .stat_mining import StatMining
from .stat_outgoing_rps import StatOutRps


class FleetBatchStats(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'fleet': AttrHookDef(func=lambda d: FleetStats(data=d)),
            'fits': AttrHookDef(func=lambda d: {k: FitStats(data=v) for k, v in d.items()}),
            'items': AttrHookDef(func=lambda d: {k: ItemStats(data=v) for k, v in d.items()})})


class FleetStats(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'dmg': AttrHookDef(func=lambda d: (
                NttList(StatDmg(data=e) if e is not None else None for e in d)
                if d is not None else None)),
            'mps': AttrHookDef(func=lambda d: NttList(StatMining(data=e) for e in d)),
            'outgoing_nps': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'outgoing_rps': AttrHookDef(func=lambda d: (
                NttList(StatOutRps(data=e) if e is not None else None for e in d)
                if d is not None else None)),
            'outgoing_cps': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'mass': AttrHookDef(func=lambda d: (NttList(d)if d is not None else None))})
