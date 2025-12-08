use crate::{def::AttrVal, util::sig_round};

pub(super) type TickCount = usize;
pub(super) const TICK_LIMIT: TickCount = 500;

pub(super) fn rah_round(val: AttrVal) -> AttrVal {
    sig_round(val, 10)
}
