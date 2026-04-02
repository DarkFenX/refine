use crate::{
    nd::{NEffectEcmOutputGetter, NEffectProjOpcSpec},
    num::Value,
    rd::RAttrConsts,
    ud::UItem,
};

pub(crate) struct NEffectEcm {
    pub(crate) checker: Option<NEffectEcmChecker>,
    pub(crate) ospec: NEffectProjOpcSpec<NEffectEcmOutputGetter>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Base item checker
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) enum NEffectEcmChecker {
    Bomb,
}
impl NEffectEcmChecker {
    pub(crate) fn check(&self, u_item: &UItem, attr_consts: &RAttrConsts) -> bool {
        match self {
            Self::Bomb => check_bomb(u_item, attr_consts),
        }
    }
}

fn check_bomb(u_item: &UItem, attr_consts: &RAttrConsts) -> bool {
    u_item.get_oattr_ffb(attr_consts.scan_radar_strength_bonus, Value::ZERO) > Value::ZERO
        || u_item.get_oattr_ffb(attr_consts.scan_magnetometric_strength_bonus, Value::ZERO) > Value::ZERO
        || u_item.get_oattr_ffb(attr_consts.scan_gravimetric_strength_bonus, Value::ZERO) > Value::ZERO
        || u_item.get_oattr_ffb(attr_consts.scan_ladar_strength_bonus, Value::ZERO) > Value::ZERO
}
