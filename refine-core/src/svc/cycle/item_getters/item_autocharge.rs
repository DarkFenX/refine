use super::{cache::CseqBuild, cycling_options::FighterCyclingOptions, item_fighter::get_fighter_cseq_map};
use crate::{
    svc::{SvcCtx, calc::Calc},
    ud::{UAutocharge, UItem},
};

#[must_use]
pub(super) fn build_autocharge_cseq_map(
    cseq_build: &mut CseqBuild,
    ctx: &SvcCtx,
    calc: &mut Calc,
    autocharge: &UAutocharge,
    fighter_co: FighterCyclingOptions,
) -> bool {
    if !autocharge.is_loaded() {
        return false;
    };
    let cont_item_uid = autocharge.get_cont_item_uid();
    // Only fighters carry autocharges
    let UItem::Fighter(cont_fighter) = ctx.u_data.items.get(cont_item_uid) else {
        return false;
    };
    // Autocharge cycles rely on parent item cycles
    let Some(cont_cseq_map) =
        get_fighter_cseq_map(cseq_build.cache(), ctx, calc, cont_item_uid, cont_fighter, fighter_co)
    else {
        return false;
    };
    // If effect controlling the autocharge doesn't cycle, autocharge doesn't cycle either
    let Some(&cont_effect_cycle) = cont_cseq_map.get(&autocharge.get_cont_effect_rid()) else {
        return false;
    };
    let effect_rids = autocharge.get_reffs().unwrap().iter();
    cseq_build.reserve(effect_rids.len());
    for &effect_rid in effect_rids {
        let effect = ctx.u_data.r_data.get_effect_by_rid(effect_rid);
        if effect.is_active() {
            cseq_build.insert(effect_rid, cont_effect_cycle);
        }
    }
    true
}
