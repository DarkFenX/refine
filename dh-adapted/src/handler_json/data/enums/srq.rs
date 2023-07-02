#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModSrq {
    SelfRef,
    ItemId(rc::EItemId),
}
impl From<&rc::ad::ModSrq> for CModSrq {
    fn from(mod_srq: &rc::ad::ModSrq) -> Self {
        match mod_srq {
            rc::ad::ModSrq::SelfRef => Self::SelfRef,
            rc::ad::ModSrq::ItemId(item_id) => Self::ItemId(*item_id),
        }
    }
}
impl Into<rc::ad::ModSrq> for &CModSrq {
    fn into(self) -> rc::ad::ModSrq {
        match self {
            CModSrq::SelfRef => rc::ad::ModSrq::SelfRef,
            CModSrq::ItemId(item_id) => rc::ad::ModSrq::ItemId(*item_id),
        }
    }
}
