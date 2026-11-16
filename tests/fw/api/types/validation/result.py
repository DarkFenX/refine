from fw.util import AttrDict, AttrHookDef
from .val_activation_blocked import ValActivationBlockedFail
from .val_capital_module import ValCapModuleFail
from .val_charge_group import ValChargeGroupFail
from .val_charge_size import ValChargeSizeFail
from .val_charge_volume import ValChargeVolumeFail
from .val_cloaking_blocked import ValCloakingBlockedFail
from .val_drone_group import ValDroneGroupFail
from .val_effect_stopper import ValEffectStopperFail
from .val_fighter_squad_size import ValFighterSquadSizeFail
from .val_item_kind import ValItemKindFail
from .val_item_vs_ship_kind import ValItemVsShipKindFail
from .val_max_group import ValMaxGroupFail
from .val_max_type import ValMaxTypeFail
from .val_module_state import ValModuleStateFail
from .val_not_loaded_item import ValNotLoadedItemFail
from .val_overload_skill import ValOverloadSkillFail
from .val_proj_immunity import ValProjImmunityFail
from .val_projectee_filter import ValProjFilterFail
from .val_resources import ValResourceFail
from .val_result import ValResult
from .val_rig_size import ValRigSizeFail
from .val_sec_zone_effect import ValEffectSecZoneFail
from .val_sec_zone_item import ValItemSecZoneFail
from .val_ship_limit import ValShipLimitFail
from .val_ship_stance import ValShipStanceFail
from .val_skill_reqs import ValSrqFail
from .val_slot_count import ValSlotCountFail
from .val_slot_index import ValSlotIndexFail
from .val_unusable_cap import ValUnusableCapFail
from .val_unusable_resource import ValUnusableResFail
from .val_unusable_slot import ValUnusableSlotFail


class SolValResult(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={'details': AttrHookDef(func=lambda d: SolValDetails(data=d))})

    @property
    def fits(self) -> dict[str, FitValDetails]:
        details = getattr(self, 'details', AttrDict(data={}))
        return getattr(details, 'fits', {})


class SolValDetails(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'fits': AttrHookDef(func=lambda d: {k: FitValDetails(data=v) for k, v in d.items()}),
            'not_loaded_item': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValNotLoadedItemFail))})


class FitValResult(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={'details': AttrHookDef(func=lambda d: FitValDetails(data=d))})


class FitValDetails(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            # Generic
            'not_loaded_item': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValNotLoadedItemFail)),
            'item_kind': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValItemKindFail)),
            'skill_reqs': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSrqFail)),
            # Implants/boosters
            'implant_slot_index': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotIndexFail)),
            'booster_slot_index': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotIndexFail)),
            # Shared between mod-alike items
            'cpu': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValResourceFail)),
            'powergrid': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValResourceFail)),
            'ship_limit': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValShipLimitFail)),
            'max_group_fitted': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValMaxGroupFail)),
            'max_group_online': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValMaxGroupFail)),
            'max_group_active': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValMaxGroupFail)),
            'max_type_fitted': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValMaxTypeFail)),
            'item_vs_ship_kind': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValItemVsShipKindFail)),
            # Modules
            'high_slot_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'mid_slot_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'low_slot_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'turret_slot_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'launcher_slot_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'module_state': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValModuleStateFail)),
            'capital_module': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValCapModuleFail)),
            'overload_skill': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValOverloadSkillFail)),
            'unusable_cap': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValUnusableCapFail)),
            # Charges
            'charge_group': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValChargeGroupFail)),
            'charge_size': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValChargeSizeFail)),
            'charge_volume': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValChargeVolumeFail)),
            # Rigs
            'rig_slot_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'calibration': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValResourceFail)),
            'rig_size': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValRigSizeFail)),
            # Services
            'service_slot_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            # T3 subsystems/stances
            'subsystem_slot_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'subsystem_slot_index': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotIndexFail)),
            'ship_stance': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValShipStanceFail)),
            # Drones
            'drone_bay_volume': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValResourceFail)),
            'launched_drone_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'drone_bandwidth': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValResourceFail)),
            'unlaunchable_drone_slot': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValUnusableSlotFail)),
            'unlaunchable_drone_bandwidth': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValUnusableResFail)),
            'drone_group': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValDroneGroupFail)),
            # Fighters
            'fighter_bay_volume': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValResourceFail)),
            'launched_fighter_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'launched_light_fighter_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'launched_heavy_fighter_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'launched_support_fighter_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'launched_st_light_fighter_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'launched_st_heavy_fighter_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'launched_st_support_fighter_count': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValSlotCountFail)),
            'unlaunchable_fighter': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValUnusableSlotFail)),
            'unlaunchable_light_fighter': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValUnusableSlotFail)),
            'unlaunchable_heavy_fighter': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValUnusableSlotFail)),
            'unlaunchable_support_fighter': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValUnusableSlotFail)),
            'unlaunchable_st_light_fighter': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValUnusableSlotFail)),
            'unlaunchable_st_heavy_fighter': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValUnusableSlotFail)),
            'unlaunchable_st_support_fighter': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValUnusableSlotFail)),
            'fighter_squad_size': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValFighterSquadSizeFail)),
            # Projection, destination side
            'activation_blocked': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValActivationBlockedFail)),
            'effect_stopper': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValEffectStopperFail)),
            'cloaking_blocked': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValCloakingBlockedFail)),
            # Projection, source side
            'projectee_filter': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValProjFilterFail)),
            'assist_immunity': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValProjImmunityFail)),
            'offense_immunity': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValProjImmunityFail)),
            'resist_immunity': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValProjImmunityFail)),
            # Sec zone
            'sec_zone_fitted': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValItemSecZoneFail)),
            'sec_zone_online': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValItemSecZoneFail)),
            'sec_zone_active': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValItemSecZoneFail)),
            'sec_zone_unonlineable': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValItemSecZoneFail)),
            'sec_zone_unactivable': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValItemSecZoneFail)),
            'sec_zone_effect': AttrHookDef(func=lambda d: ValResult(data=d, cls=ValEffectSecZoneFail))})
