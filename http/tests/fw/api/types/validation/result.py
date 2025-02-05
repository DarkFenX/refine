from tests.fw.consts import ApiValType
from tests.fw.util import AttrDict, AttrHookDef
from .capital_module import ValCapModuleDetails
from .charge_group import ValChargeGroupDetails
from .charge_size import ValChargeSizeDetails
from .charge_volume import ValChargeVolumeDetails
from .max_group import ValMaxGroupDetails
from .not_loaded_item import ValNotLoadedItemDetails
from .resources import ValResourceDetails
from .rig_size import ValRigSizeDetails
from .ship_limit import ValShipLimitDetails
from .skill_reqs import ValSrqDetails
from .slot_amount import ValSlotAmountDetails
from .slot_index import ValSlotIndexDetails


class ValResult(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={'details': AttrHookDef(func=lambda d: ValResultDetails(data=d))})


class ValResultDetails(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            ApiValType.cpu: AttrHookDef(func=lambda d: ValResourceDetails(data=d)),
            ApiValType.powergrid: AttrHookDef(func=lambda d: ValResourceDetails(data=d)),
            ApiValType.calibration: AttrHookDef(func=lambda d: ValResourceDetails(data=d)),
            ApiValType.dronebay_volume: AttrHookDef(func=lambda d: ValResourceDetails(data=d)),
            ApiValType.drone_bandwidth: AttrHookDef(func=lambda d: ValResourceDetails(data=d)),
            ApiValType.rig_slots: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.subsystem_slots: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.launched_drones: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.launched_fighters: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.launched_support_fighters: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.launched_light_fighters: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.launched_heavy_fighters: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.launched_standup_support_fighters: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.launched_standup_light_fighters: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.launched_standup_heavy_fighters: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.turret_slots: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.launcher_slots: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.high_slots: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.mid_slots: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.low_slots: AttrHookDef(func=lambda d: ValSlotAmountDetails(data=d)),
            ApiValType.implant_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            ApiValType.booster_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            ApiValType.subsystem_slot_index: AttrHookDef(func=lambda d: ValSlotIndexDetails(data=d)),
            ApiValType.ship_limit: AttrHookDef(func=lambda d: ValShipLimitDetails(data=d)),
            ApiValType.max_group_fitted: AttrHookDef(func=lambda d: ValMaxGroupDetails(data=d)),
            ApiValType.max_group_online: AttrHookDef(func=lambda d: ValMaxGroupDetails(data=d)),
            ApiValType.max_group_active: AttrHookDef(func=lambda d: ValMaxGroupDetails(data=d)),
            ApiValType.rig_size: AttrHookDef(func=lambda d: ValRigSizeDetails(data=d)),
            ApiValType.skill_reqs: AttrHookDef(func=lambda d: ValSrqDetails(data=d)),
            ApiValType.charge_group: AttrHookDef(func=lambda d: ValChargeGroupDetails(data=d)),
            ApiValType.charge_size: AttrHookDef(func=lambda d: ValChargeSizeDetails(data=d)),
            ApiValType.charge_volume: AttrHookDef(func=lambda d: ValChargeVolumeDetails(data=d)),
            ApiValType.capital_module: AttrHookDef(func=lambda d: ValCapModuleDetails(data=d)),
            ApiValType.not_loaded_item: AttrHookDef(func=lambda d: ValNotLoadedItemDetails(data=d))})
