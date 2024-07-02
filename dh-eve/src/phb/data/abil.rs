use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PFighterAbil {
    #[serde(rename = "disallowInHighSec")]
    pub(in crate::phb) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(in crate::phb) disallow_lowsec: bool,
}
impl FsdMerge<rc::ed::EFighterAbil> for PFighterAbil {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EFighterAbil> {
        vec![rc::ed::EFighterAbil::new(
            id,
            self.disallow_hisec,
            self.disallow_lowsec,
        )]
    }
}
