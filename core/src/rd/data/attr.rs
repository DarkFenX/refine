use crate::{
    ad,
    util::{GetId, Named},
};

// Represents a dogma attribute.
//
// An attribute carries just properties which govern how modified attribute values are calculated.
// Values themselves are stored elsewhere as plain numbers.
pub(crate) struct RAttr {
    a_attr: ad::AAttr,
}
impl RAttr {
    pub(crate) fn new(a_attr: ad::AAttr) -> Self {
        Self { a_attr }
    }
    // Defines if modifications applied to the attribute's values are immune to stacking penalties
    // or not.
    pub(crate) fn is_penalizable(&self) -> bool {
        self.a_attr.penalizable
    }
    // "High is good" defines if higher value of the attribute is considered good or not.
    pub(crate) fn is_hig(&self) -> bool {
        self.a_attr.hig
    }
    // Default value of the attribute, used if not provided by an item type.
    pub(crate) fn get_def_val(&self) -> ad::AAttrVal {
        self.a_attr.def_val
    }
    // Refers another attribute, whose value limits minimum value of this attribute.
    pub(crate) fn get_min_attr_id(&self) -> Option<ad::AAttrId> {
        self.a_attr.min_attr_id
    }
    // Refers another attribute, whose value limits maximum value of this attribute.
    pub(crate) fn get_max_attr_id(&self) -> Option<ad::AAttrId> {
        self.a_attr.max_attr_id
    }
}
impl GetId<ad::AAttrId> for RAttr {
    fn get_id(&self) -> ad::AAttrId {
        self.a_attr.id
    }
}
impl Named for RAttr {
    fn get_name() -> &'static str {
        "RAttr"
    }
}
