use crate::{ad, util::Named};

pub(crate) struct RBuff {
    a_buff: ad::ABuff,
}
impl RBuff {
    pub(crate) fn new(a_buff: ad::ABuff) -> Self {
        Self { a_buff }
    }
    pub(crate) fn get_aggr_mode(&self) -> ad::ABuffAggrMode {
        self.a_buff.aggr_mode
    }
    pub(crate) fn get_op(&self) -> ad::AOp {
        self.a_buff.op
    }
    pub(crate) fn get_modifiers(&self) -> &Vec<ad::ABuffModifier> {
        &self.a_buff.mods
    }
}
impl Named for RBuff {
    fn get_name() -> &'static str {
        "RBuff"
    }
}
