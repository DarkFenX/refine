#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub struct HModSrcInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub item_id: rc::SsItemId,
    pub val: HModSrcValInfo,
}
impl HModSrcInfo {
    fn new(item_id: rc::SsItemId, val: HModSrcValInfo) -> Self {
        Self { item_id, val }
    }
}
impl From<&rc::SsModSrcInfo> for HModSrcInfo {
    fn from(core_src: &rc::SsModSrcInfo) -> Self {
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
impl From<&rc::SsModSrcValInfo> for HModSrcValInfo {
    fn from(core_src_val: &rc::SsModSrcValInfo) -> Self {
        match core_src_val {
            rc::SsModSrcValInfo::AttrId(attr_id) => Self::AttrId(*attr_id),
            rc::SsModSrcValInfo::Hardcoded(attr_val) => Self::Hardcoded(*attr_val),
        }
    }
}
