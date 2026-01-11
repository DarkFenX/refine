use crate::ad::{AAttrId, AEffectBuff, AEffectCatId, AEffectId, AEffectModifiers, AEffectStopIds, AState};

pub struct AEffect {
    pub id: AEffectId,
    pub category: AEffectCatId,
    pub state: AState,
    pub modifiers: AEffectModifiers = AEffectModifiers::new(),
    pub stopped_effect_ids: AEffectStopIds = AEffectStopIds::new(),
    pub buff: Option<AEffectBuff> = None,
    pub is_assist: bool = false,
    pub is_offense: bool = false,
    pub banned_in_hisec: bool = false,
    pub banned_in_lowsec: bool = false,
    pub discharge_attr_id: Option<AAttrId> = None,
    pub duration_attr_id: Option<AAttrId> = None,
    pub range_attr_id: Option<AAttrId> = None,
    pub falloff_attr_id: Option<AAttrId> = None,
    pub track_attr_id: Option<AAttrId> = None,
    pub chance_attr_id: Option<AAttrId> = None,
    pub resist_attr_id: Option<AAttrId> = None,
}
impl std::fmt::Display for AEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AEffect(id={})", self.id)
    }
}
