use crate::DpsProfile;

#[derive(Copy, Clone, Default)]
pub struct StatOptionEhp {
    pub incoming_dps: Option<DpsProfile> = None,
}
