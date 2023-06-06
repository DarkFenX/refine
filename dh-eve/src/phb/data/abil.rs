use crate::phb::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PFighterAbil {
    #[serde(rename = "targetMode")]
    pub(in crate::phb) target_mode: String,
    #[serde(rename = "disallowInHighSec")]
    pub(in crate::phb) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(in crate::phb) disallow_lowsec: bool,
}
impl FsdMerge<rc::ed::EFighterAbil> for PFighterAbil {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EFighterAbil> {
        vec![rc::ed::EFighterAbil::new(
            id,
            self.target_mode,
            self.disallow_hisec,
            self.disallow_lowsec,
        )]
    }
}
