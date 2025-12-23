#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::item::extended) struct HAttrVals {
    base: rc::AttrVal,
    modified: rc::AttrVal,
}
impl From<&rc::AttrVals> for HAttrVals {
    fn from(core_attr_vals: &rc::AttrVals) -> Self {
        Self {
            base: core_attr_vals.base,
            modified: core_attr_vals.modified,
        }
    }
}
