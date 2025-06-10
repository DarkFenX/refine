import dataclasses

from tests.fw.util import Absent

type ValOption = bool | tuple[bool, list[int]] | type[Absent]


@dataclasses.dataclass(kw_only=True)
class ValOptions:

    default: bool = False
    # Generic
    not_loaded_item: ValOption = Absent
    item_kind: ValOption = Absent
    skill_reqs: ValOption = Absent
    # Implants/boosters
    implant_slot_index: ValOption = Absent
    booster_slot_index: ValOption = Absent
    # Shared between mod-alike items
    cpu: ValOption = Absent
    powergrid: ValOption = Absent
    ship_limit: ValOption = Absent
    max_group_fitted: ValOption = Absent
    max_group_online: ValOption = Absent
    max_group_active: ValOption = Absent
    max_type_fitted: ValOption = Absent
    item_vs_ship_kind: ValOption = Absent
    # Modules
    high_slot_count: ValOption = Absent
    mid_slot_count: ValOption = Absent
    low_slot_count: ValOption = Absent
    turret_slot_count: ValOption = Absent
    launcher_slot_count: ValOption = Absent
    module_state: ValOption = Absent
    capital_module: ValOption = Absent
    overload_skill: ValOption = Absent
    # Charges
    charge_group: ValOption = Absent
    charge_size: ValOption = Absent
    charge_volume: ValOption = Absent
    # Rigs
    rig_slot_count: ValOption = Absent
    calibration: ValOption = Absent
    rig_size: ValOption = Absent
    # Services
    service_slot_count: ValOption = Absent
    # T3 subsystems/stances
    subsystem_slot_count: ValOption = Absent
    subsystem_slot_index: ValOption = Absent
    ship_stance: ValOption = Absent
    # Drones
    drone_bay_volume: ValOption = Absent
    launched_drone_count: ValOption = Absent
    drone_bandwidth: ValOption = Absent
    unlaunchable_drone_slot: ValOption = Absent
    unlaunchable_drone_bandwidth: ValOption = Absent
    drone_group: ValOption = Absent
    # Fighters
    fighter_bay_volume: ValOption = Absent
    launched_fighter_count: ValOption = Absent
    launched_light_fighter_count: ValOption = Absent
    launched_heavy_fighter_count: ValOption = Absent
    launched_support_fighter_count: ValOption = Absent
    launched_st_light_fighter_count: ValOption = Absent
    launched_st_heavy_fighter_count: ValOption = Absent
    launched_st_support_fighter_count: ValOption = Absent
    unlaunchable_fighter: ValOption = Absent
    unlaunchable_light_fighter: ValOption = Absent
    unlaunchable_heavy_fighter: ValOption = Absent
    unlaunchable_support_fighter: ValOption = Absent
    unlaunchable_st_light_fighter: ValOption = Absent
    unlaunchable_st_heavy_fighter: ValOption = Absent
    unlaunchable_st_support_fighter: ValOption = Absent
    fighter_squad_size: ValOption = Absent
    # Projection, destination side
    activation_blocked: ValOption = Absent
    effect_stopper: ValOption = Absent
    # Projection, source side
    assist_immunity: ValOption = Absent
    offense_immunity: ValOption = Absent
    resist_immunity: ValOption = Absent
    # Sec zone
    sec_zone_fitted: ValOption = Absent
    sec_zone_online: ValOption = Absent
    sec_zone_active: ValOption = Absent
    sec_zone_unonlineable: ValOption = Absent
    sec_zone_unactivable: ValOption = Absent

    def to_dict(self) -> dict:
        return dataclasses.asdict(self, dict_factory=lambda d: {k: v for k, v in d if v is not Absent})
