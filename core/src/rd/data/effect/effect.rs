use crate::{ad, util::Named};

pub(crate) struct REffect {
    a_effect: ad::AEffect,
}
impl REffect {
    pub(crate) fn new(a_effect: ad::AEffect) -> Self {
        Self { a_effect }
    }
}
impl Named for REffect {
    fn get_name() -> &'static str {
        "REffect"
    }
}
