use crate::{
    ad::{
        AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId, AItemId, AItemXt, ASkillLevel, AState,
    },
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

/// Adapted item with extra data added to it during runtime.
pub struct AItemRt {
    /// Adapted item.
    pub ai: AItem,
    /// Extra data, which is generated using data from adapted data during runtime.
    pub(crate) xt: AItemXt,
}
impl AItemRt {
    /// Construct new adapted item with extra data.
    pub fn new(a_item: AItem) -> Self {
        let xt = AItemXt::new_initial(&a_item);
        Self { ai: a_item, xt }
    }
}
impl Named for AItemRt {
    fn get_name() -> &'static str {
        "AItemRt"
    }
}
impl std::fmt::Display for AItemRt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.ai.id)
    }
}
