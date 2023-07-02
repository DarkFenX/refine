#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModSrq {
    SelfRef,
    ItemId(rc::EItemId),
}
impl From<&rc::ad::AModSrq> for CModSrq {
    fn from(mod_srq: &rc::ad::AModSrq) -> Self {
        match mod_srq {
            rc::ad::AModSrq::SelfRef => Self::SelfRef,
            rc::ad::AModSrq::ItemId(item_id) => Self::ItemId(*item_id),
        }
    }
}
impl Into<rc::ad::AModSrq> for &CModSrq {
    fn into(self) -> rc::ad::AModSrq {
        match self {
            CModSrq::SelfRef => rc::ad::AModSrq::SelfRef,
            CModSrq::ItemId(item_id) => rc::ad::AModSrq::ItemId(*item_id),
        }
    }
}
