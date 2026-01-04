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
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        Some(Self {
            location: REffectChargeLoc::try_from_n_charge_loc(&n_charge.location, attr_aid_rid_map)?,
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
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        match n_charge_loc {
            NEffectChargeLoc::Loaded(n_charge_depl) => Some(Self::Loaded(*n_charge_depl)),
            NEffectChargeLoc::Autocharge(attr_aid) => {
                let attr_rid = *attr_aid_rid_map.get(attr_aid)?;
                Some(Self::Autocharge(attr_rid))
            }
            NEffectChargeLoc::TargetAttack(_) => Some(Self::TargetAttack),
        }
    }
    pub(crate) fn get_autocharge_attr_rid(&self) -> Option<RAttrId> {
        match self {
            Self::Loaded(_) => None,
            Self::Autocharge(attr_rid) => Some(*attr_rid),
            Self::TargetAttack => None,
        }
    }
}
