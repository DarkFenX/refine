use ordered_float::OrderedFloat as OF;

use super::rah_info::RahInfo;
use crate::{def::AttrVal, misc::DmgKinds};

pub(super) struct RahDataSim {
    pub(super) info: RahInfo,
    pub(super) taken_dmg: DmgKinds<AttrVal>,
}
impl RahDataSim {
    pub(super) fn new(info: RahInfo) -> Self {
        Self {
            info,
            taken_dmg: DmgKinds {
                em: OF(0.0),
                thermal: OF(0.0),
                kinetic: OF(0.0),
                explosive: OF(0.0),
            },
        }
    }
}
