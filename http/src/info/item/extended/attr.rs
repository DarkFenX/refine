use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(in crate::info::item::extended) struct HAttrVals {
    base: f64,
    modified: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HAttrVals {
    pub(in crate::info::item::extended) fn from_core(core_attr_vals: rc::AttrVals) -> Self {
        Self {
            base: core_attr_vals.base.into_f64(),
            modified: core_attr_vals.modified.into_f64(),
        }
    }
}
