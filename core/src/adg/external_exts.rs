use crate::{ad, nd};

impl ad::AEffectBuffInfo {
    pub(in crate::adg) fn extract_a_attr_ids(&self) -> Vec<ad::AAttrId> {
        let mut a_attr_ids = Vec::new();
        match &self.source {
            ad::AEffectBuffSrc::DefaultAttrs => (),
            ad::AEffectBuffSrc::Customized(buff_srcs) => {
                for buff_src in buff_srcs.iter() {
                    match buff_src {
                        ad::AEffectBuffSrcCustom::AffectorVal(_, a_attr_id) => a_attr_ids.push(*a_attr_id),
                        ad::AEffectBuffSrcCustom::HardcodedVal(_, _) => (),
                    }
                }
            }
        }
        a_attr_ids
    }
    pub(in crate::adg) fn extract_a_buff_ids(&self) -> Vec<ad::ABuffId> {
        let mut a_buff_ids = Vec::new();
        match &self.source {
            ad::AEffectBuffSrc::DefaultAttrs => (),
            ad::AEffectBuffSrc::Customized(buff_srcs) => {
                for buff_src in buff_srcs.iter() {
                    match buff_src {
                        ad::AEffectBuffSrcCustom::AffectorVal(a_buff_id, _) => a_buff_ids.push(*a_buff_id),
                        ad::AEffectBuffSrcCustom::HardcodedVal(a_buff_id, _) => a_buff_ids.push(*a_buff_id),
                    }
                }
            }
        }
        a_buff_ids
    }
}

impl nd::NEffect {
    pub(in crate::adg) fn extract_a_attr_ids(&self) -> Vec<ad::AAttrId> {
        let mut a_attr_ids = Vec::new();
        if let Some(n_charge) = self.hc.charge
            && let Some(a_attr_id) = n_charge.location.get_autocharge_attr_id()
        {
            a_attr_ids.push(a_attr_id)
        }
        a_attr_ids
    }
}
