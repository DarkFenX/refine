use crate::misc::{Count, InfCount, UnitInterval};

pub(in crate::svc::cycle) struct EffectChargeInfo {
    pub(in crate::svc::cycle) fully_charged: InfCount,
    pub(in crate::svc::cycle) part_charged: Option<UnitInterval>,
    pub(in crate::svc::cycle) can_run_uncharged: bool,
}
impl EffectChargeInfo {
    pub(in crate::svc::cycle) fn is_unrunnable(&self) -> bool {
        matches!(self.fully_charged, InfCount::Count(Count::ZERO))
            && self.part_charged.is_none()
            && !self.can_run_uncharged
    }
    pub(in crate::svc::cycle) fn get_first_cycle_chargedness(&self) -> Option<UnitInterval> {
        match self.fully_charged {
            InfCount::Count(count) if count > Count::ZERO => Some(UnitInterval::ONE),
            InfCount::Infinite => Some(UnitInterval::ONE),
            _ => self.part_charged,
        }
    }
}
