use serde::Deserialize;

use crate::shared::HSpool;

#[derive(Copy, Clone, educe::Educe, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
#[educe(Default)]
pub(in crate::cmd) enum HStatTimeOptions {
    #[educe(Default)]
    Burst(HStatTimeOptionsBurst),
    Sim(HStatTimeOptionsSim),
}

#[derive(Copy, Clone, Default, Deserialize)]
pub(in crate::cmd) struct HStatTimeOptionsBurst {
    #[serde(default)]
    pub(in crate::cmd) spool: Option<HSpool>,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub(in crate::cmd) struct HStatTimeOptionsSim {
    #[serde(default)]
    pub(in crate::cmd) time: Option<f64>,
    #[serde(default)]
    pub(in crate::cmd) reload_optionals: Option<bool>,
    #[serde(default)]
    pub(in crate::cmd) rearm_minions: Option<bool>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatTimeOptions {
    pub(in crate::cmd::stats) fn into_core(self) -> rc::stats::StatTimeOptions {
        match self {
            Self::Burst(inner) => rc::stats::StatTimeOptions::Burst(inner.into_core()),
            Self::Sim(inner) => rc::stats::StatTimeOptions::Sim(inner.into_core()),
        }
    }
}
impl HStatTimeOptionsBurst {
    fn into_core(self) -> rc::stats::StatTimeOptionsBurst {
        rc::stats::StatTimeOptionsBurst {
            spool: self.spool.map(|v| v.into_core()),
        }
    }
}
impl HStatTimeOptionsSim {
    fn into_core(self) -> rc::stats::StatTimeOptionsSim {
        rc::stats::StatTimeOptionsSim {
            time: self.time.map(rc::PValue::from_f64_clamped),
            reload_optionals: self.reload_optionals,
            rearm_minions: self.rearm_minions,
        }
    }
}
