use super::{info_drone::get_drone_cycle_info, info_module::get_module_cycle_info, info_shared::CycleOptions};
use crate::{
    ad,
    def::{AttrVal, Count, ItemKey},
    svc::{SvcCtx, calc::Calc},
    uad::UadItem,
    util::{InfCount, RMap},
};

pub(in crate::svc) enum Cycle {
    Simple(CycleSimple),
    Reload1(CycleReload1),
    Reload2(CycleReload2),
}
impl Cycle {
    pub(in crate::svc) fn get_cycles_until_reload(&self) -> InfCount {
        match self {
            Self::Simple(simple) => simple.get_cycles_until_reload(),
            Self::Reload1(reload1) => reload1.get_cycles_until_reload(),
            Self::Reload2(reload2) => reload2.get_cycles_until_reload(),
        }
    }
    pub(in crate::svc) fn get_average_cycle_time(&self) -> AttrVal {
        match self {
            Self::Simple(simple) => simple.get_average_cycle_time(),
            Self::Reload1(reload1) => reload1.get_average_cycle_time(),
            Self::Reload2(reload2) => reload2.get_average_cycle_time(),
        }
    }
}

pub(in crate::svc) struct CycleSimple {
    pub(super) active_time: AttrVal,
    pub(super) inactive_time: AttrVal,
    pub(super) repeat_count: InfCount,
}
impl CycleSimple {
    fn get_cycles_until_reload(&self) -> InfCount {
        // Even if charges are depletable, consider moment of depletion as a "reload" for the
        // purpose of this method
        self.repeat_count
    }
    fn get_average_cycle_time(&self) -> AttrVal {
        self.active_time + self.inactive_time
    }
}

pub(in crate::svc) struct CycleReload1 {
    pub(super) inner: CycleInner,
}
impl CycleReload1 {
    fn get_cycles_until_reload(&self) -> InfCount {
        InfCount::Count(self.inner.repeat_count)
    }
    fn get_average_cycle_time(&self) -> AttrVal {
        self.inner.active_time + self.inner.inactive_time
    }
}

pub(in crate::svc) struct CycleReload2 {
    pub(super) inner_early: CycleInner,
    pub(super) inner_final: CycleInner,
}
impl CycleReload2 {
    fn get_cycles_until_reload(&self) -> InfCount {
        InfCount::Count(self.inner_early.repeat_count + self.inner_final.repeat_count)
    }
    fn get_average_cycle_time(&self) -> AttrVal {
        (self.inner_early.get_total_time() + self.inner_final.get_total_time())
            / (self.inner_early.repeat_count + self.inner_final.repeat_count) as f64
    }
}

pub(super) struct CycleInner {
    pub(super) active_time: AttrVal,
    pub(super) inactive_time: AttrVal,
    pub(super) repeat_count: Count,
}
impl CycleInner {
    fn get_total_time(&self) -> AttrVal {
        (self.active_time + self.inactive_time) * self.repeat_count as f64
    }
}

pub(in crate::svc) fn get_item_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    options: CycleOptions,
    ignore_state: bool,
) -> Option<RMap<ad::AEffectId, Cycle>> {
    let uad_item = ctx.uad.items.get(item_key);
    match uad_item {
        UadItem::Autocharge(uad_autocharge) => {
            get_item_cycle_info(ctx, calc, uad_autocharge.get_cont_key(), options, ignore_state)
        }
        UadItem::Charge(uad_charge) => get_item_cycle_info(ctx, calc, uad_charge.get_cont_key(), options, ignore_state),
        UadItem::Drone(uad_drone) => get_drone_cycle_info(ctx, calc, item_key, uad_drone, ignore_state),
        UadItem::Fighter(uad_fighter) => {
            if !uad_fighter.is_loaded() {
                return None;
            };
            Some(RMap::new())
        }
        UadItem::Module(uad_module) => {
            get_module_cycle_info(ctx, calc, item_key, uad_item, uad_module, options, ignore_state)
        }
        _ => None,
    }
}
