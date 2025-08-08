use super::{
    info_autocharge::get_autocharge_cycle_info, info_charge::get_charge_cycle_info, info_drone::get_drone_cycle_info,
    info_module::get_module_cycle_info, info_shared::CycleOptions,
};
use crate::{
    def::{AttrVal, Count},
    rd::REffectKey,
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemKey},
    util::{InfCount, RMap},
};

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleSimple {
    pub(in crate::svc) active_time: AttrVal,
    pub(in crate::svc) inactive_time: AttrVal,
    pub(in crate::svc) repeat_count: InfCount,
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

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleReload1 {
    pub(in crate::svc) inner: CycleInner,
}
impl CycleReload1 {
    fn get_cycles_until_reload(&self) -> InfCount {
        InfCount::Count(self.inner.repeat_count)
    }
    fn get_average_cycle_time(&self) -> AttrVal {
        self.inner.active_time + self.inner.inactive_time
    }
}

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleReload2 {
    pub(in crate::svc) inner_early: CycleInner,
    pub(in crate::svc) inner_final: CycleInner,
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

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleInner {
    pub(in crate::svc) active_time: AttrVal,
    pub(in crate::svc) inactive_time: AttrVal,
    pub(in crate::svc) repeat_count: Count,
}
impl CycleInner {
    fn get_total_time(&self) -> AttrVal {
        (self.active_time + self.inactive_time) * self.repeat_count as f64
    }
}

pub(in crate::svc) fn get_item_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    options: CycleOptions,
    ignore_state: bool,
) -> Option<RMap<REffectKey, Cycle>> {
    let item = ctx.u_data.items.get(item_key);
    match item {
        UItem::Autocharge(autocharge) => get_autocharge_cycle_info(ctx, calc, autocharge, options, ignore_state),
        UItem::Charge(charge) => get_charge_cycle_info(ctx, calc, charge, options, ignore_state),
        UItem::Drone(drone) => get_drone_cycle_info(ctx, calc, item_key, drone, ignore_state),
        UItem::Fighter(fighter) => {
            if !fighter.is_loaded() {
                return None;
            };
            Some(RMap::new())
        }
        UItem::Module(module) => get_module_cycle_info(ctx, calc, item_key, item, module, options, ignore_state),
        _ => None,
    }
}
