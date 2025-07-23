use crate::{ad, util::Named};

pub(crate) struct RAttr {
    a_attr: ad::AAttr,
}
impl RAttr {
    pub(crate) fn new(a_attr: ad::AAttr) -> Self {
        Self { a_attr, .. }
    }
    pub(crate) fn is_penalizable(&self) -> bool {
        self.a_attr.penalizable
    }
    pub(crate) fn is_hig(&self) -> bool {
        self.a_attr.hig
    }
    pub(crate) fn get_min_attr_id(&self) -> Option<ad::AAttrId> {
        self.a_attr.min_attr_id
    }
    pub(crate) fn get_max_attr_id(&self) -> Option<ad::AAttrId> {
        self.a_attr.max_attr_id
    }
}
impl Named for RAttr {
    fn get_name() -> &'static str {
        "RAttr"
    }
}
