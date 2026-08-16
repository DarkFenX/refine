use crate::{DpsProfile, FitSecStatus, TriStateField};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(super) struct CmdFitChangeShared {
    pub(super) sec_status: Option<FitSecStatus> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(super) rah_incoming_dps: TriStateField<DpsProfile> = TriStateField::Absent,
}
