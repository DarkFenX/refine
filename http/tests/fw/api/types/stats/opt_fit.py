import dataclasses

from tests.fw.util import Absent
from .opt_shared import (
    StatOptionAlias,
    StatOptionEhpAlias,
    StatOptionErpsAlias,
    StatOptionFitDpsAlias,
    StatOptionFitRemoteRpsAlias,
    StatOptionFitVolleyAlias,
    StatOptionRpsAlias,
    dc_to_dict,
)


@dataclasses.dataclass(kw_only=True)
class FitStatsOptions:

    default: bool = False
    high_slots: StatOptionAlias = Absent
    mid_slots: StatOptionAlias = Absent
    low_slots: StatOptionAlias = Absent
    turret_slots: StatOptionAlias = Absent
    launcher_slots: StatOptionAlias = Absent
    rig_slots: StatOptionAlias = Absent
    service_slots: StatOptionAlias = Absent
    subsystem_slots: StatOptionAlias = Absent
    launched_drones: StatOptionAlias = Absent
    launched_fighters: StatOptionAlias = Absent
    launched_light_fighters: StatOptionAlias = Absent
    launched_heavy_fighters: StatOptionAlias = Absent
    launched_support_fighters: StatOptionAlias = Absent
    launched_st_light_fighters: StatOptionAlias = Absent
    launched_st_heavy_fighters: StatOptionAlias = Absent
    launched_st_support_fighters: StatOptionAlias = Absent
    cpu: StatOptionAlias = Absent
    powergrid: StatOptionAlias = Absent
    calibration: StatOptionAlias = Absent
    drone_bay_volume: StatOptionAlias = Absent
    drone_bandwidth: StatOptionAlias = Absent
    fighter_bay_volume: StatOptionAlias = Absent
    speed: StatOptionAlias = Absent
    agility: StatOptionAlias = Absent
    align_time: StatOptionAlias = Absent
    sig_radius: StatOptionAlias = Absent
    mass: StatOptionAlias = Absent
    dps: StatOptionFitDpsAlias = Absent
    volley: StatOptionFitVolleyAlias = Absent
    hp: StatOptionAlias = Absent
    ehp: StatOptionEhpAlias = Absent
    wc_ehp: StatOptionAlias = Absent
    rps: StatOptionRpsAlias = Absent
    erps: StatOptionErpsAlias = Absent
    resists: StatOptionAlias = Absent
    remote_rps: StatOptionFitRemoteRpsAlias = Absent
    remote_cps: StatOptionAlias = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
