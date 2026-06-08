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
    pub(crate) proj_mult: Option<NEffectProjGetter>,
    pub(crate) proj_attr_rids: [Option<RAttrId>; 2],
    pub(crate) resist: Option<REffectResist>,
}
impl REffectProjModSpec {
    pub(in crate::rd::data::effect) fn default_from_a_effect(
        a_effect: &AEffect,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Self {
        // Assume effects are not range-limited by default
        Self {
            proj_mult: None,
            proj_attr_rids: [None, None],
            resist: match a_effect.resist_attr_id.as_ref() {
                // Effect is not resisted if specified attribute doesn't exist
                Some(attr_aid) => attr_aid_rid_map.get(attr_aid).map(|v| REffectResist::Attr(*v)),
                None => Some(REffectResist::RemoteResistance),
            },
        }
    }
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
            proj_attr_rids: match n_proj_mod_spec.proj_mult {
                Some(proj_getter) => proj_getter
                    .get_modifier_attr_aids(a_effect)
                    .map(|attr_aid| attr_aid.and_then(|attr_aid| attr_aid_rid_map.get(&attr_aid).copied())),
                None => [None, None],
            },
            resist: REffectResist::try_from_n_effect_resist(&n_proj_mod_spec.resist, a_effect, attr_aid_rid_map),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectProjModSpec {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for attr_rid in self.proj_attr_rids.into_iter().flatten() {
            attr_rid.consistency_check(u_data)?;
        }
        if let Some(resist) = &self.resist {
            resist.consistency_check(u_data)?;
        }
        Ok(())
    }
}
