use crate::cacher_json::data::{CAttrId, CEffectBuffInfo, CEffectCatId, CEffectId, CEffectModifier, CState};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CEffect {
    id: CEffectId,
    category: CEffectCatId,
    state: CState,
    is_assist: bool,
    is_offense: bool,
    is_usable_in_hisec: Option<bool>,
    is_usable_in_lowsec: Option<bool>,
    discharge_attr_id: Option<CAttrId>,
    duration_attr_id: Option<CAttrId>,
    range_attr_id: Option<CAttrId>,
    falloff_attr_id: Option<CAttrId>,
    track_attr_id: Option<CAttrId>,
    chance_attr_id: Option<CAttrId>,
    resist_attr_id: Option<CAttrId>,
    mods: Vec<CEffectModifier>,
    stop_ids: Vec<CEffectId>,
    buff: Option<CEffectBuffInfo>,
}
impl From<&rc::ad::AEffect> for CEffect {
    fn from(a_effect: &rc::ad::AEffect) -> Self {
        Self {
            id: (&a_effect.id).into(),
            category: a_effect.category,
            state: (&a_effect.state).into(),
            is_assist: a_effect.is_assist,
            is_offense: a_effect.is_offense,
            is_usable_in_hisec: a_effect.is_usable_in_hisec,
            is_usable_in_lowsec: a_effect.is_usable_in_lowsec,
            discharge_attr_id: a_effect.discharge_attr_id,
            duration_attr_id: a_effect.duration_attr_id,
            range_attr_id: a_effect.range_attr_id,
            falloff_attr_id: a_effect.falloff_attr_id,
            track_attr_id: a_effect.track_attr_id,
            chance_attr_id: a_effect.chance_attr_id,
            resist_attr_id: a_effect.resist_attr_id,
            mods: a_effect.mods.iter().map(Into::into).collect(),
            stop_ids: a_effect.stoped_effect_ids.iter().map(Into::into).collect(),
            buff: a_effect.buff_info.as_ref().map(Into::into),
        }
    }
}
impl From<&CEffect> for rc::ad::AEffect {
    fn from(c_effect: &CEffect) -> Self {
        Self {
            id: (&c_effect.id).into(),
            category: c_effect.category,
            state: (&c_effect.state).into(),
            is_assist: c_effect.is_assist,
            is_offense: c_effect.is_offense,
            is_usable_in_hisec: c_effect.is_usable_in_hisec,
            is_usable_in_lowsec: c_effect.is_usable_in_lowsec,
            discharge_attr_id: c_effect.discharge_attr_id,
            duration_attr_id: c_effect.duration_attr_id,
            range_attr_id: c_effect.range_attr_id,
            falloff_attr_id: c_effect.falloff_attr_id,
            track_attr_id: c_effect.track_attr_id,
            chance_attr_id: c_effect.chance_attr_id,
            resist_attr_id: c_effect.resist_attr_id,
            mods: c_effect.mods.iter().map(Into::into).collect(),
            stoped_effect_ids: c_effect.stop_ids.iter().map(Into::into).collect(),
            buff_info: c_effect.buff.as_ref().map(Into::into),
        }
    }
}
