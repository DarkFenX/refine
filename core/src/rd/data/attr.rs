use crate::{ad, rd::RAttrKey, util::Named};

pub(crate) struct RAttr {
    a_attr: ad::AAttr,
    min_attr_key: Option<RAttrKey> = None,
    max_attr_key: Option<RAttrKey> = None,
}
impl RAttr {
    pub(crate) fn new(a_attr: ad::AAttr) -> Self {
        Self { a_attr, .. }
    }
    fn fill_r_keys(&mut self) {
        // TODO: add actual contents which fill min/max keys
    }
    pub(crate) fn is_penalizable(&self) -> bool {
        self.a_attr.penalizable
    }
    pub(crate) fn is_hig(&self) -> bool {
        self.a_attr.hig
    }
    pub(crate) fn get_min_attr_key(&self) -> Option<RAttrKey> {
        self.min_attr_key
    }
    pub(crate) fn get_max_attr_key(&self) -> Option<RAttrKey> {
        self.max_attr_key
    }
}
impl Named for RAttr {
    fn get_name() -> &'static str {
        "RAttr"
    }
}
