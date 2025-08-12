from tests.fw.util import AttrDict, AttrHookDef, NttList
from .stat_dmg import StatDmg
from .stat_remote_rps import StatRemoteRps


class FleetStats(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'dps': AttrHookDef(func=lambda d: (NttList(StatDmg(data=e) for e in d))),
            'volley': AttrHookDef(func=lambda d: (NttList(StatDmg(data=e) for e in d))),
            'remote_rps': AttrHookDef(func=lambda d: (
                NttList(StatRemoteRps(data=e) for e in d)
                if d is not None else None))})
