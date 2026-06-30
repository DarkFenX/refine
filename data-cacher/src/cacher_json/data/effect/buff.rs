use super::strength::CEffectModStrength;
use crate::cacher_json::data::AdaptedConv;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(super) struct CEffectBuff {
    attr_merge: Option<CEffectBuffAttrMerge>,
    full: Vec<CEffectBuffFull>,
}

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct CEffectBuffAttrMerge {
    duration: CEffectBuffDuration,
    scope: CEffectBuffScope,
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct CEffectBuffFull {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    buff_id: rc::ad::ABuffId,
    strength: CEffectModStrength,
    duration: CEffectBuffDuration,
    scope: CEffectBuffScope,
}

#[serde_with::serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum CEffectBuffDuration {
    Effect,
    AttrS(#[serde_as(as = "serde_with::DisplayFromStr")] rc::ad::AAttrId),
    AttrMs(#[serde_as(as = "serde_with::DisplayFromStr")] rc::ad::AAttrId),
}

#[serde_with::serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum CEffectBuffScope {
    Carrier,
    Projected(#[serde_as(as = "serde_with::DisplayFromStr")] rc::ad::AItemListId),
    Fleet(#[serde_as(as = "serde_with::DisplayFromStr")] rc::ad::AItemListId),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CEffectBuff {
    type AEntity = rc::ad::AEffectBuff;

    fn from_adapted(a_effect_buff: &Self::AEntity) -> Self {
        Self {
            attr_merge: a_effect_buff
                .attr_merge
                .as_ref()
                .map(CEffectBuffAttrMerge::from_adapted),
            full: a_effect_buff.full.iter().map(CEffectBuffFull::from_adapted).collect(),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        Self::AEntity {
            attr_merge: self.attr_merge.map(|c_buff_merge| c_buff_merge.into_adapted()),
            full: self
                .full
                .into_iter()
                .map(|c_buff_full| c_buff_full.into_adapted())
                .collect(),
        }
    }
}

impl AdaptedConv for CEffectBuffAttrMerge {
    type AEntity = rc::ad::AEffectBuffAttrMerge;

    fn from_adapted(a_buff_attr_merge: &Self::AEntity) -> Self {
        Self {
            duration: CEffectBuffDuration::from_adapted(&a_buff_attr_merge.duration),
            scope: CEffectBuffScope::from_adapted(&a_buff_attr_merge.scope),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        Self::AEntity {
            duration: self.duration.into_adapted(),
            scope: self.scope.into_adapted(),
        }
    }
}

impl AdaptedConv for CEffectBuffFull {
    type AEntity = rc::ad::AEffectBuffFull;

    fn from_adapted(a_buff_full: &Self::AEntity) -> Self {
        Self {
            buff_id: a_buff_full.buff_id,
            strength: CEffectModStrength::from_adapted(&a_buff_full.strength),
            duration: CEffectBuffDuration::from_adapted(&a_buff_full.duration),
            scope: CEffectBuffScope::from_adapted(&a_buff_full.scope),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        Self::AEntity {
            buff_id: self.buff_id,
            strength: self.strength.into_adapted(),
            duration: self.duration.into_adapted(),
            scope: self.scope.into_adapted(),
        }
    }
}

impl AdaptedConv for CEffectBuffDuration {
    type AEntity = rc::ad::AEffectBuffDuration;

    fn from_adapted(a_buff_duration: &Self::AEntity) -> Self {
        match a_buff_duration {
            Self::AEntity::Effect => Self::Effect,
            Self::AEntity::AttrS(attr_id) => Self::AttrS(*attr_id),
            Self::AEntity::AttrMs(attr_id) => Self::AttrMs(*attr_id),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        match self {
            Self::Effect => Self::AEntity::Effect,
            Self::AttrS(attr_id) => Self::AEntity::AttrS(attr_id),
            Self::AttrMs(attr_id) => Self::AEntity::AttrMs(attr_id),
        }
    }
}

impl AdaptedConv for CEffectBuffScope {
    type AEntity = rc::ad::AEffectBuffScope;

    fn from_adapted(a_buff_scope: &Self::AEntity) -> Self {
        match a_buff_scope {
            Self::AEntity::Carrier => Self::Carrier,
            Self::AEntity::Projected(a_item_list_id) => Self::Projected(*a_item_list_id),
            Self::AEntity::Fleet(a_item_list_id) => Self::Fleet(*a_item_list_id),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        match self {
            Self::Carrier => Self::AEntity::Carrier,
            Self::Projected(c_item_list_id) => Self::AEntity::Projected(c_item_list_id),
            Self::Fleet(c_item_list_id) => Self::AEntity::Fleet(c_item_list_id),
        }
    }
}
