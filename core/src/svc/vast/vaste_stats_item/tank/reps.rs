use super::shared::item_check;
use crate::{
    def::{AttrVal, ItemKey, OF},
    misc::EffectSpec,
    nd::NLocalRepGetter,
    svc::{
        SvcCtx,
        calc::Calc,
        efuncs,
        vast::{StatTank, Vast},
    },
    uad::UadItem,
    util::RMap,
};

pub struct StatLayerReps {
    pub local: AttrVal,
    pub remote: AttrVal,
    pub remote_penalized: AttrVal,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_reps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Option<StatTank<StatLayerReps>> {
        let uad_item = ctx.uad.items.get(item_key);
        item_check(uad_item)?;
        // Local reps
        let (local_shield, local_armor, local_hull) = match uad_item {
            UadItem::Ship(uad_ship) => {
                let fit_data = self.get_fit_data(&uad_ship.get_fit_key());
                let local_shield = get_local_rps(ctx, calc, &fit_data.lr_shield);
                let local_armor = get_local_rps(ctx, calc, &fit_data.lr_armor);
                let local_hull = get_local_rps(ctx, calc, &fit_data.lr_hull);
                (local_shield, local_armor, local_hull)
            }
            _ => (OF(0.0), OF(0.0), OF(0.0)),
        };
        Some(StatTank {
            shield: StatLayerReps {
                local: local_shield,
                remote: OF(0.0),
                remote_penalized: OF(0.0),
            },
            armor: StatLayerReps {
                local: local_armor,
                remote: OF(0.0),
                remote_penalized: OF(0.0),
            },
            hull: StatLayerReps {
                local: local_hull,
                remote: OF(0.0),
                remote_penalized: OF(0.0),
            },
        })
    }
}

fn get_local_rps(ctx: SvcCtx, calc: &mut Calc, rep_data: &RMap<EffectSpec, NLocalRepGetter>) -> AttrVal {
    let mut total_rps = OF(0.0);
    for (&rep_espec, rep_getter) in rep_data.iter() {
        if let Some(rep_hp) = rep_getter(ctx, calc, rep_espec.item_key) {
            // Can unwrap here because if rep effects is registered, it should have its item loaded
            // and the effect should have duration attribute specified
            let cycle_time = efuncs::get_espec_cycle_time(ctx, calc, rep_espec).unwrap();
            if cycle_time > OF(0.0) {
                total_rps += rep_hp / cycle_time;
            }
        }
    }
    total_rps
}
