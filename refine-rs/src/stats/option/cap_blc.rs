use crate::stats::{StatCapBlcSrcKinds, StatTimeOptions, StatTimeOptionsSim};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionCapBlc {
    #[cfg_attr(feature = "serde", serde(default))]
    pub src_kinds: StatCapBlcSrcKinds = StatCapBlcSrcKinds::default(),
    // Unlike other stats, default is sim mode over burst mode
    #[cfg_attr(feature = "serde", serde(default = "time_default"))]
    pub time: StatTimeOptions = StatTimeOptions::Sim(StatTimeOptionsSim { .. }),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
fn time_default() -> StatTimeOptions {
    StatTimeOptions::Sim(StatTimeOptionsSim { .. })
}
