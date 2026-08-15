use crate::val::SolValInfoDetails;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SolValInfo {
    pub passed: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "custom_serde::skip_details"))]
    pub details: Option<SolValInfoDetails>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use super::*;

    pub(super) fn skip_details(details: &Option<SolValInfoDetails>) -> bool {
        match details {
            Some(details) => details.all_passed(),
            None => true,
        }
    }
}
