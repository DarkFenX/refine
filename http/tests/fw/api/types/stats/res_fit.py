from tests.fw.util import AttrDict, AttrHookDef, NttList
from .stat_dmg import StatDmg
from .stat_ehp import StatEhp
from .stat_erps import StatErps
from .stat_hp import StatHp
from .stat_jam import StatIncomingJam
from .stat_mining import StatMining
from .stat_outgoing_rps import StatOutRps
from .stat_resists import StatResists
from .stat_resource import StatResource
from .stat_rps import StatRps
from .stat_sensors import StatSensors
from .stat_slot import StatSlot


class FitStats(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            # Fit output stats
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
            # Fit resources
            'cpu': AttrHookDef(func=lambda d: StatResource(data=d)),
            'powergrid': AttrHookDef(func=lambda d: StatResource(data=d)),
            'calibration': AttrHookDef(func=lambda d: StatResource(data=d)),
            'drone_bay_volume': AttrHookDef(func=lambda d: StatResource(data=d)),
            'drone_bandwidth': AttrHookDef(func=lambda d: StatResource(data=d)),
            'fighter_bay_volume': AttrHookDef(func=lambda d: StatResource(data=d)),
            # Fit slots
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
            # Ship tank
            'resists': AttrHookDef(func=lambda d: StatResists(data=d) if d is not None else None),
            'hp': AttrHookDef(func=lambda d: StatHp(data=d) if d is not None else None),
            'ehp': AttrHookDef(func=lambda d: (NttList(StatEhp(data=e) for e in d) if d is not None else None)),
            'wc_ehp': AttrHookDef(func=lambda d: StatEhp(data=d) if d is not None else None),
            'rps': AttrHookDef(func=lambda d: NttList(StatRps(data=e) for e in d) if d is not None else None),
            'erps': AttrHookDef(func=lambda d: (NttList(StatErps(data=e) for e in d) if d is not None else None)),
            # Ship cap
            'cap_balance': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'cap_sim': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            # Ship sensors
            'sensors': AttrHookDef(func=lambda d: StatSensors(data=d) if d is not None else None),
            'incoming_jam': AttrHookDef(func=lambda d: StatIncomingJam(data=d) if d is not None else None)})
