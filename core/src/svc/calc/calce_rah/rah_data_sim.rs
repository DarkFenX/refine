use super::rah_info::RahInfo;
use crate::misc::{DmgKinds, Value};

pub(super) struct RahDataSim {
    pub(super) info: RahInfo,
    pub(super) taken_dmg: DmgKinds<Value>,
}
impl RahDataSim {
    pub(super) fn new(info: RahInfo) -> Self {
        Self {
            info,
            taken_dmg: DmgKinds {
                em: Value::ZERO,
                thermal: Value::ZERO,
                kinetic: Value::ZERO,
                explosive: Value::ZERO,
            },
        }
    }
}
