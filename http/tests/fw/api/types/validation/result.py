from tests.fw.consts import ApiValType
from tests.fw.util import AttrDict, AttrHookDef
from .capital_module import ValCapModuleFail
from .charge_group import ValChargeGroupFail
from .charge_size import ValChargeSizeFail
from .charge_volume import ValChargeVolumeFail
from .drone_group import ValDroneGroupFail
from .item_kind import ValItemKindFail
from .max_group import ValMaxGroupFail
from .module_state import ValModuleStateFail
from .not_loaded_item import ValNotLoadedItemFail
from .resources import ValResourceFail
from .rig_size import ValRigSizeFail
from .ship_limit import ValShipLimitFail
from .skill_reqs import ValSrqFail
from .slot_count import ValSlotCountFail
from .slot_index import ValSlotIndexFail


class ValResult(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={'details': AttrHookDef(func=lambda d: ValResultDetails(data=d))})


class ValResultDetails(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            ApiValType.cpu: AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            ApiValType.powergrid: AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            ApiValType.calibration: AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            ApiValType.dronebay_volume: AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            ApiValType.drone_bandwidth: AttrHookDef(func=lambda d: ValResourceFail(data=d)),
            ApiValType.rig_slot_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.subsystem_slot_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.launched_drone_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.launched_fighter_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.launched_support_fighter_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.launched_light_fighter_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.launched_heavy_fighter_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.launched_standup_support_fighter_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.launched_standup_light_fighter_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.launched_standup_heavy_fighter_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.turret_slot_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.launcher_slot_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.high_slot_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.mid_slot_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.low_slot_count: AttrHookDef(func=lambda d: ValSlotCountFail(data=d)),
            ApiValType.implant_slot_index: AttrHookDef(func=lambda d: ValSlotIndexFail(data=d)),
            ApiValType.booster_slot_index: AttrHookDef(func=lambda d: ValSlotIndexFail(data=d)),
            ApiValType.subsystem_slot_index: AttrHookDef(func=lambda d: ValSlotIndexFail(data=d)),
            ApiValType.ship_limit: AttrHookDef(func=lambda d: ValShipLimitFail(data=d)),
            ApiValType.max_group_fitted: AttrHookDef(func=lambda d: ValMaxGroupFail(data=d)),
            ApiValType.max_group_online: AttrHookDef(func=lambda d: ValMaxGroupFail(data=d)),
            ApiValType.max_group_active: AttrHookDef(func=lambda d: ValMaxGroupFail(data=d)),
            ApiValType.rig_size: AttrHookDef(func=lambda d: ValRigSizeFail(data=d)),
            ApiValType.skill_reqs: AttrHookDef(func=lambda d: ValSrqFail(data=d)),
            ApiValType.charge_group: AttrHookDef(func=lambda d: ValChargeGroupFail(data=d)),
            ApiValType.charge_size: AttrHookDef(func=lambda d: ValChargeSizeFail(data=d)),
            ApiValType.charge_volume: AttrHookDef(func=lambda d: ValChargeVolumeFail(data=d)),
            ApiValType.capital_module: AttrHookDef(func=lambda d: ValCapModuleFail(data=d)),
            ApiValType.not_loaded_item: AttrHookDef(func=lambda d: ValNotLoadedItemFail(data=d)),
            ApiValType.module_state: AttrHookDef(func=lambda d: ValModuleStateFail(data=d)),
            ApiValType.item_kind: AttrHookDef(func=lambda d: ValItemKindFail(data=d)),
            ApiValType.drone_group: AttrHookDef(func=lambda d: ValDroneGroupFail(data=d))})
