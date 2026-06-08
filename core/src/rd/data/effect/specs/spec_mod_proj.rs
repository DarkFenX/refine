use crate::{
    ad::{AAttrId, AEffect},
    dbg::DebugResult,
    nd::{NEffectProjGetter, NEffectProjModSpec},
    rd::{RAttrId, REffectResist},
    ud::UData,
    util::RMap,
};

#[derive(Copy, Clone)]
pub(crate) struct REffectProjModSpec {
    pub(crate) proj_mult: NEffectProjGetter,
    pub(crate) proj_attr_rids: [Option<RAttrId>; 2],
    pub(crate) resist: Option<REffectResist>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectProjModSpec {
    pub(in crate::rd::data::effect) fn from_n_proj_mod_spec(
        n_proj_mod_spec: &NEffectProjModSpec,
        a_effect: &AEffect,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Self {
        Self {
            proj_mult: n_proj_mod_spec.proj_mult,
            proj_attr_rids: n_proj_mod_spec
                .proj_mult
                .get_modifier_attr_aids(a_effect)
                .map(|attr_aid| attr_aid.and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid).copied())),
            resist: REffectResist::try_from_n_effect_resist(&n_proj_mod_spec.resist, a_effect, attr_aid_rid_map),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectProjModSpec {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for attr_rid in self.proj_attr_rids {
            if let Some(attr_rid) = attr_rid {
                attr_rid.consistency_check(u_data)?;
            }
        }
        if let Some(resist) = &self.resist {
            resist.consistency_check(u_data)?;
        }
        Ok(())
    }
}
