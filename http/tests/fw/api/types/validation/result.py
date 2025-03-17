from tests.fw.util import AttrDict, AttrHookDef
from .capital_module import ValCapModuleFail
from .charge_group import ValChargeGroupFail
from .charge_size import ValChargeSizeFail
from .charge_volume import ValChargeVolumeFail
from .drone_group import ValDroneGroupFail
from .fighter_count import ValFighterCountFail
from .item_kind import ValItemKindFail
from .max_group import ValMaxGroupFail
from .module_state import ValModuleStateFail
from .not_loaded_item import ValNotLoadedItemFail
from .resources import ValResourceFail
from .rig_size import ValRigSizeFail
from .ship_limit import ValShipLimitFail
from .ship_stance import ValShipStanceFail
from .skill_reqs import ValSrqFail
from .slot_count import ValSlotCountFail
from .slot_index import ValSlotIndexFail
from .unusable_resource import ValUnusableResFail
from .unusable_slot import ValUnusableSlotFail


class ValResult(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={'details': AttrHookDef(func=lambda d: ValResultDetails(data=d))})


class ValResultDetails(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'cpu': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'powergrid': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'calibration': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'drone_bay_volume': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'drone_bandwidth': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'fighter_bay_volume': AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            'rig_slot_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'subsystem_slot_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_drone_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_support_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_light_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_heavy_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_standup_support_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_standup_light_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            'launched_standup_heavy_fighter_count': AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
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
            'fighter_count': AttrHookDef(func=lambda d: ValFighterCountFail(data=d)),
            'unlaunchable_drone_slot': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_drone_bandwidth': AttrHookDef(func=lambda d: ValUnusableResFail(data=d)),
            'unlaunchable_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_support_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_light_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_heavy_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_standup_support_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_standup_light_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'unlaunchable_standup_heavy_fighter': AttrHookDef(func=lambda d: ValUnusableSlotFail(data=d)),
            'ship_stance': AttrHookDef(func=lambda d: ValShipStanceFail(data=d))})
