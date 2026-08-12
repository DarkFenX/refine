use serde::Deserialize;

use crate::sde::data::ExtractOne;

#[derive(Deserialize)]
pub(in crate::sde) struct SAbil {
    #[serde(rename = "_key")]
    id: i32,
    #[serde(rename = "disallowInHighSec")]
    disallow_in_high_sec: bool,
    #[serde(rename = "disallowInLowSec")]
    disallow_in_low_sec: bool,
}
impl ExtractOne<rc::ed::EAbil> for SAbil {
    fn extract(self) -> Vec<rc::ed::EAbil> {
        vec![rc::ed::EAbil {
            id: rc::ed::EAbilId::from_i32(self.id),
            disallow_hisec: self.disallow_in_high_sec,
            disallow_lowsec: self.disallow_in_low_sec,
        }]
    }
}
