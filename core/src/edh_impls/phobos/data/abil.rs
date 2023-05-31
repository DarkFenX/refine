use crate::{defs::ReeInt, edh_impls::phobos::fsd::FsdMerge, edt};

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct FighterAbil {
    #[serde(rename = "targetMode")]
    pub(in super::super) target_mode: String,
    #[serde(rename = "disallowInHighSec")]
    pub(in super::super) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(in super::super) disallow_lowsec: bool,
}
impl FsdMerge<edt::FighterAbil> for FighterAbil {
    fn fsd_merge(self, id: ReeInt) -> Vec<edt::FighterAbil> {
        vec![edt::FighterAbil::new(
            id,
            self.target_mode,
            self.disallow_hisec,
            self.disallow_lowsec,
        )]
    }
}
