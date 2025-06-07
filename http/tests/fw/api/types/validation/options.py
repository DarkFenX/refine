import dataclasses

from tests.fw.util import Absent

type ValOption = bool | tuple[bool, list[int]] | type[Absent]


@dataclasses.dataclass(kw_only=True)
class ValOptions:

    default: bool | type[Absent] = False
    cpu: ValOption = Absent
    powergrid: ValOption = Absent
    calibration: ValOption = Absent
    drone_bay_volume: ValOption = Absent
    drone_bandwidth: ValOption = Absent
    fighter_bay_volume: ValOption = Absent
    rig_slot_count: ValOption = Absent
    service_slot_count: ValOption = Absent
    subsystem_slot_count: ValOption = Absent
    launched_drone_count: ValOption = Absent
    launched_fighter_count: ValOption = Absent
    launched_support_fighter_count: ValOption = Absent
    launched_light_fighter_count: ValOption = Absent
    launched_heavy_fighter_count: ValOption = Absent
    launched_st_support_fighter_count: ValOption = Absent
    launched_st_light_fighter_count: ValOption = Absent
    launched_st_heavy_fighter_count: ValOption = Absent
    turret_slot_count: ValOption = Absent
    launcher_slot_count: ValOption = Absent
    high_slot_count: ValOption = Absent
    mid_slot_count: ValOption = Absent
    low_slot_count: ValOption = Absent
    implant_slot_index: ValOption = Absent
    booster_slot_index: ValOption = Absent
    subsystem_slot_index: ValOption = Absent
    ship_limit: ValOption = Absent
    max_group_fitted: ValOption = Absent
    max_group_online: ValOption = Absent
    max_group_active: ValOption = Absent
    rig_size: ValOption = Absent
    skill_reqs: ValOption = Absent
    charge_group: ValOption = Absent
    charge_size: ValOption = Absent
    charge_volume: ValOption = Absent
    capital_module: ValOption = Absent
    not_loaded_item: ValOption = Absent
    module_state: ValOption = Absent
    item_kind: ValOption = Absent
    drone_group: ValOption = Absent
    fighter_squad_size: ValOption = Absent
    unlaunchable_drone_slot: ValOption = Absent
    unlaunchable_drone_bandwidth: ValOption = Absent
    unlaunchable_fighter: ValOption = Absent
    unlaunchable_support_fighter: ValOption = Absent
    unlaunchable_light_fighter: ValOption = Absent
    unlaunchable_heavy_fighter: ValOption = Absent
    unlaunchable_st_support_fighter: ValOption = Absent
    unlaunchable_st_light_fighter: ValOption = Absent
    unlaunchable_st_heavy_fighter: ValOption = Absent
    ship_stance: ValOption = Absent
    overload_skill: ValOption = Absent
    max_type_fitted: ValOption = Absent
    sec_zone_fitted: ValOption = Absent
    sec_zone_online: ValOption = Absent
    sec_zone_active: ValOption = Absent
    sec_zone_unonlineable: ValOption = Absent
    sec_zone_unactivable: ValOption = Absent
    activation_blocked: ValOption = Absent
    item_vs_ship_kind: ValOption = Absent
    effect_stopper: ValOption = Absent
    assist_immunity: ValOption = Absent
    offense_immunity: ValOption = Absent
    resist_immunity: ValOption = Absent

    def to_dict(self) -> dict:
        return dataclasses.asdict(self, dict_factory=lambda d: {k: v for k, v in d if v is not Absent})
