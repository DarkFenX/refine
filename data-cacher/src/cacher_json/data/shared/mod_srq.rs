#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cacher_json::data) enum CModifierSrq {
    SelfRef,
    ItemId(i32),
}
impl CModifierSrq {
    pub(in crate::cacher_json::data) fn from_adapted(a_modifier_srq: &rc::ad::AModifierSrq) -> Self {
        match a_modifier_srq {
            rc::ad::AModifierSrq::SelfRef => Self::SelfRef,
            rc::ad::AModifierSrq::ItemId(item_id) => Self::ItemId(item_id.into_i32()),
        }
    }
    pub(in crate::cacher_json::data) fn into_adapted(self) -> rc::ad::AModifierSrq {
        match self {
            Self::SelfRef => rc::ad::AModifierSrq::SelfRef,
            Self::ItemId(item_id) => rc::ad::AModifierSrq::ItemId(rc::ad::AItemId::from_i32(item_id)),
        }
    }
}
