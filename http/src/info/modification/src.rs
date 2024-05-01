#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub struct HModSrcInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub item_id: rc::SolItemId,
    pub val: HModSrcValInfo,
}
impl HModSrcInfo {
    fn new(item_id: rc::SolItemId, val: HModSrcValInfo) -> Self {
        Self { item_id, val }
    }
}
impl From<&rc::SolAffectorInfo> for HModSrcInfo {
    fn from(core_src: &rc::SolAffectorInfo) -> Self {
        Self::new(core_src.item_id, (&core_src.val).into())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HModSrcValInfo {
    #[serde(rename = "attr")]
    AttrId(rc::EAttrId),
    #[serde(rename = "hc")]
    Hardcoded(rc::AttrVal),
}
impl From<&rc::SolAffectorValueInfo> for HModSrcValInfo {
    fn from(core_src_val: &rc::SolAffectorValueInfo) -> Self {
        match core_src_val {
            rc::SolAffectorValueInfo::AttrId(attr_id) => Self::AttrId(*attr_id),
            rc::SolAffectorValueInfo::Hardcoded(attr_val) => Self::Hardcoded(*attr_val),
        }
    }
}
