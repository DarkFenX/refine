use crate::num::{PValue, UnitInterval};

pub(in crate::svc) trait GetDuration {
    fn get_duration(&self) -> PValue;
}

pub(in crate::svc) trait GetMainDuration {
    // Active duration and soft downtime duration combined
    fn get_main_duration(&self) -> PValue;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle data containers
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleDataFull {
    pub(in crate::svc) active: CycleActive,
    pub(in crate::svc) soft_dt: Option<CycleSoftDtFull>,
}
impl GetMainDuration for CycleDataFull {
    fn get_main_duration(&self) -> PValue {
        let mut duration = self.active.duration;
        if let Some(soft_dt) = &self.soft_dt {
            duration += soft_dt.duration;
        }
        duration
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Detail fields
////////////////////////////////////////////////////////////////////////////////////////////////////
// Data about active part of cycle
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleActive {
    // Duration of effect cycle
    pub(in crate::svc) duration: PValue,
    // How charged cycle is
    pub(in crate::svc) chargedness: Option<UnitInterval>,
}

// Info about soft downtime between cycles (during which effects can apply their instances)
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleSoftDtFull {
    pub(in crate::svc) duration: PValue,
    pub(in crate::svc) reasons: CycleSoftDtReasons,
}
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleSoftDtReasons {
    // There are actually many more reasons for soft downtime to be created, but since they were not
    // used by anything, they were removed. Full list of reasons:
    // - non-repeating: used for effects which cannot be auto-repeated (some modules like titan DDs, fighter abilities
    //   limited by charge count);
    // - reactivation delay: used on modules when repeating cycle sequence is broken;
    // - reload: module reloads only; fighter rearms are considered as a hard downtime;
    // - cooldown: fighter ability cooldowns which happen after every cycle;
    // - pre-rearm idle: when there is some time, but too little to fit even partial cycle before fighter rearm, that
    //   time is considered as pre-rearm-idling.
    pub(in crate::svc) reload: bool,
}

// Info about cycle sequence hard downtime (during which effects cannot apply their instances)
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqHardDtFull {
    pub(in crate::svc) duration: PValue,
    pub(in crate::svc) reasons: CSeqHardDtReasons,
}
impl CSeqHardDtFull {
    pub(super) fn try_new(duration: PValue, rearm: bool) -> Option<Self> {
        let reasons = CSeqHardDtReasons::try_new(rearm)?;
        Some(Self { duration, reasons })
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CSeqHardDtReasons {
    pub(in crate::svc) refuel: bool,
}
impl CSeqHardDtReasons {
    fn try_new(refuel: bool) -> Option<Self> {
        match refuel {
            true => Some(Self { refuel }),
            false => None,
        }
    }
}
