use crate::{
    def::{AttrVal, OF},
    misc::{EffectSpec, SpoolOptions},
    nd::NRemoteRepGetter,
    svc::{SvcCtx, calc::Calc, efuncs, vast::VastFitData},
};

impl VastFitData {
    pub(in crate::svc) fn get_stat_orr_shield(&self, ctx: SvcCtx, calc: &mut Calc) -> AttrVal {
        get_orrps(ctx, calc, self.orr_shield.iter())
    }
    pub(in crate::svc) fn get_stat_orr_armor(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        _spool: Option<SpoolOptions>,
    ) -> AttrVal {
        get_orrps(ctx, calc, self.orr_armor.iter())
    }
    pub(in crate::svc) fn get_stat_orr_hull(&self, ctx: SvcCtx, calc: &mut Calc) -> AttrVal {
        get_orrps(ctx, calc, self.orr_hull.iter())
    }
    pub(in crate::svc) fn get_stat_orr_cap(&self, ctx: SvcCtx, calc: &mut Calc) -> AttrVal {
        get_orrps(ctx, calc, self.orr_cap.iter())
    }
}

fn get_orrps<'a>(
    ctx: SvcCtx,
    calc: &mut Calc,
    data: impl Iterator<Item = (&'a EffectSpec, &'a NRemoteRepGetter)>,
) -> AttrVal {
    let mut rps = OF(0.0);
    for (&espec, rep_getter) in data {
        if let Some(rep_amount_per_cycle) = rep_getter(ctx, calc, espec, None) {
            let cycle_time = efuncs::get_espec_cycle_time(ctx, calc, espec).unwrap();
            if cycle_time != OF(0.0) {
                rps += rep_amount_per_cycle / cycle_time;
            }
        }
    }
    rps
}
