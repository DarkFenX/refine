import dataclasses
import typing

from tests.fw.util import Absent, dc_to_dict

if typing.TYPE_CHECKING:
    from .opt_cap import StatsOptionCapBalance, StatsOptionCapSim
    from .opt_dmg import StatsOptionFitDps, StatsOptionFitVolley
    from .opt_ehp import StatsOptionEhp
    from .opt_mining import StatsOptionFitMining
    from .opt_outgoing_nps import StatsOptionFitOutNps
    from .opt_outgoing_rps import StatsOptionFitOutRps
    from .opt_rps import StatsOptionErps, StatsOptionRps


@dataclasses.dataclass(kw_only=True)
class FitStatsOptions:

    default: bool | type[Absent] = False
    # Fit output stats
    dps: bool | tuple[bool, list[StatsOptionFitDps]] | type[Absent] = Absent
    volley: bool | tuple[bool, list[StatsOptionFitVolley]] | type[Absent] = Absent
    mps: bool | tuple[bool, list[StatsOptionFitMining]] | type[Absent] = Absent
    outgoing_nps: bool | tuple[bool, list[StatsOptionFitOutNps]] | type[Absent] = Absent
    outgoing_rps: bool | tuple[bool, list[StatsOptionFitOutRps]] | type[Absent] = Absent
    outgoing_cps: bool | type[Absent] = Absent
    # Fit resources
    cpu: bool | type[Absent] = Absent
    powergrid: bool | type[Absent] = Absent
    calibration: bool | type[Absent] = Absent
    drone_bay_volume: bool | type[Absent] = Absent
    drone_bandwidth: bool | type[Absent] = Absent
    fighter_bay_volume: bool | type[Absent] = Absent
    # Fit slots
    high_slots: bool | type[Absent] = Absent
    mid_slots: bool | type[Absent] = Absent
    low_slots: bool | type[Absent] = Absent
    turret_slots: bool | type[Absent] = Absent
    launcher_slots: bool | type[Absent] = Absent
    rig_slots: bool | type[Absent] = Absent
    service_slots: bool | type[Absent] = Absent
    subsystem_slots: bool | type[Absent] = Absent
    launched_drones: bool | type[Absent] = Absent
    launched_fighters: bool | type[Absent] = Absent
    launched_light_fighters: bool | type[Absent] = Absent
    launched_heavy_fighters: bool | type[Absent] = Absent
    launched_support_fighters: bool | type[Absent] = Absent
    launched_st_light_fighters: bool | type[Absent] = Absent
    launched_st_heavy_fighters: bool | type[Absent] = Absent
    launched_st_support_fighters: bool | type[Absent] = Absent
    # Ship tank
    resists: bool | type[Absent] = Absent
    hp: bool | type[Absent] = Absent
    ehp: bool | tuple[bool, list[StatsOptionEhp]] | type[Absent] = Absent
    wc_ehp: bool | type[Absent] = Absent
    rps: bool | tuple[bool, list[StatsOptionRps]] | type[Absent] = Absent
    erps: bool | tuple[bool, list[StatsOptionErps]] | type[Absent] = Absent
    # Ship cap
    cap_amount: bool | type[Absent] = Absent
    cap_balance: bool | tuple[bool, list[StatsOptionCapBalance]] | type[Absent] = Absent
    cap_sim: bool | tuple[bool, list[StatsOptionCapSim]] | type[Absent] = Absent
    neut_resist: bool | type[Absent] = Absent
    # Ship sensors
    locks: bool | type[Absent] = Absent
    lock_range: bool | type[Absent] = Absent
    scan_res: bool | type[Absent] = Absent
    sensors: bool | type[Absent] = Absent
    dscan_range: bool | type[Absent] = Absent
    probing_size: bool | type[Absent] = Absent
    incoming_jam: bool | type[Absent] = Absent
    # Ship mobility
    speed: bool | type[Absent] = Absent
    agility: bool | type[Absent] = Absent
    align_time: bool | type[Absent] = Absent
    sig_radius: bool | type[Absent] = Absent
    mass: bool | type[Absent] = Absent
    warp_speed: bool | type[Absent] = Absent
    max_warp_range: bool | type[Absent] = Absent
    # Ship misc stats
    drone_control_range: bool | type[Absent] = Absent
    can_warp: bool | type[Absent] = Absent
    can_jump_gate: bool | type[Absent] = Absent
    can_jump_drive: bool | type[Absent] = Absent
    can_dock_station: bool | type[Absent] = Absent
    can_dock_citadel: bool | type[Absent] = Absent
    can_tether: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
