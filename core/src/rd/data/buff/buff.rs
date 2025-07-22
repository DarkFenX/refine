use crate::{ad, rd::RBuffModifier, util::Named};

pub(crate) struct RBuff {
    a_buff: ad::ABuff,
    mods: Vec<RBuffModifier>,
}
impl RBuff {
    pub(crate) fn new(a_buff: ad::ABuff) -> Self {
        Self {
            a_buff,
            mods: Vec::new(),
        }
    }
    pub(crate) fn get_aggr_mode(&self) -> ad::ABuffAggrMode {
        self.a_buff.aggr_mode
    }
    pub(crate) fn get_op(&self) -> ad::AOp {
        self.a_buff.op
    }
}
impl Named for RBuff {
    fn get_name() -> &'static str {
        "RBuff"
    }
}
