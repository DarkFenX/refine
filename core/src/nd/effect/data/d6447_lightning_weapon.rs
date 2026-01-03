use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectDmgKind, NEffectProjecteeFilter, effect::data::shared::base_opc::get_direct_dd_dmg_opc_spec},
    ud::UItem,
};

const EFFECT_EID: EEffectId = ec::effects::LIGHTNING_WEAPON;
const EFFECT_AID: AEffectId = ac::effects::LIGHTNING_WEAPON;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        projectee_filter: Some(NEffectProjecteeFilter::ItemListAttr(ac::attrs::TGT_FILTER_TYPELIST_ID)),
        dmg_kind_getter: Some(internal_get_dmg_kind),
        // Standup vorton seems to work similarly to direct DDs - same attributes, no range limits
        normal_dmg_opc_spec: Some(get_direct_dd_dmg_opc_spec()),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}
