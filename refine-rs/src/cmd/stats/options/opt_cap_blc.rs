use crate::stats::{StatCapBlcSrcKinds, StatTimeOptions, StatTimeOptionsSim};

#[derive(Copy, Clone, Default)]
pub struct StatOptionCapBlc {
    pub src_kinds: StatCapBlcSrcKinds = StatCapBlcSrcKinds { default: true, .. },
    // Unlike other stats, default is sim mode over burst mode
    pub time_options: StatTimeOptions = StatTimeOptions::Sim(StatTimeOptionsSim { .. }),
}
