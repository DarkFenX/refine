use super::shared::get_orps_cycle_options;
use crate::{
    def::{AttrVal, OF},
    misc::Spool,
    rd::{REffectKey, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::get_item_cseq_map,
        spool::ResolvedSpool,
        vast::{StatOutRepItemKinds, StatTank, Vast},
    },
    ud::{UFitKey, UItemKey},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_outgoing_rps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatOutRepItemKinds,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        let mut rps = StatTank {
            shield: OF(0.0),
            armor: OF(0.0),
            hull: OF(0.0),
        };
        for fit_key in fit_keys {
            let fit_data = self.get_fit_data(&fit_key);
            rps.shield += get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_shield);
            rps.armor += get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_armor);
            rps.hull += get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_hull);
        }
        rps
    }
    pub(in crate::svc) fn get_stat_fit_outgoing_rps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatOutRepItemKinds,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        let fit_data = self.get_fit_data(&fit_key);
        StatTank {
            shield: get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_shield),
            armor: get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_armor),
            hull: get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_hull),
        }
    }
    pub(in crate::svc) fn get_stat_fits_outgoing_cps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
    ) -> AttrVal {
        fit_keys
            .map(|fit_key| {
                get_orrps(
                    ctx,
                    calc,
                    StatOutRepItemKinds::all_enabled(),
                    None,
                    &self.get_fit_data(&fit_key).out_cap,
                )
            })
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_outgoing_cps(&self, ctx: SvcCtx, calc: &mut Calc, fit_key: UFitKey) -> AttrVal {
        let fit_data = self.get_fit_data(&fit_key);
        get_orrps(ctx, calc, StatOutRepItemKinds::all_enabled(), None, &fit_data.out_cap)
    }
}

fn get_orrps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatOutRepItemKinds,
    spool: Option<Spool>,
    fit_data: &RMapRMap<UItemKey, REffectKey, REffectProjOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut rps = OF(0.0);
    // TODO: allow configuring cycle options by caller
    let cycle_options = get_orps_cycle_options(false);
    for (&item_key, item_data) in fit_data.iter() {
        let cycle_map = match get_item_cseq_map(ctx, calc, item_key, cycle_options, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        let u_item = ctx.u_data.items.get(item_key);
        if !item_kinds.resolve(u_item) {
            continue;
        }
        for (&effect_key, ospec) in item_data.iter() {
            let effect = ctx.u_data.src.get_effect(effect_key);
            let effect_cycle = match cycle_map.get(&effect_key) {
                Some(effect_cycle_loop) => effect_cycle_loop.to_time_charge(),
                None => continue,
            };
            let spool_mult = if ospec.spoolable
                && let Some(spool_attrs) = effect.spool_attr_keys
                && let Some(resolved) = ResolvedSpool::try_build(ctx, calc, item_key, effect, spool, spool_attrs)
            {
                Some(resolved.mult)
            } else {
                None
            };
            let effect_cycle_part = effect_cycle.get_first_cycle();
            let invar_data = ospec.make_invar_data(ctx, calc, item_key, effect, None);
            let output_per_cycle = match ospec.get_total(
                ctx,
                calc,
                item_key,
                effect,
                effect_cycle_part.chargedness,
                spool_mult,
                invar_data,
            ) {
                Some(output_per_cycle) => output_per_cycle,
                None => continue,
            };
            rps += output_per_cycle / effect_cycle_part.time;
        }
    }
    rps
}
