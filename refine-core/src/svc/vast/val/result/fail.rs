use crate::{
    ItemId,
    val::{
        ValActivationBlockedFail, ValCapitalModFail, ValChargeGroupFail, ValChargeSizeFail, ValChargeVolumeFail,
        ValCloakingBlockedFail, ValDroneGroupFail, ValEffectSecZoneFail, ValEffectStopperFail, ValFighterSquadSizeFail,
        ValItemKindFail, ValItemSecZoneFail, ValItemVsShipKindFail, ValMaxGroupFail, ValMaxTypeFail,
        ValModuleStateFail, ValNotLoadedItemFail, ValOverloadSkillFail, ValProjFilterFail, ValProjImmunityFail,
        ValResourceFail, ValRigSizeFail, ValShipLimitFail, ValShipStanceFail, ValSlotCountFail, ValSlotIndexFail,
        ValSrqFail, ValUnusableCapFail, ValUnusableResFail, ValUnusableSlotFail,
    },
};

/// Failure details of a validation.
pub(in crate::svc::vast::val) trait ValFail {
    /// Items which fail the validation, known failures included.
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId>;
}

impl ValFail for ValActivationBlockedFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.modules.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValCapitalModFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.module_volumes.iter().map(|v| v.module_id)
    }
}
impl ValFail for ValChargeGroupFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.charges.iter().map(|v| v.charge_item_id)
    }
}
impl ValFail for ValChargeSizeFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.charges.iter().map(|v| v.charge_item_id)
    }
}
impl ValFail for ValChargeVolumeFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.charges.iter().map(|v| v.charge_item_id)
    }
}
impl ValFail for ValCloakingBlockedFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.modules.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValDroneGroupFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.drone_groups.iter().map(|v| v.drone_id)
    }
}
impl ValFail for ValEffectSecZoneFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.items.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValEffectStopperFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.items.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValFighterSquadSizeFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.fighters.iter().map(|v| v.fighter_id)
    }
}
impl ValFail for ValItemKindFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.item_kinds.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValItemSecZoneFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.items.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValItemVsShipKindFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.items.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValMaxGroupFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.groups.iter().flat_map(|v| v.items.iter().map(|v| v.item_id))
    }
}
impl ValFail for ValMaxTypeFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.item_types.iter().flat_map(|v| v.items.iter().map(|v| v.item_id))
    }
}
impl ValFail for ValModuleStateFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.modules.iter().map(|v| v.module_id)
    }
}
impl ValFail for ValNotLoadedItemFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.items.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValOverloadSkillFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.module_reqs.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValProjFilterFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.items.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValProjImmunityFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.items.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValResourceFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.users.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValRigSizeFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.rig_sizes.iter().map(|v| v.rig_id)
    }
}
impl ValFail for ValShipLimitFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.items.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValShipStanceFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        std::iter::once(self.stance_item_id)
    }
}
impl ValFail for ValSlotCountFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.users.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValSlotIndexFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.slot_users.iter().flat_map(|v| v.items.iter().map(|v| v.item_id))
    }
}
impl ValFail for ValSrqFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.items.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValUnusableCapFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.items.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValUnusableResFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.users.iter().map(|v| v.item_id)
    }
}
impl ValFail for ValUnusableSlotFail {
    fn iter_item_ids(&self) -> impl Iterator<Item = ItemId> {
        self.users.iter().map(|v| v.item_id)
    }
}
