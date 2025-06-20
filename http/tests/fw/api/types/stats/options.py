import dataclasses

from tests.fw.util import Absent

type StatOption = bool | type[Absent]


@dataclasses.dataclass(kw_only=True)
class StatsOptions:

    default: bool = False
    high_slots: StatOption  = Absent
    mid_slots: StatOption  = Absent
    low_slots: StatOption  = Absent
    turret_slots: StatOption  = Absent
    launcher_slots: StatOption  = Absent
    rig_slots: StatOption  = Absent
    service_slots: StatOption  = Absent
    subsystem_slots: StatOption  = Absent
    launched_drones: StatOption  = Absent
    launched_fighters: StatOption  = Absent
    launched_light_fighters: StatOption  = Absent
    launched_heavy_fighters: StatOption  = Absent
    launched_support_fighters: StatOption  = Absent
    launched_st_light_fighters: StatOption  = Absent
    launched_st_heavy_fighters: StatOption  = Absent
    launched_st_support_fighters: StatOption  = Absent
    cpu: StatOption  = Absent
    powergrid: StatOption  = Absent
    calibration: StatOption  = Absent
    drone_bay_volume: StatOption  = Absent
    drone_bandwidth: StatOption  = Absent
    fighter_bay_volume: StatOption  = Absent
    agility: StatOption  = Absent
    align_time: StatOption  = Absent
    speed: StatOption  = Absent
    hp: StatOption  = Absent

    def to_dict(self) -> dict:
        return dataclasses.asdict(self, dict_factory=lambda d: {k: v for k, v in d if v is not Absent})
