use crate::{DpsProfile, FitSecStatus};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(super) struct FitAddCmdShared {
    pub(super) sec_status: Option<FitSecStatus> = None,
    pub(super) rah_incoming_dps: Option<DpsProfile> = None,
}
