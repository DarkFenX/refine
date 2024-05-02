#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModSrq {
    SelfRef,
    ItemId(rc::EItemId),
}
impl From<&rc::ad::AModifierSrq> for CModSrq {
    fn from(mod_srq: &rc::ad::AModifierSrq) -> Self {
        match mod_srq {
            rc::ad::AModifierSrq::SelfRef => Self::SelfRef,
            rc::ad::AModifierSrq::ItemId(item_id) => Self::ItemId(*item_id),
        }
    }
}
impl Into<rc::ad::AModifierSrq> for &CModSrq {
    fn into(self) -> rc::ad::AModifierSrq {
        match self {
            CModSrq::SelfRef => rc::ad::AModifierSrq::SelfRef,
            CModSrq::ItemId(item_id) => rc::ad::AModifierSrq::ItemId(*item_id),
        }
    }
}
