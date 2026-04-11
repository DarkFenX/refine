use crate::{
    misc::{OptionalReload, RearmMinion},
    num::PValue,
    rd::REffectId,
    svc::vast::{StatTimeOptions, StatTimeOptionsSim},
};

#[derive(Copy, Clone)]
pub(in crate::svc) enum CyclingOptions {
    Burst,
    Sim(CycleOptionsSim),
}

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleOptionsSim {
    // Controls if effects which can run with/without charges (e.g. ancillary reps) are forced to
    // reload once they run out of charges
    pub(in crate::svc) optional_reloads: Option<OptionalReload> = None,
    // Controls if depleted fighter abilities force fighter recall, refuel and rearm
    pub(in crate::svc) rearm_minions: Option<RearmMinion> = None,
}

pub(super) struct SelfKillerEffect {
    pub(super) effect_rid: REffectId,
    pub(super) duration: PValue,
}

pub(super) struct SelfKillerItem {
    effect: Option<SelfKillerEffect>,
}
impl SelfKillerItem {
    pub(super) fn new() -> Self {
        Self { effect: None }
    }
    pub(super) fn push(&mut self, effect: SelfKillerEffect) {
        match &self.effect {
            Some(stored) => {
                if effect.duration < stored.duration {
                    self.effect = Some(effect);
                }
            }
            None => self.effect = Some(effect),
        }
    }
    pub(super) fn get_effect_rid(&self) -> Option<REffectId> {
        self.effect.as_ref().map(|v| v.effect_rid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CyclingOptions {
    pub(in crate::svc) fn from_time_options(time_options: StatTimeOptions) -> Self {
        match time_options {
            StatTimeOptions::Burst(_) => Self::Burst,
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
