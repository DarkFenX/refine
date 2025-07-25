use crate::{
    ad::{AAttrId, AEffectBuffInfo, AEffectCatId, AEffectId, AEffectModifier, AState},
    util::Named,
};

pub struct AEffect {
    pub id: AEffectId,
    pub category: AEffectCatId,
    pub state: AState,
    pub is_assist: bool = false,
    pub is_offense: bool = false,
    pub hisec: Option<bool> = None,
    pub lowsec: Option<bool> = None,
    pub discharge_attr_id: Option<AAttrId> = None,
    pub duration_attr_id: Option<AAttrId> = None,
    pub range_attr_id: Option<AAttrId> = None,
    pub falloff_attr_id: Option<AAttrId> = None,
    pub track_attr_id: Option<AAttrId> = None,
    pub chance_attr_id: Option<AAttrId> = None,
    pub resist_attr_id: Option<AAttrId> = None,
    pub mods: Vec<AEffectModifier> = Vec::new(),
    pub stoped_effect_ids: Vec<AEffectId> = Vec::new(),
    pub buff_info: Option<AEffectBuffInfo> = None,
}
impl Named for AEffect {
    fn get_name() -> &'static str {
        "AEffect"
    }
}
impl std::fmt::Display for AEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}
