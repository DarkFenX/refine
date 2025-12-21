use crate::{
    def::{AttrVal, OF},
    util::InfCount,
};

pub(in crate::svc::cycle) struct EffectChargeInfo {
    pub(in crate::svc::cycle) fully_charged: InfCount,
    pub(in crate::svc::cycle) part_charged: Option<AttrVal>,
    pub(in crate::svc::cycle) can_run_uncharged: bool,
}
impl EffectChargeInfo {
    pub(in crate::svc::cycle) fn is_unrunnable(&self) -> bool {
        matches!(self.fully_charged, InfCount::Count(0)) && self.part_charged.is_none() && !self.can_run_uncharged
    }
    pub(in crate::svc::cycle) fn get_first_cycle_chargeness(&self) -> Option<AttrVal> {
        match self.fully_charged {
            InfCount::Count(count) if count > 0 => Some(OF(1.0)),
            InfCount::Infinite => Some(OF(1.0)),
            _ => self.part_charged,
        }
    }
}
