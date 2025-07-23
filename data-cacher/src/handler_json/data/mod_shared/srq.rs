use crate::handler_json::data::CItemId;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CModifierSrq {
    SelfRef,
    ItemId(CItemId),
}
impl From<&rc::ad::AModifierSrq> for CModifierSrq {
    fn from(a_modidier_srq: &rc::ad::AModifierSrq) -> Self {
        match a_modidier_srq {
            rc::ad::AModifierSrq::SelfRef => Self::SelfRef,
            rc::ad::AModifierSrq::ItemId(item_id) => Self::ItemId(*item_id),
        }
    }
}
impl From<&CModifierSrq> for rc::ad::AModifierSrq {
    fn from(c_modidier_srq: &CModifierSrq) -> Self {
        match c_modidier_srq {
            CModifierSrq::SelfRef => Self::SelfRef,
            CModifierSrq::ItemId(item_id) => Self::ItemId(*item_id),
        }
    }
}
