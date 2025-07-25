use crate::{
    ad::{AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId, AItemId, ASkillLevel, AState},
    util::{Named, RMap},
};

pub struct AItem {
    pub id: AItemId,
    pub grp_id: AItemGrpId,
    pub cat_id: AItemCatId,
    pub attrs: RMap<AAttrId, AAttrVal>,
    pub effect_datas: RMap<AEffectId, AItemEffectData>,
    pub defeff_id: Option<AEffectId>,
    pub srqs: RMap<AItemId, ASkillLevel>,
    pub max_state: AState,
    pub val_fitted_group_id: Option<AItemGrpId>,
    pub val_online_group_id: Option<AItemGrpId>,
    pub val_active_group_id: Option<AItemGrpId>,
    pub disallowed_in_wspace: bool,
}
impl Named for AItem {
    fn get_name() -> &'static str {
        "AItem"
    }
}
impl std::fmt::Display for AItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}
