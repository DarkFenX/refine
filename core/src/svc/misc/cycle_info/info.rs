use super::{info_drone::get_drone_cycle_info, info_module::get_module_cycle_info, info_shared::CycleOptions};
use crate::{
    ad,
    def::{AttrVal, Count, ItemKey},
    sol::REffs,
    svc::{SvcCtx, calc::Calc},
    uad::UadItem,
    util::{InfCount, RMap},
};

pub(super) enum CycleInfo {
    Simple(CycleSimple),
    Complex(CycleComplex),
}

pub(super) struct CycleInner {
    pub(super) active_time: AttrVal,
    pub(super) inactive_time: AttrVal,
    pub(super) repeat_count: Count,
    pub(super) reload: bool,
}

pub(super) struct CycleSimple {
    pub(super) active_time: AttrVal,
    pub(super) inactive_time: AttrVal,
    pub(super) repeat_count: InfCount,
    pub(super) reload: bool,
}

pub(super) struct CycleComplex {
    pub(super) inner1: CycleInner,
    pub(super) inner2: CycleInner,
    pub(super) repeat_count: InfCount,
}

// TODO: optimize for simpler cases when complex logic is not needed
fn get_item_cycle_info(
    ctx: SvcCtx,
    reffs: &REffs,
    calc: &mut Calc,
    item_key: ItemKey,
    options: CycleOptions,
    ignore_state: bool,
) -> Option<RMap<ad::AEffectId, CycleInfo>> {
    let uad_item = ctx.uad.items.get(item_key);
    match uad_item {
        UadItem::Drone(uad_drone) => get_drone_cycle_info(ctx, reffs, calc, item_key, uad_drone, ignore_state),
        UadItem::Fighter(uad_fighter) => {
            if !uad_fighter.is_loaded() {
                return None;
            };
            let mut cycle_infos = RMap::new();
            Some(cycle_infos)
        }
        UadItem::Module(uad_module) => {
            get_module_cycle_info(ctx, reffs, calc, item_key, uad_item, uad_module, options, ignore_state)
        }
        _ => None,
    }
}
