use crate::DpsProfile;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionEhp {
    pub incoming_dps: Option<DpsProfile> = None,
}
