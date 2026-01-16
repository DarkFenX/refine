use crate::{misc::Spool, num::PValue};

#[derive(Copy, Clone)]
pub enum StatTimeOptions {
    /// Burst value of parameter is considered, with ability to set some overrides.
    Burst(StatTimeOptionsBurst),
    /// Full cycling simulation happens, and the result returned is total or average value over that
    /// duration, depending on context.
    Sim(StatTimeOptionsSim),
}

#[derive(Copy, Clone)]
pub struct StatTimeOptionsBurst {
    /// Set spool parameters override in the stats request. If not set, uses on-item value or
    /// default value set on solar system.
    pub spool: Option<Spool> = None,
}

#[derive(Copy, Clone)]
pub struct StatTimeOptionsSim {
    /// Time over which period stats will be considered. If not set or invalid, fetches stats over
    /// infinite period of time.
    pub time: Option<PValue> = None,
    /// Are modules which can run uncharged reloaded upon depleting their charges.
    pub reload_optionals: Option<bool> = None,
    /// Are fighters sent into refueling/rearming once one of their charge-limited abilities runs
    /// out.
    pub rearm_minions: Option<bool> = None,
}
