use crate::{
    OptionalReload, RearmMinion,
    stats::{StatTimeOptions, StatTimeOptionsSim},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// "Public"
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::svc) enum CyclingOptions {
    Burst,
    Sim(CycleOptionsSim),
}

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleOptionsSim {
    /// Controls if effects which can run with/without charges (e.g. ancillary reps) are forced to
    /// reload once they run out of charges
    pub(in crate::svc) optional_reloads: Option<OptionalReload> = None,
    /// Controls if depleted fighter abilities force fighter recall, refuel and rearm
    pub(in crate::svc) rearm_minions: Option<RearmMinion> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item-kind-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Cycling options module and charge cycle builders read.
#[derive(Copy, Clone)]
pub(super) enum ModuleCyclingOptions {
    Burst,
    Sim { optional_reloads: Option<OptionalReload> },
}

/// Cycling options fighter and autocharge cycle builders read.
#[derive(Copy, Clone)]
pub(super) enum FighterCyclingOptions {
    Burst,
    Sim { rearm_minions: Option<RearmMinion> },
}

/// Cycling options drone cycle builders read: none.
#[derive(Copy, Clone)]
pub(super) struct DroneCyclingOptions;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CyclingOptions {
    pub(in crate::svc) fn from_time_options(time_options: StatTimeOptions) -> Self {
        match time_options {
            StatTimeOptions::Burst(..) => Self::Burst,
            StatTimeOptions::Sim(inner) => Self::Sim(CycleOptionsSim::from_time_options_sim(inner)),
        }
    }
}

impl CycleOptionsSim {
    fn from_time_options_sim(time_options_sim: StatTimeOptionsSim) -> Self {
        Self {
            optional_reloads: time_options_sim.optional_reloads,
            rearm_minions: time_options_sim.rearm_minions,
        }
    }
}

impl ModuleCyclingOptions {
    pub(super) fn from_cycling_options(options: CyclingOptions) -> Self {
        match options {
            CyclingOptions::Burst => Self::Burst,
            CyclingOptions::Sim(CycleOptionsSim {
                optional_reloads,
                rearm_minions: _,
            }) => Self::Sim { optional_reloads },
        }
    }
}

impl FighterCyclingOptions {
    pub(super) fn from_cycling_options(options: CyclingOptions) -> Self {
        match options {
            CyclingOptions::Burst => Self::Burst,
            CyclingOptions::Sim(CycleOptionsSim {
                optional_reloads: _,
                rearm_minions,
            }) => Self::Sim { rearm_minions },
        }
    }
}

impl DroneCyclingOptions {
    pub(super) fn from_cycling_options(options: CyclingOptions) -> Self {
        match options {
            CyclingOptions::Burst
            | CyclingOptions::Sim(CycleOptionsSim {
                optional_reloads: _,
                rearm_minions: _,
            }) => Self,
        }
    }
}
