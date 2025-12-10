use crate::{
    ad::{
        AAbilId, AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId, AItemId, AItemListId,
        ASkillLevel, AState,
    },
    util::RMap,
};

pub struct AItem {
    pub id: AItemId,
    pub grp_id: AItemGrpId,
    pub cat_id: AItemCatId,
    pub attrs: RMap<AAttrId, AAttrVal>,
    pub effect_datas: RMap<AEffectId, AItemEffectData>,
    pub defeff_id: Option<AEffectId>,
    pub abil_ids: Vec<AAbilId>,
    pub srqs: RMap<AItemId, ASkillLevel>,
    pub proj_buff_item_list_ids: Vec<AItemListId>,
    pub fleet_buff_item_list_ids: Vec<AItemListId>,
    pub max_state: AState,
    pub val_fitted_group_id: Option<AItemGrpId>,
    pub val_online_group_id: Option<AItemGrpId>,
    pub val_active_group_id: Option<AItemGrpId>,
    pub disallowed_in_wspace: bool,
}
