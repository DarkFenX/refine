import dataclasses

from tests.fw.api.aliases import DpsProfile
from tests.fw.util import Absent

type StatOptionAlias = bool | type[Absent]
type StatOptionAliasEhp = StatOptionAlias | tuple[bool, list[StatsOptionEhp]]
type StatOptionAliasRr = StatOptionAlias | tuple[bool, list[StatsOptionRr]]


@dataclasses.dataclass(kw_only=True)
class StatsOptions:

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
    agility: StatOptionAlias = Absent
    align_time: StatOptionAlias = Absent
    speed: StatOptionAlias = Absent
    hp: StatOptionAlias = Absent
    ehp: StatOptionAlias = Absent
    wc_ehp: StatOptionAlias = Absent
    resists: StatOptionAlias = Absent
    rr_shield: StatOptionAliasRr = Absent
    rr_armor: StatOptionAliasRr = Absent
    rr_hull: StatOptionAliasRr = Absent
    rr_capacitor: StatOptionAliasRr = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionEhp:

    incoming_dps: DpsProfile | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionRr:

    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


def dc_to_dict(data: dataclasses.dataclass) -> dict:
    return dataclasses.asdict(data, dict_factory=lambda d: {k: v for k, v in d if v is not Absent})
