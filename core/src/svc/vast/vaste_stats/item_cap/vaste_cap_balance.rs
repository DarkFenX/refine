use std::cmp::Ordering;

use crate::{
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        aggr::{
            aggr_local_first_ps, aggr_local_looped_ps, aggr_local_time_ps, aggr_proj_first_ps, aggr_proj_looped_ps,
            aggr_proj_time_ps,
        },
        calc::Calc,
        cycle::get_item_cseq_map,
        err::StatItemCheckError,
        vast::{StatTimeOptions, Vast, VastFitData, shared::calc_regen, vaste_stats::item_checks::check_ship},
    },
    ud::UItemId,
    util::UnitInterval,
};

/// Capacitor change sources which will be considered for cap balance stats.
#[derive(Copy, Clone)]
pub struct StatCapSrcKinds {
    pub regen: StatCapRegenOptions,
    pub cap_injectors: bool,
    pub nosfs: bool,
    pub consumers: bool,
    pub incoming_transfers: bool,
    pub incoming_neuts: bool,
}
impl StatCapSrcKinds {
    /// Include all capacitor change sources.
    pub fn all_enabled() -> Self {
        Self {
            regen: StatCapRegenOptions { enabled: true, .. },
            cap_injectors: true,
            nosfs: true,
            consumers: true,
            incoming_transfers: true,
            incoming_neuts: true,
        }
    }
    /// Exclude all capacitor change sources.
    pub fn all_disabled() -> Self {
        Self {
            regen: StatCapRegenOptions { enabled: false, .. },
            cap_injectors: false,
            nosfs: false,
            consumers: false,
            incoming_transfers: false,
            incoming_neuts: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct StatCapRegenOptions {
    pub enabled: bool,
    pub cap_perc: UnitInterval = UnitInterval::new_const(OF(0.25)),
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_balance(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemId,
        src_kinds: StatCapSrcKinds,
        time_options: StatTimeOptions,
    ) -> Result<AttrVal, StatItemCheckError> {
        let ship = check_ship(ctx.u_data, item_key)?;
        let fit_data = self.fit_datas.get(&ship.get_fit_uid()).unwrap();
        let mut balance = OF(0.0);
        if src_kinds.regen.enabled {
            balance += get_cap_regen(ctx, calc, item_key, src_kinds.regen.cap_perc);
        }
        if src_kinds.cap_injectors {
            balance += get_cap_injects(ctx, calc, time_options, fit_data);
        }
        if src_kinds.consumers || src_kinds.nosfs {
            balance -= get_cap_consumed(ctx, calc, time_options, fit_data, src_kinds.consumers, src_kinds.nosfs);
        }
        if src_kinds.incoming_transfers {
            balance += get_cap_transfers(ctx, calc, time_options, item_key, self);
        }
        if src_kinds.incoming_neuts {
            balance -= get_neuts(ctx, calc, time_options, item_key, self);
        }
        Ok(balance)
    }
}

fn get_cap_regen(ctx: SvcCtx, calc: &mut Calc, item_key: UItemId, cap_perc: UnitInterval) -> AttrVal {
    let max_amount = Vast::internal_get_stat_item_cap_unchecked(ctx, calc, item_key);
    let cap_regen_time = calc
        .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().recharge_rate, OF(0.0))
        .unwrap()
        / OF(1000.0);
    calc_regen(max_amount, cap_regen_time, cap_perc.get_inner())
}

fn get_cap_injects(ctx: SvcCtx, calc: &mut Calc, time_options: StatTimeOptions, fit_data: &VastFitData) -> AttrVal {
    let mut cps = OF(0.0);
    let cycling_options = time_options.into();
    for (&item_key, item_data) in fit_data.cap_injects.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_key);
            match time_options {
                StatTimeOptions::Burst(_) => {
                    if let Some(effect_cps) = aggr_local_first_ps(ctx, calc, item_key, effect, cseq, ospec) {
                        cps += effect_cps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > OF(0.0) => {
                        if let Some(effect_cps) = aggr_local_time_ps(ctx, calc, item_key, effect, cseq, ospec, time) {
                            cps += effect_cps;
                        }
                    }
                    _ => {
                        if let Some(effect_cps) = aggr_local_looped_ps(ctx, calc, item_key, effect, cseq, ospec) {
                            cps += effect_cps;
                        }
                    }
                },
            }
        }
    }
    cps
}

fn get_cap_consumed(
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    fit_data: &VastFitData,
    drains: bool,
    gains: bool,
) -> AttrVal {
    let mut cps = OF(0.0);
    let cycling_options = time_options.into();
    for (&item_key, item_data) in fit_data.cap_consumers_active.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, &attr_key) in item_data.iter() {
            let cap_consumed = match calc.get_item_attr_oextra(ctx, item_key, attr_key) {
                Some(cap_consumed) => cap_consumed,
                None => continue,
            };
            match (cap_consumed.cmp(&OF(0.0)), drains, gains) {
                (Ordering::Greater, true, _) | (Ordering::Less, _, true) => (),
                _ => continue,
            };
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            cps += cap_consumed / cseq.get_average_time();
        }
    }
    cps
}

fn get_cap_transfers(
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    cap_item_key: UItemId,
    vast: &Vast,
) -> AttrVal {
    let mut cps = OF(0.0);
    let cycling_options = time_options.into();
    let transfer_data = match vast.in_cap.get_l1(&cap_item_key) {
        Some(transfer_data) => transfer_data,
        None => return cps,
    };
    for (&transfer_item_key, item_data) in transfer_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, transfer_item_key, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_key);
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_cps) = aggr_proj_first_ps(
                        ctx,
                        calc,
                        transfer_item_key,
                        effect,
                        cseq,
                        ospec,
                        Some(cap_item_key),
                        burst_opts.spool,
                    ) {
                        cps += effect_cps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > OF(0.0) => {
                        if let Some(effect_cps) = aggr_proj_time_ps(
                            ctx,
                            calc,
                            transfer_item_key,
                            effect,
                            cseq,
                            ospec,
                            Some(cap_item_key),
                            time,
                        ) {
                            cps += effect_cps;
                        }
                    }
                    _ => {
                        if let Some(effect_cps) =
                            aggr_proj_looped_ps(ctx, calc, transfer_item_key, effect, cseq, ospec, Some(cap_item_key))
                        {
                            cps += effect_cps;
                        }
                    }
                },
            }
        }
    }
    cps
}

fn get_neuts(
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    cap_item_key: UItemId,
    vast: &Vast,
) -> AttrVal {
    let mut nps = OF(0.0);
    let cycling_options = time_options.into();
    let neut_data = match vast.in_neuts.get_l1(&cap_item_key) {
        Some(neut_data) => neut_data,
        None => return nps,
    };
    for (&neut_item_key, item_data) in neut_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, neut_item_key, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_key);
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_nps) = aggr_proj_first_ps(
                        ctx,
                        calc,
                        neut_item_key,
                        effect,
                        cseq,
                        ospec,
                        Some(cap_item_key),
                        burst_opts.spool,
                    ) {
                        nps += effect_nps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > OF(0.0) => {
                        if let Some(effect_nps) =
                            aggr_proj_time_ps(ctx, calc, neut_item_key, effect, cseq, ospec, Some(cap_item_key), time)
                        {
                            nps += effect_nps;
                        }
                    }
                    _ => {
                        if let Some(effect_nps) =
                            aggr_proj_looped_ps(ctx, calc, neut_item_key, effect, cseq, ospec, Some(cap_item_key))
                        {
                            nps += effect_nps;
                        }
                    }
                },
            }
        }
    }
    nps
}
