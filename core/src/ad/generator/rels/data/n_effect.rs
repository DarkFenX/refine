use crate::{
    ed::EAttrId,
    nd::{NEffect, NEffectChargeLoc},
};

impl NEffect {
    pub(super) fn iter_attr_eids(&self) -> impl Iterator<Item = EAttrId> {
        self.charge.as_ref().and_then(|v| v.location.get_attr_eid()).into_iter()
    }
}

impl NEffectChargeLoc {
    fn get_attr_eid(&self) -> Option<EAttrId> {
        match self {
            Self::Loaded(_) => None,
            Self::Autocharge(attr_aid) => attr_aid.dc_eve(),
            Self::TargetAttack(attr_aid) => attr_aid.dc_eve(),
        }
    }
}
