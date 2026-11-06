use super::{cache::CseqCache, cycling_options::ModuleCyclingOptions, item_module::get_module_cseq_map};
use crate::{
    svc::{SvcCtx, calc::Calc},
    ud::{UCharge, UItem},
};

#[must_use]
pub(super) fn build_charge_cseq_map(
    cseq_cache: &mut CseqCache,
    ctx: &SvcCtx,
    calc: &mut Calc,
    charge: &UCharge,
    module_co: ModuleCyclingOptions,
) -> bool {
    if !charge.is_loaded() {
        return false;
    };
    let cont_item_uid = charge.get_cont_item_uid();
    let cont_item = ctx.u_data.items.get(cont_item_uid);
    // Charges are loaded only into modules
    let UItem::Module(cont_module) = cont_item else {
        return false;
    };
    // Default effect of parent item is assumed to control the charge. If there is none, charge is
    // not cycling
    let cont_effect_rid = match cont_item.get_r_item_base() {
        Some(cont_rib) if let Some(cont_effect_rid) = cont_rib.defeff_rid => cont_effect_rid,
        _ => return false,
    };
    // If cycle info for parent item is not available, charge is not cycling
    let Some(cont_cseq_map) =
        get_module_cseq_map(cseq_cache, ctx, calc, cont_item_uid, cont_item, cont_module, module_co)
    else {
        return false;
    };
    // If controlling effect is not cycling, charge is not cycling either
    let Some(&cont_effect_cycle) = cont_cseq_map.get(&cont_effect_rid) else {
        return false;
    };
    let effect_rids = charge.get_reffs().unwrap().iter();
    cseq_cache.reserve(effect_rids.len());
    for &effect_rid in effect_rids {
        let effect = ctx.u_data.r_data.get_effect_by_rid(effect_rid);
        if effect.is_active() {
            cseq_cache.insert(effect_rid, cont_effect_cycle);
        }
    }
    true
}
