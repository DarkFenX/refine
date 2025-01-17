use crate::{
    defs::{AttrVal, OF},
    sol::SolDmgKinds,
};

use super::rah_info::SolRahInfo;

pub(super) struct SolRahDataSim {
    pub(super) info: SolRahInfo,
    pub(super) taken_dmg: SolDmgKinds<AttrVal>,
}
impl SolRahDataSim {
    pub(super) fn new(info: SolRahInfo) -> Self {
        Self {
            info,
            taken_dmg: SolDmgKinds::new(OF(0.0), OF(0.0), OF(0.0), OF(0.0)),
        }
    }
}
