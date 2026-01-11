use serde::Deserialize;

use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(Deserialize)]
pub(in crate::phb) struct PFighterAbil {
    #[serde(rename = "disallowInHighSec")]
    pub(in crate::phb) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(in crate::phb) disallow_lowsec: bool,
}
impl FsdMerge<rc::ed::EAbil> for PFighterAbil {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EAbil> {
        vec![rc::ed::EAbil {
            id: rc::ed::EAbilId::from_i32(id),
            disallow_hisec: self.disallow_hisec,
            disallow_lowsec: self.disallow_lowsec,
        }]
    }
}
