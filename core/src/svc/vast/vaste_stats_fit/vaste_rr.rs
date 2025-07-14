use crate::{
    def::{AttrVal, OF},
    misc::{EffectSpec, Spool},
    nd::NRemoteRepGetter,
    svc::{
        SvcCtx,
        calc::Calc,
        efuncs,
        vast::{StatTank, VastFitData},
    },
};

impl VastFitData {
    pub(in crate::svc) fn get_stat_remote_rps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        StatTank {
            shield: get_orrps(ctx, calc, spool, self.orr_shield.iter()),
            armor: get_orrps(ctx, calc, spool, self.orr_armor.iter()),
            hull: get_orrps(ctx, calc, spool, self.orr_hull.iter()),
        }
    }
    pub(in crate::svc) fn get_stat_remote_cps(&self, ctx: SvcCtx, calc: &mut Calc) -> AttrVal {
        get_orrps(ctx, calc, None, self.orr_cap.iter())
    }
}

fn get_orrps<'a>(
    ctx: SvcCtx,
    calc: &mut Calc,
    spool: Option<Spool>,
    data: impl Iterator<Item = (&'a EffectSpec, &'a NRemoteRepGetter)>,
) -> AttrVal {
    let mut rps = OF(0.0);
    for (&espec, rep_getter) in data {
        if let Some(rep_amount_per_cycle) = rep_getter(ctx, calc, espec, spool, None) {
            if let Some(cycle_time) = efuncs::get_espec_cycle_time(ctx, calc, espec) {
                rps += rep_amount_per_cycle / cycle_time;
            }
        }
    }
    rps
}
