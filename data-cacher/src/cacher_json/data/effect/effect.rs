use crate::cacher_json::data::{CAttrId, CEffectBuffInfo, CEffectCatId, CEffectId, CEffectModifier, CState};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CEffect {
    id: CEffectId,
    category: CEffectCatId,
    state: CState,
    modifiers: Vec<CEffectModifier>,
    stoped_effect_ids: Vec<CEffectId>,
    buff: Option<CEffectBuffInfo>,
    is_assist: bool,
    is_offense: bool,
    banned_in_hisec: bool,
    banned_in_lowsec: bool,
    discharge_attr_id: Option<CAttrId>,
    duration_attr_id: Option<CAttrId>,
    range_attr_id: Option<CAttrId>,
    falloff_attr_id: Option<CAttrId>,
    track_attr_id: Option<CAttrId>,
    chance_attr_id: Option<CAttrId>,
    resist_attr_id: Option<CAttrId>,
}
impl From<&rc::ad::AEffect> for CEffect {
    fn from(a_effect: &rc::ad::AEffect) -> Self {
        Self {
            id: (&a_effect.id).into(),
            category: a_effect.category,
            state: (&a_effect.state).into(),
            modifiers: a_effect.modifiers.iter().map(Into::into).collect(),
            stoped_effect_ids: a_effect.stoped_effect_ids.iter().map(Into::into).collect(),
            buff: a_effect.buff.as_ref().map(Into::into),
            is_assist: a_effect.is_assist,
            is_offense: a_effect.is_offense,
            banned_in_hisec: a_effect.banned_in_hisec,
            banned_in_lowsec: a_effect.banned_in_lowsec,
            discharge_attr_id: a_effect.discharge_attr_id.as_ref().map(Into::into),
            duration_attr_id: a_effect.duration_attr_id.as_ref().map(Into::into),
            range_attr_id: a_effect.range_attr_id.as_ref().map(Into::into),
            falloff_attr_id: a_effect.falloff_attr_id.as_ref().map(Into::into),
            track_attr_id: a_effect.track_attr_id.as_ref().map(Into::into),
            chance_attr_id: a_effect.chance_attr_id.as_ref().map(Into::into),
            resist_attr_id: a_effect.resist_attr_id.as_ref().map(Into::into),
        }
    }
}
impl From<&CEffect> for rc::ad::AEffect {
    fn from(c_effect: &CEffect) -> Self {
        Self {
            id: (&c_effect.id).into(),
            category: c_effect.category,
            state: (&c_effect.state).into(),
            modifiers: c_effect.modifiers.iter().map(Into::into).collect(),
            stoped_effect_ids: c_effect.stoped_effect_ids.iter().map(Into::into).collect(),
            buff: c_effect.buff.as_ref().map(Into::into),
            is_assist: c_effect.is_assist,
            is_offense: c_effect.is_offense,
            banned_in_hisec: c_effect.banned_in_hisec,
            banned_in_lowsec: c_effect.banned_in_lowsec,
            discharge_attr_id: c_effect.discharge_attr_id.as_ref().map(Into::into),
            duration_attr_id: c_effect.duration_attr_id.as_ref().map(Into::into),
            range_attr_id: c_effect.range_attr_id.as_ref().map(Into::into),
            falloff_attr_id: c_effect.falloff_attr_id.as_ref().map(Into::into),
            track_attr_id: c_effect.track_attr_id.as_ref().map(Into::into),
            chance_attr_id: c_effect.chance_attr_id.as_ref().map(Into::into),
            resist_attr_id: c_effect.resist_attr_id.as_ref().map(Into::into),
        }
    }
}
