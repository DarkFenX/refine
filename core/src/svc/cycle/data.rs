use crate::{def::AttrVal, util::sig_round};

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleDataFull {
    pub(in crate::svc) time: AttrVal,
    pub(in crate::svc) interrupt: bool,
    pub(in crate::svc) charged: Option<AttrVal>,
}
impl CycleDataFull {
    pub(super) fn new(time: AttrVal, interrupt: bool, charged: Option<AttrVal>) -> Self {
        Self {
            time,
            interrupt,
            charged,
        }
    }
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            time: sig_round(self.time, 10),
            interrupt: self.interrupt,
            charged: self.charged.map(|v| sig_round(v, 10)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleDataTime {
    pub(in crate::svc) time: AttrVal,
}
impl From<&CycleDataFull> for CycleDataTime {
    fn from(full: &CycleDataFull) -> Self {
        Self { time: full.time }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::svc) struct CycleDataTimeCharged {
    pub(in crate::svc) time: AttrVal,
    pub(in crate::svc) charged: Option<AttrVal>,
}
impl From<&CycleDataFull> for CycleDataTimeCharged {
    fn from(full: &CycleDataFull) -> Self {
        Self {
            time: full.time,
            charged: full.charged,
        }
    }
}
