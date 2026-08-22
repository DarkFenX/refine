from fw.util import AttrDict, AttrHookDef
from .res_fit import FitStats
from .res_fleet import FleetStats
from .res_item import ItemStats


class SolBatchStats(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'fleets': AttrHookDef(func=lambda d: {k: FleetStats(data=v) for k, v in d.items()}),
            'fits': AttrHookDef(func=lambda d: {k: FitStats(data=v) for k, v in d.items()}),
            'items': AttrHookDef(func=lambda d: {k: ItemStats(data=v) for k, v in d.items()})})
