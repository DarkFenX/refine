use crate::ad::{
    AEffectId, AItemAbils, AItemAttrs, AItemBuffItemLists, AItemCatId, AItemEffects, AItemGrpId, AItemId,
    AItemSkillReqs, AState,
};

pub struct AItem {
    pub id: AItemId,
    pub grp_id: AItemGrpId,
    pub cat_id: AItemCatId,
    pub attrs: AItemAttrs,
    pub effects: AItemEffects,
    pub defeff_id: Option<AEffectId>,
    pub abil_ids: AItemAbils,
    pub srqs: AItemSkillReqs,
    pub max_state: AState,
    pub proj_buff_item_list_ids: AItemBuffItemLists,
    pub fleet_buff_item_list_ids: AItemBuffItemLists,
    pub val_fitted_group_id: Option<AItemGrpId>,
    pub val_online_group_id: Option<AItemGrpId>,
    pub val_active_group_id: Option<AItemGrpId>,
    pub is_cloak: bool,
    pub is_ice_harvester: bool,
    pub disallowed_in_wspace: bool,
    pub enables_bridge: bool,
}
