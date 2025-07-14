from tests.fw.util import AttrDict, AttrHookDef, NttList
from .stat_ehp import StatEhp
from .stat_erps import StatErps
from .stat_hp import StatHp
from .stat_remote_rps import StatRemoteRps
from .stat_resists import StatResists
from .stat_resource import StatResource
from .stat_rps import StatRps
from .stat_slot import StatSlot


class FitStats(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'high_slots': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'mid_slots': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'low_slots': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'turret_slots': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'launcher_slots': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'rig_slots': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'service_slots': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'subsystem_slots': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'launched_drones': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'launched_fighters': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'launched_light_fighters': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'launched_heavy_fighters': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'launched_support_fighters': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'launched_st_light_fighters': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'launched_st_heavy_fighters': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'launched_st_support_fighters': AttrHookDef(func=lambda d: StatSlot(data=d)),
            'cpu': AttrHookDef(func=lambda d: StatResource(data=d)),
            'powergrid': AttrHookDef(func=lambda d: StatResource(data=d)),
            'calibration': AttrHookDef(func=lambda d: StatResource(data=d)),
            'drone_bay_volume': AttrHookDef(func=lambda d: StatResource(data=d)),
            'drone_bandwidth': AttrHookDef(func=lambda d: StatResource(data=d)),
            'fighter_bay_volume': AttrHookDef(func=lambda d: StatResource(data=d)),
            'hp': AttrHookDef(func=lambda d: StatHp(data=d) if d is not None else None),
            'ehp': AttrHookDef(func=lambda d: (
                NttList(StatEhp(data=e) if e is not None else None for e in d)
                if d is not None else None)),
            'wc_ehp': AttrHookDef(func=lambda d: StatEhp(data=d) if d is not None else None),
            'rps': AttrHookDef(func=lambda d: NttList(StatRps(data=e) for e in d) if d is not None else None),
            'erps': AttrHookDef(func=lambda d: (
                NttList(StatErps(data=e) if e is not None else None for e in d)
                if d is not None else None)),
            'resists': AttrHookDef(func=lambda d: StatResists(data=d) if d is not None else None),
            'remote_rps': AttrHookDef(func=lambda d: (
                NttList(StatRemoteRps(data=e) for e in d)
                if d is not None else None))})
