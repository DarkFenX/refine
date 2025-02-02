use crate::handler_json::data::{CItemChargeLimit, CItemKind, CItemShipLimit, CShipKind};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItemExtras {
    pub(in crate::handler_json) kind: Option<CItemKind>,
    pub(in crate::handler_json) volume: Option<rc::AttrVal>,
    pub(in crate::handler_json) ship_limit: Option<CItemShipLimit>,
    pub(in crate::handler_json) charge_limit: Option<CItemChargeLimit>,
    pub(in crate::handler_json) val_fitted_group_id: Option<rc::EItemGrpId>,
    pub(in crate::handler_json) val_online_group_id: Option<rc::EItemGrpId>,
    pub(in crate::handler_json) val_active_group_id: Option<rc::EItemGrpId>,
    pub(in crate::handler_json) implant_slot: Option<rc::SlotIndex>,
    pub(in crate::handler_json) booster_slot: Option<rc::SlotIndex>,
    pub(in crate::handler_json) subsystem_slot: Option<rc::SlotIndex>,
    pub(in crate::handler_json) is_light_fighter: bool,
    pub(in crate::handler_json) is_heavy_fighter: bool,
    pub(in crate::handler_json) is_support_fighter: bool,
    pub(in crate::handler_json) is_standup_light_fighter: bool,
    pub(in crate::handler_json) is_standup_heavy_fighter: bool,
    pub(in crate::handler_json) is_standup_support_fighter: bool,
    pub(in crate::handler_json) ship_kind: Option<CShipKind>,
    pub(in crate::handler_json) item_ship_kind: Option<CShipKind>,
}
impl From<&rc::ad::AItemExtras> for CItemExtras {
    fn from(a_item_extras: &rc::ad::AItemExtras) -> Self {
        CItemExtras {
            kind: a_item_extras.kind.as_ref().map(|v| v.into()),
            volume: a_item_extras.volume,
            ship_limit: a_item_extras.ship_limit.as_ref().map(|v| v.into()),
            charge_limit: a_item_extras.charge_limit.as_ref().map(|v| v.into()),
            val_fitted_group_id: a_item_extras.val_fitted_group_id,
            val_online_group_id: a_item_extras.val_online_group_id,
            val_active_group_id: a_item_extras.val_active_group_id,
            implant_slot: a_item_extras.implant_slot,
            booster_slot: a_item_extras.booster_slot,
            subsystem_slot: a_item_extras.subsystem_slot,
            is_light_fighter: a_item_extras.is_light_fighter,
            is_heavy_fighter: a_item_extras.is_heavy_fighter,
            is_support_fighter: a_item_extras.is_support_fighter,
            is_standup_light_fighter: a_item_extras.is_standup_light_fighter,
            is_standup_heavy_fighter: a_item_extras.is_standup_heavy_fighter,
            is_standup_support_fighter: a_item_extras.is_standup_support_fighter,
            ship_kind: a_item_extras.ship_kind.as_ref().map(|v| v.into()),
            item_ship_kind: a_item_extras.item_ship_kind.as_ref().map(|v| v.into()),
        }
    }
}
impl Into<rc::ad::AItemExtras> for &CItemExtras {
    fn into(self) -> rc::ad::AItemExtras {
        rc::ad::AItemExtras {
            kind: self.kind.as_ref().map(|v| v.into()),
            volume: self.volume,
            ship_limit: self.ship_limit.as_ref().map(|v| v.into()),
            charge_limit: self.charge_limit.as_ref().map(|v| v.into()),
            val_fitted_group_id: self.val_fitted_group_id,
            val_online_group_id: self.val_online_group_id,
            val_active_group_id: self.val_active_group_id,
            implant_slot: self.implant_slot,
            booster_slot: self.booster_slot,
            subsystem_slot: self.subsystem_slot,
            is_light_fighter: self.is_light_fighter,
            is_heavy_fighter: self.is_heavy_fighter,
            is_support_fighter: self.is_support_fighter,
            is_standup_light_fighter: self.is_standup_light_fighter,
            is_standup_heavy_fighter: self.is_standup_heavy_fighter,
            is_standup_support_fighter: self.is_standup_support_fighter,
            ship_kind: self.ship_kind.as_ref().map(|v| v.into()),
            item_ship_kind: self.item_ship_kind.as_ref().map(|v| v.into()),
        }
    }
}
