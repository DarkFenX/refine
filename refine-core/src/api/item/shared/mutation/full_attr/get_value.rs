use crate::{FullMAttr, Value};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FullMAttr<'s> {
    /// Return mutated attribute value.
    pub fn get_value(&self) -> Value {
        let attr_aid = &self.attr_aid;
        let attr_rid = self.core.u_data.r_data.get_attr_rid_by_aid(attr_aid).unwrap();
        *self
            .core
            .u_data
            .items
            .get(self.item_uid)
            .get_r_item_flex_data()
            .unwrap()
            .attrs
            .get(&attr_rid)
            .unwrap()
    }
}
