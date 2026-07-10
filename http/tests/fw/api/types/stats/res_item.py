from fw.util import AttrDict, AttrHookDef, NttList
from .stat_dmg import StatDmg
from .stat_ehp import StatEhp
from .stat_erps import StatErps
from .stat_hp import StatHp
from .stat_jam import StatIncomingJam
from .stat_mining import StatMining
from .stat_outgoing_rps import StatOutRps
from .stat_resists import StatResists
from .stat_rps import StatRps
from .stat_sensors import StatSensors


class ItemStats(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            # Output
            'dmg': AttrHookDef(func=lambda d: (
                NttList(StatDmg(data=e) if e is not None else None for e in d) if d is not None else None)),
            'mps': AttrHookDef(func=lambda d: NttList(StatMining(data=e) for e in d) if d is not None else None),
            'outgoing_nps': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'outgoing_rps': AttrHookDef(func=lambda d: (
                NttList(StatOutRps(data=e) if e is not None else None for e in d)
                if d is not None else None)),
            'outgoing_cps': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            # Tank
            'resists': AttrHookDef(func=lambda d: NttList(StatResists(data=e) for e in d) if d is not None else None),
            'hp': AttrHookDef(func=lambda d: NttList(StatHp(data=e) for e in d) if d is not None else None),
            'ehp': AttrHookDef(func=lambda d: NttList(StatEhp(data=e) for e in d) if d is not None else None),
            'wc_ehp': AttrHookDef(func=lambda d: NttList(StatEhp(data=e) for e in d) if d is not None else None),
            'rps': AttrHookDef(func=lambda d: NttList(StatRps(data=e) for e in d) if d is not None else None),
            'erps': AttrHookDef(func=lambda d: NttList(StatErps(data=e) for e in d) if d is not None else None),
            'breach_resist': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            # Cap
            'cap_amount': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'cap_balance': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'cap_sim': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'neut_resist': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            # Sensors
            'locks': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'lock_range': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'scan_res': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'sensors': AttrHookDef(func=lambda d: NttList(StatSensors(data=e) for e in d) if d is not None else None),
            'dscan_range': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'probing_size': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'incoming_jam': AttrHookDef(func=lambda d: (
                NttList(StatIncomingJam(data=e) for e in d) if d is not None else None)),
            # Mobility
            'speed': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'agility': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'align_time': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'sig_radius': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'mass': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'warp_speed': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'max_warp_range': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'jump': AttrHookDef(func=lambda d: NttList(AttrDict(data=e) for e in d) if d is not None else None),
            # Misc
            'drone_control_range': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'can_warp': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'can_jump_gate': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'can_jump_wormhole': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'can_jump_drive': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'can_dock_station': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'can_dock_citadel': AttrHookDef(func=lambda d: NttList(d) if d is not None else None),
            'can_tether': AttrHookDef(func=lambda d: NttList(d) if d is not None else None)})
