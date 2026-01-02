use crate::{
    ad::AAttrId,
    nd::{NEffectCharge, NEffectChargeDepl, NEffectChargeLoc},
    rd::RAttrId,
    util::RMap,
};

pub(crate) struct REffectCharge {
    pub(crate) location: REffectChargeLoc,
    pub(crate) activates_charge: bool,
}
impl REffectCharge {
    pub(in crate::rd::data::effect) fn try_from_n_charge(
        n_charge: &NEffectCharge,
        attr_id_key_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        Some(Self {
            location: REffectChargeLoc::try_from_n_charge_loc(&n_charge.location, attr_id_key_map)?,
            activates_charge: n_charge.activates_charge,
        })
    }
}

pub(crate) enum REffectChargeLoc {
    Loaded(NEffectChargeDepl),
    Autocharge(RAttrId),
    TargetAttack,
}
impl REffectChargeLoc {
    pub(in crate::rd::data::effect) fn try_from_n_charge_loc(
        n_charge_loc: &NEffectChargeLoc,
        attr_id_key_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        match n_charge_loc {
            NEffectChargeLoc::Loaded(n_charge_depl) => Some(Self::Loaded(*n_charge_depl)),
            NEffectChargeLoc::Autocharge(attr_id) => {
                let attr_key = *attr_id_key_map.get(attr_id)?;
                Some(Self::Autocharge(attr_key))
            }
            NEffectChargeLoc::TargetAttack(_) => Some(Self::TargetAttack),
        }
    }
    pub(crate) fn get_autocharge_attr_key(&self) -> Option<RAttrId> {
        match self {
            Self::Loaded(_) => None,
            Self::Autocharge(attr_key) => Some(*attr_key),
            Self::TargetAttack => None,
        }
    }
}
