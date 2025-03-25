use ordered_float::OrderedFloat as OF;

use crate::sol::{AttrVal, DmgKinds};

use super::rah_info::RahInfo;

pub(super) struct RahDataSim {
    pub(super) info: RahInfo,
    pub(super) taken_dmg: DmgKinds<AttrVal>,
}
impl RahDataSim {
    pub(super) fn new(info: RahInfo) -> Self {
        Self {
            info,
            taken_dmg: DmgKinds::new(OF(0.0), OF(0.0), OF(0.0), OF(0.0)),
        }
    }
}
