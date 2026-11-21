use crate::DpsProfile;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionEhp {
    pub(in crate::stats) incoming_dps: Option<DpsProfile> = None,
}
const impl Default for StatOptionEhp {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionEhp {
    pub const fn new() -> Self {
        Self::default()
    }
    pub const fn with_incoming_dps(mut self, incoming_dps: DpsProfile) -> Self {
        self.incoming_dps = Some(incoming_dps);
        self
    }
}
