use crate::{
    ed::EAttrId,
    nd::{NEffect, NEffectChargeLoc},
};

impl NEffect {
    pub(super) fn iter_e_attr_ids(&self) -> impl Iterator<Item = EAttrId> {
        self.charge
            .as_ref()
            .and_then(|v| v.location.get_e_attr_id())
            .into_iter()
    }
}

impl NEffectChargeLoc {
    fn get_e_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::Loaded(_) => None,
            Self::Autocharge(attr_id) => attr_id.dc_eve(),
            Self::TargetAttack(attr_id) => attr_id.dc_eve(),
        }
    }
}
