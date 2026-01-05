use either::Either;

use super::{item::get_item_cseq_map, shared::CyclingOptions};
use crate::{
    rd::REffectId,
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UAutocharge,
    util::RMap,
};

pub(super) fn get_autocharge_cseq_map(
    ctx: SvcCtx,
    calc: &mut Calc,
    autocharge: &UAutocharge,
    options: CyclingOptions,
    ignore_state: bool,
) -> Option<RMap<REffectId, CycleSeq>> {
    if !autocharge.is_loaded() {
        return None;
    };
    // Autocharge cycles rely on parent item cycles
    let mut cseq_map = get_item_cseq_map(ctx, calc, autocharge.get_cont_item_uid(), options, ignore_state)?;
    // If effect controlling the autocharge doesn't cycle, autocharge doesn't cycle either
    let cont_effect_cycle = cseq_map.remove(&autocharge.get_cont_effect_rid())?;
    cseq_map.clear();
    let effect_keys = match ignore_state {
        true => Either::Left(autocharge.get_effect_datas().unwrap().keys().copied()),
        false => Either::Right(autocharge.get_reffs().unwrap().iter().copied()),
    };
    cseq_map.reserve(effect_keys.len());
    for effect_key in effect_keys {
        cseq_map.insert(effect_key, cont_effect_cycle);
    }
    Some(cseq_map)
}
