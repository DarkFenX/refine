use crate::{
    ad::{
        AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId, AItemId, AItemXt, ASkillLevel, AState,
    },
    util::{Named, RMap},
};

pub struct AItem {
    pub id: AItemId,
    /// Item group ID.
    pub grp_id: AItemGrpId,
    /// Item category ID.
    pub cat_id: AItemCatId,
    /// Attribute values of the item.
    pub attrs: RMap<AAttrId, AAttrVal>,
    /// Refers effects of the item.
    pub effect_datas: RMap<AEffectId, AItemEffectData>,
    /// Refers an effect which is default for the item.
    pub defeff_id: Option<AEffectId>,
    /// Skill requirement map.
    pub srqs: RMap<AItemId, ASkillLevel>,
    /// Max state item can take.
    pub max_state: AState,
    /// Item effectively has this group ID for purposes of "max group fitted" validation.
    pub val_fitted_group_id: Option<AItemGrpId>,
    /// Item effectively has this group ID for purposes of "max group online" validation.
    pub val_online_group_id: Option<AItemGrpId>,
    /// Item effectively has this group ID for purposes of "max group active" validation.
    pub val_active_group_id: Option<AItemGrpId>,
    /// Can ship be in wormhole space or not.
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
