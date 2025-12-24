use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    misc::{DmgKinds, Spool},
    nd::{
        NEffect,
        effect::data::shared::{base_opc::get_missile_dmg_opc, proj_mult::get_null_proj_mult},
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::DEFENDER_MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::DEFENDER_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        normal_dmg_opc_getter: Some(internal_get_dmg_opc),
        ..
    }
}

fn internal_get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    // Defenders cannot be used vs targets allowed by the lib, so always return 0 if target is
    // specified
    get_missile_dmg_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        projectee_key,
        get_null_proj_mult,
    )
}
