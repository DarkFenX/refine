#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(super) struct CEffectBuffInfo {
    attr_merge: Option<CEffectBuffAttrMerge>,
    full: Vec<CEffectBuffFull>,
}
impl CEffectBuffInfo {
    pub(super) fn from_adapted(a_effect_buff: &rc::ad::AEffectBuff) -> Self {
        Self {
            attr_merge: a_effect_buff
                .attr_merge
                .as_ref()
                .map(|a_buff_merge| CEffectBuffAttrMerge::from_adapted(a_buff_merge)),
            full: a_effect_buff
                .full
                .iter()
                .map(|a_buff_full| CEffectBuffFull::from_adapted(a_buff_full))
                .collect(),
        }
    }
    pub(super) fn into_adapted(self) -> rc::ad::AEffectBuff {
        rc::ad::AEffectBuff {
            attr_merge: self.attr_merge.map(|c_buff_merge| c_buff_merge.into_adapted()),
            full: self
                .full
                .into_iter()
                .map(|c_buff_full| c_buff_full.into_adapted())
                .collect(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct CEffectBuffAttrMerge {
    duration: CEffectBuffDuration,
    scope: CEffectBuffScope,
}
impl CEffectBuffAttrMerge {
    fn from_adapted(a_buff_attr_merge: &rc::ad::AEffectBuffAttrMerge) -> Self {
        Self {
            duration: CEffectBuffDuration::from_adapted(&a_buff_attr_merge.duration),
            scope: CEffectBuffScope::from_adapted(&a_buff_attr_merge.scope),
        }
    }
    fn into_adapted(self) -> rc::ad::AEffectBuffAttrMerge {
        rc::ad::AEffectBuffAttrMerge {
            duration: self.duration.into_adapted(),
            scope: self.scope.into_adapted(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct CEffectBuffFull {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    buff_id: rc::ad::ABuffId,
    strength: CEffectBuffStrength,
    duration: CEffectBuffDuration,
    scope: CEffectBuffScope,
}
impl CEffectBuffFull {
    fn from_adapted(a_buff_full: &rc::ad::AEffectBuffFull) -> Self {
        Self {
            buff_id: a_buff_full.buff_id,
            strength: CEffectBuffStrength::from_adapted(&a_buff_full.strength),
            duration: CEffectBuffDuration::from_adapted(&a_buff_full.duration),
            scope: CEffectBuffScope::from_adapted(&a_buff_full.scope),
        }
    }
    fn into_adapted(self) -> rc::ad::AEffectBuffFull {
        rc::ad::AEffectBuffFull {
            buff_id: self.buff_id,
            strength: self.strength.into_adapted(),
            duration: self.duration.into_adapted(),
            scope: self.scope.into_adapted(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum CEffectBuffStrength {
    Attr(#[serde_as(as = "serde_with::DisplayFromStr")] rc::ad::AAttrId),
    Hardcoded(f64),
}
impl CEffectBuffStrength {
    fn from_adapted(a_buff_str: &rc::ad::AEffectBuffStrength) -> Self {
        match a_buff_str {
            rc::ad::AEffectBuffStrength::Attr(attr_id) => Self::Attr(*attr_id),
            rc::ad::AEffectBuffStrength::Hardcoded(buff_val) => Self::Hardcoded(buff_val.into_f64()),
        }
    }
    fn into_adapted(self) -> rc::ad::AEffectBuffStrength {
        match self {
            Self::Attr(attr_id) => rc::ad::AEffectBuffStrength::Attr(attr_id),
            Self::Hardcoded(buff_val) => rc::ad::AEffectBuffStrength::Hardcoded(rc::ad::AValue::from_f64(buff_val)),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum CEffectBuffDuration {
    None,
    AttrMs(#[serde_as(as = "serde_with::DisplayFromStr")] rc::ad::AAttrId),
}
impl CEffectBuffDuration {
    fn from_adapted(a_buff_duration: &rc::ad::AEffectBuffDuration) -> Self {
        match a_buff_duration {
            rc::ad::AEffectBuffDuration::None => Self::None,
            rc::ad::AEffectBuffDuration::AttrMs(attr_id) => Self::AttrMs(*attr_id),
        }
    }
    fn into_adapted(self) -> rc::ad::AEffectBuffDuration {
        match self {
            Self::None => rc::ad::AEffectBuffDuration::None,
            Self::AttrMs(attr_id) => rc::ad::AEffectBuffDuration::AttrMs(attr_id),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum CEffectBuffScope {
    Carrier,
    Projected(#[serde_as(as = "serde_with::DisplayFromStr")] rc::ad::AItemListId),
    Fleet(#[serde_as(as = "serde_with::DisplayFromStr")] rc::ad::AItemListId),
}
impl CEffectBuffScope {
    fn from_adapted(a_buff_scope: &rc::ad::AEffectBuffScope) -> Self {
        match a_buff_scope {
            rc::ad::AEffectBuffScope::Carrier => Self::Carrier,
            rc::ad::AEffectBuffScope::Projected(a_item_list_id) => Self::Projected(*a_item_list_id),
            rc::ad::AEffectBuffScope::Fleet(a_item_list_id) => Self::Fleet(*a_item_list_id),
        }
    }
    fn into_adapted(self) -> rc::ad::AEffectBuffScope {
        match self {
            Self::Carrier => rc::ad::AEffectBuffScope::Carrier,
            Self::Projected(c_item_list_id) => rc::ad::AEffectBuffScope::Projected(c_item_list_id),
            Self::Fleet(c_item_list_id) => rc::ad::AEffectBuffScope::Fleet(c_item_list_id),
        }
    }
}
