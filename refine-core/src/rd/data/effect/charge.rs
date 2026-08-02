use crate::nd::{NEffectCharge, NEffectChargeDepl, NEffectChargeLoc};

pub(crate) struct REffectCharge {
    pub(crate) location: REffectChargeLoc,
    pub(crate) activates_charge: bool,
}

pub(crate) enum REffectChargeLoc {
    Loaded(NEffectChargeDepl),
    Autocharge,
    TargetAttack,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectCharge {
    pub(in crate::rd::data::effect) fn from_n_charge(n_charge: &NEffectCharge) -> Self {
        Self {
            location: REffectChargeLoc::from_n_charge_loc(&n_charge.location),
            activates_charge: n_charge.activates_charge,
        }
    }
}

impl REffectChargeLoc {
    fn from_n_charge_loc(n_charge_loc: &NEffectChargeLoc) -> Self {
        match n_charge_loc {
            NEffectChargeLoc::Loaded(n_charge_depl) => Self::Loaded(*n_charge_depl),
            NEffectChargeLoc::Autocharge(..) => Self::Autocharge,
            NEffectChargeLoc::TargetAttack(..) => Self::TargetAttack,
        }
    }
}
