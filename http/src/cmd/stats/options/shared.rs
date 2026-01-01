use crate::shared::HSpool;

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
#[educe(Default)]
pub(in crate::cmd) enum HStatTimeOptions {
    #[educe(Default)]
    Burst(HStatTimeOptionsBurst),
    Sim(HStatTimeOptionsSim),
}
impl From<HStatTimeOptions> for rc::stats::StatTimeOptions {
    fn from(h_time_options: HStatTimeOptions) -> Self {
        match h_time_options {
            HStatTimeOptions::Burst(inner) => Self::Burst(inner.into()),
            HStatTimeOptions::Sim(inner) => Self::Sim(inner.into()),
        }
    }
}

#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatTimeOptionsBurst {
    #[serde(default)]
    pub(in crate::cmd) spool: Option<HSpool>,
}
impl From<HStatTimeOptionsBurst> for rc::stats::StatTimeOptionsBurst {
    fn from(h_time_options: HStatTimeOptionsBurst) -> Self {
        Self {
            spool: h_time_options.spool.map(Into::into),
        }
    }
}

#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatTimeOptionsSim {
    #[serde(default)]
    pub(in crate::cmd) time: Option<rc::AttrVal>,
    #[serde(default)]
    pub(in crate::cmd) reload_optionals: Option<bool>,
    #[serde(default)]
    pub(in crate::cmd) rearm_minions: Option<bool>,
}
impl From<HStatTimeOptionsSim> for rc::stats::StatTimeOptionsSim {
    fn from(h_time_options: HStatTimeOptionsSim) -> Self {
        Self {
            time: h_time_options.time,
            reload_optionals: h_time_options.reload_optionals,
            rearm_minions: h_time_options.rearm_minions,
        }
    }
}
