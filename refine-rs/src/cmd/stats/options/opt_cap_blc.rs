use crate::stats::{StatCapBlcSrcKinds, StatTimeOptions, StatTimeOptionsSim};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionCapBlc {
    #[cfg_attr(feature = "serde", serde(default))]
    pub src_kinds: StatCapBlcSrcKinds = StatCapBlcSrcKinds { default: true, .. },
    // Unlike other stats, default is sim mode over burst mode
    #[cfg_attr(feature = "serde", serde(default = "time_options_default"))]
    pub time_options: StatTimeOptions = StatTimeOptions::Sim(StatTimeOptionsSim { .. }),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
fn time_options_default() -> StatTimeOptions {
    StatTimeOptions::Sim(StatTimeOptionsSim { .. })
}
