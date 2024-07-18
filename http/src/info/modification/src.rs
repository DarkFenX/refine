#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub struct HModSrcInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub item_id: rc::SolItemId,
    pub attr_id: Option<rc::EAttrId>,
}
impl HModSrcInfo {
    fn new(item_id: rc::SolItemId, attr_id: Option<rc::EAttrId>) -> Self {
        Self { item_id, attr_id }
    }
}
impl From<&rc::SolAffectorInfo> for HModSrcInfo {
    fn from(core_src: &rc::SolAffectorInfo) -> Self {
        match core_src.val {
            rc::SolAffectorValueInfo::AttrId(attr_id) => Self::new(core_src.item_id, Some(attr_id)),
            rc::SolAffectorValueInfo::Hardcoded(_) => Self::new(core_src.item_id, None),
        }
    }
}
