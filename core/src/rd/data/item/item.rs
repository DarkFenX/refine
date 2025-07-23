use crate::{ad, util::Named};

pub(crate) struct RItem {
    a_item: ad::AItem,
}
impl RItem {
    pub(crate) fn new(a_item: ad::AItem) -> Self {
        Self { a_item }
    }
}
impl Named for RItem {
    fn get_name() -> &'static str {
        "RItem"
    }
}
