use crate::DpsProfile;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionEhp {
    pub(in crate::stats) incoming_dps: Option<DpsProfile> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionEhp {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_incoming_dps(mut self, incoming_dps: DpsProfile) -> Self {
        self.incoming_dps = Some(incoming_dps);
        self
    }
}
