use serde::Deserialize;

use crate::sde::data::{Key, KeyMergeOne};

#[derive(Deserialize)]
pub(in crate::sde) struct PAbil {
    #[serde(rename = "disallowInHighSec")]
    disallow_in_high_sec: bool,
    #[serde(rename = "disallowInLowSec")]
    disallow_in_low_sec: bool,
}
impl KeyMergeOne<rc::ed::EAbil> for PAbil {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EAbil> {
        vec![rc::ed::EAbil {
            id: rc::ed::EAbilId::from_i32(key),
            disallow_hisec: self.disallow_in_high_sec,
            disallow_lowsec: self.disallow_in_low_sec,
        }]
    }
}
