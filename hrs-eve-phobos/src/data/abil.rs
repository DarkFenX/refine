use crate::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct FighterAbil {
    #[serde(rename = "targetMode")]
    pub(crate) target_mode: String,
    #[serde(rename = "disallowInHighSec")]
    pub(crate) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(crate) disallow_lowsec: bool,
}
impl FsdMerge<rc::ed::EFighterAbil> for FighterAbil {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EFighterAbil> {
        vec![rc::ed::EFighterAbil::new(
            id,
            self.target_mode,
            self.disallow_hisec,
            self.disallow_lowsec,
        )]
    }
}
