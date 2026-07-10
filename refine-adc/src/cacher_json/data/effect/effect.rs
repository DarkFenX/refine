use super::{buff::CEffectBuff, duration::CEffectAggroDuration, modifier::CEffectModifier};
use crate::cacher_json::data::{AdaptedConv, CState};

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json::data) struct CEffect {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ad::AEffectId,
    category: i32,
    state: CState,
    modifiers: Vec<CEffectModifier>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    stopped_effect_ids: Vec<rc::ad::AEffectId>,
    buff: Option<CEffectBuff>,
    aggro: Option<CEffectAggroDuration>,
    is_assist: bool,
    is_offense: bool,
    banned_in_hisec: bool,
    banned_in_lowsec: bool,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>", no_default)]
    discharge_attr_id: Option<rc::ad::AAttrId>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>", no_default)]
    duration_attr_id: Option<rc::ad::AAttrId>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>", no_default)]
    range_attr_id: Option<rc::ad::AAttrId>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>", no_default)]
    falloff_attr_id: Option<rc::ad::AAttrId>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>", no_default)]
    track_attr_id: Option<rc::ad::AAttrId>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>", no_default)]
    chance_attr_id: Option<rc::ad::AAttrId>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>", no_default)]
    resist_attr_id: Option<rc::ad::AAttrId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CEffect {
    type AEntity = rc::ad::AEffect;

    fn from_adapted(a_effect: &Self::AEntity) -> Self {
        Self {
            id: a_effect.id,
            category: a_effect.category.into_i32(),
            state: CState::from_adapted(&a_effect.state),
            modifiers: a_effect.modifiers.iter().map(CEffectModifier::from_adapted).collect(),
            stopped_effect_ids: a_effect.stopped_effect_ids.iter().copied().collect(),
            buff: a_effect.buff.as_ref().map(CEffectBuff::from_adapted),
            aggro: a_effect.aggro.as_ref().map(CEffectAggroDuration::from_adapted),
            is_assist: a_effect.is_assist,
            is_offense: a_effect.is_offense,
            banned_in_hisec: a_effect.banned_in_hisec,
            banned_in_lowsec: a_effect.banned_in_lowsec,
            discharge_attr_id: a_effect.discharge_attr_id,
            duration_attr_id: a_effect.duration_attr_id,
            range_attr_id: a_effect.range_attr_id,
            falloff_attr_id: a_effect.falloff_attr_id,
            track_attr_id: a_effect.track_attr_id,
            chance_attr_id: a_effect.chance_attr_id,
            resist_attr_id: a_effect.resist_attr_id,
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        Self::AEntity {
            id: self.id,
            category: rc::ad::AEffectCatId::from_i32(self.category),
            state: self.state.into_adapted(),
            modifiers: self.modifiers.into_iter().map(|c_mod| c_mod.into_adapted()).collect(),
            stopped_effect_ids: self.stopped_effect_ids.into_iter().collect(),
            buff: self.buff.map(|c_buff| c_buff.into_adapted()),
            aggro: self.aggro.map(|c_aggro| c_aggro.into_adapted()),
            is_assist: self.is_assist,
            is_offense: self.is_offense,
            banned_in_hisec: self.banned_in_hisec,
            banned_in_lowsec: self.banned_in_lowsec,
            discharge_attr_id: self.discharge_attr_id,
            duration_attr_id: self.duration_attr_id,
            range_attr_id: self.range_attr_id,
            falloff_attr_id: self.falloff_attr_id,
            track_attr_id: self.track_attr_id,
            chance_attr_id: self.chance_attr_id,
            resist_attr_id: self.resist_attr_id,
        }
    }
}
