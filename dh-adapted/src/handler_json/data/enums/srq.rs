#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModSrq {
    SelfRef,
    ItemId(rc::EItemId),
}
impl From<&rc::consts::ModSrq> for CModSrq {
    fn from(mod_srq: &rc::consts::ModSrq) -> Self {
        match mod_srq {
            rc::consts::ModSrq::SelfRef => Self::SelfRef,
            rc::consts::ModSrq::ItemId(item_id) => Self::ItemId(*item_id),
        }
    }
}
impl Into<rc::consts::ModSrq> for &CModSrq {
    fn into(self) -> rc::consts::ModSrq {
        match self {
            CModSrq::SelfRef => rc::consts::ModSrq::SelfRef,
            CModSrq::ItemId(item_id) => rc::consts::ModSrq::ItemId(*item_id),
        }
    }
}
