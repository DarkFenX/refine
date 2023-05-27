use crate::{defs::ReeInt, dh};

use super::super::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct FighterAbil {
    #[serde(rename = "targetMode")]
    pub(in super::super) target_mode: String,
    #[serde(rename = "disallowInHighSec")]
    pub(in super::super) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(in super::super) disallow_lowsec: bool,
}
impl FsdMerge<dh::FighterAbil> for FighterAbil {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::FighterAbil> {
        vec![dh::FighterAbil::new(
            id,
            self.target_mode,
            self.disallow_hisec,
            self.disallow_lowsec,
        )]
    }
}
