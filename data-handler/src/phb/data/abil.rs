use serde::Deserialize;

use crate::phb::parsing::{Key, KeyMerge};

#[derive(Deserialize)]
pub(in crate::phb) struct PFighterAbil {
    #[serde(rename = "disallowInHighSec")]
    pub(in crate::phb) disallow_hisec: bool,
    #[serde(rename = "disallowInLowSec")]
    pub(in crate::phb) disallow_lowsec: bool,
}
impl KeyMerge<rc::ed::EAbil> for PFighterAbil {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EAbil> {
        vec![rc::ed::EAbil {
            id: rc::ed::EAbilId::from_i32(key),
            disallow_hisec: self.disallow_hisec,
            disallow_lowsec: self.disallow_lowsec,
        }]
    }
}
