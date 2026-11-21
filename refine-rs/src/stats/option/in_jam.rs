use crate::stats::StatTimeOptions;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionIncomingJam {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
}
const impl Default for StatOptionIncomingJam {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionIncomingJam {
    pub const fn new() -> Self {
        Self::default()
    }
    pub const fn with_time(mut self, time: StatTimeOptions) -> Self {
        self.time = time;
        self
    }
}
