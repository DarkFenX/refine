use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectHc, NEffectProjecteeFilter, effect::data::shared::opc::get_direct_dd_dmg_opc,
    },
    ud::UItem,
};

const E_EFFECT_ID: EEffectId = ec::effects::LIGHTNING_WEAPON;
const A_EFFECT_ID: AEffectId = ac::effects::LIGHTNING_WEAPON;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            projectee_filter: Some(NEffectProjecteeFilter::ItemListAttr(ac::attrs::TGT_FILTER_TYPELIST_ID)),
            dmg_kind_getter: Some(internal_get_dmg_kind),
            // Standup vorton seems to have the same set of damage attributes as direct DDs
            normal_dmg_opc_getter: Some(get_direct_dd_dmg_opc),
            ..
        },
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}
