use crate::{OptionalReload, RearmMinion, svc::cycle::CyclingOptions};

impl CyclingOptions {
    pub(in crate::svc::cycle::item_getters) fn cache_key(self) -> u8 {
        // One value per distinct combination of options
        fn reload_code(reload: Option<OptionalReload>) -> u8 {
            match reload {
                None => 0,
                Some(OptionalReload::Disabled) => 1,
                Some(OptionalReload::OnEmpty) => 2,
            }
        }
        fn rearm_code(rearm: Option<RearmMinion>) -> u8 {
            match rearm {
                None => 0,
                Some(RearmMinion::Disabled) => 1,
                Some(RearmMinion::OnFirstEmpty) => 2,
            }
        }
        match self {
            Self::Burst => 0,
            Self::Sim(sim) => 1 + 3 * reload_code(sim.optional_reloads) + rearm_code(sim.rearm_minions),
        }
    }
}
