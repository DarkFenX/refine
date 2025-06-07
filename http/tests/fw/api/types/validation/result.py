from __future__ import annotations

from tests.fw.util import AttrDict, AttrHookDef
from .val_activation_blocked import ValActivationBlockedFail
from .val_capital_module import ValCapModuleFail
from .val_charge_group import ValChargeGroupFail
from .val_charge_size import ValChargeSizeFail
from .val_charge_volume import ValChargeVolumeFail
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
from .val_resources import ValResourceFail
from .val_rig_size import ValRigSizeFail
from .val_sec_zone import ValSecZoneFail
from .val_ship_limit import ValShipLimitFail
from .val_ship_stance import ValShipStanceFail
from .val_skill_reqs import ValSrqFail
from .val_slot_count import ValSlotCountFail
from .val_slot_index import ValSlotIndexFail
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
            'not_loaded_item': AttrHookDef(func=lambda d: ValNotLoadedItemFail(data=d))})


class FitValResult(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={'details': AttrHookDef(func=lambda d: FitValDetails(data=d))})


class FitValDetails(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'cpu': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'powergrid': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'calibration': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'drone_bay_volume': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'drone_bandwidth': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'fighter_bay_volume': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'rig_slot_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'service_slot_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'subsystem_slot_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_drone_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_light_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_heavy_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_support_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_st_light_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_st_heavy_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_st_support_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'turret_slot_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launcher_slot_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'high_slot_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'mid_slot_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'low_slot_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'implant_slot_index': AttrHookDef(func=lambda d: ValSlotIndexFail(data=d)),
            'booster_slot_index': AttrHookDef(func=lambda d: ValSlotIndexFail(data=d)),
            'subsystem_slot_index': AttrHookDef(func=lambda d: ValSlotIndexFail(data=d)),
            'ship_limit': AttrHookDef(func=lambda d: ValShipLimitFail(data=d)),
            'max_group_fitted': AttrHookDef(func=lambda d: ValMaxGroupFail(data=d)),
            'max_group_online': AttrHookDef(func=lambda d: ValMaxGroupFail(data=d)),
            'max_group_active': AttrHookDef(func=lambda d: ValMaxGroupFail(data=d)),
            'rig_size': AttrHookDef(func=lambda d: ValRigSizeFail(data=d)),
            'skill_reqs': AttrHookDef(func=lambda d: ValSrqFail(data=d)),
            'charge_group': AttrHookDef(func=lambda d: ValChargeGroupFail(data=d)),
            'charge_size': AttrHookDef(func=lambda d: ValChargeSizeFail(data=d)),
            'charge_volume': AttrHookDef(func=lambda d: ValChargeVolumeFail(data=d)),
            'capital_module': AttrHookDef(func=lambda d: ValCapModuleFail(data=d)),
            'not_loaded_item': AttrHookDef(func=lambda d: ValNotLoadedItemFail(data=d)),
            'module_state': AttrHookDef(func=lambda d: ValModuleStateFail(data=d)),
            'item_kind': AttrHookDef(func=lambda d: ValItemKindFail(data=d)),
            'drone_group': AttrHookDef(func=lambda d: ValDroneGroupFail(data=d)),
            'fighter_squad_size': AttrHookDef(func=lambda d: ValFighterSquadSizeFail(data=d)),
            'unlaunchable_drone_slot': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_drone_bandwidth': AttrHookDef(func=lambda d: ValUnusableResFail(data=d)),
            'unlaunchable_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_light_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_heavy_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_support_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_st_light_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_st_heavy_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_st_support_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'ship_stance': AttrHookDef(func=lambda d: ValShipStanceFail(data=d)),
            'overload_skill': AttrHookDef(func=lambda d: ValOverloadSkillFail(data=d)),
            'max_type_fitted': AttrHookDef(func=lambda d: ValMaxTypeFail(data=d)),
            'sec_zone_fitted': AttrHookDef(func=lambda d: ValSecZoneFail(data=d)),
            'sec_zone_online': AttrHookDef(func=lambda d: ValSecZoneFail(data=d)),
            'sec_zone_active': AttrHookDef(func=lambda d: ValSecZoneFail(data=d)),
            'sec_zone_unonlineable': AttrHookDef(func=lambda d: ValSecZoneFail(data=d)),
            'sec_zone_unactivable': AttrHookDef(func=lambda d: ValSecZoneFail(data=d)),
            'activation_blocked': AttrHookDef(func=lambda d: ValActivationBlockedFail(data=d)),
            'item_vs_ship_kind': AttrHookDef(func=lambda d: ValItemVsShipKindFail(data=d)),
            'effect_stopper': AttrHookDef(func=lambda d: ValEffectStopperFail(data=d)),
            'assist_immunity': AttrHookDef(func=lambda d: ValProjImmunityFail(data=d)),
            'offense_immunity': AttrHookDef(func=lambda d: ValProjImmunityFail(data=d)),
            'resist_immunity': AttrHookDef(func=lambda d: ValProjImmunityFail(data=d))})
