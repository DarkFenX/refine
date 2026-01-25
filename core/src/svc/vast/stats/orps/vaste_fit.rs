use super::stat::StatOutReps;
use crate::{
    num::PValue,
    rd::{REffectId, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        vast::{
            StatOutRepItemKinds, StatTimeOptions, Vast,
            aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        },
    },
    ud::{UFitId, UItemId},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_outgoing_rps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> StatOutReps {
        let mut shield = PValue::ZERO;
        let mut armor = PValue::ZERO;
        let mut hull = PValue::ZERO;
        for fit_uid in fit_uids {
            let fit_data = self.get_fit_data(&fit_uid);
            shield += get_orps(ctx, calc, item_kinds, time_options, projectee_uid, &fit_data.orr_shield);
            armor += get_orps(ctx, calc, item_kinds, time_options, projectee_uid, &fit_data.orr_armor);
            hull += get_orps(ctx, calc, item_kinds, time_options, projectee_uid, &fit_data.orr_hull);
        }
        StatOutReps { shield, armor, hull }
    }
    pub(in crate::svc) fn get_stat_fit_outgoing_rps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> StatOutReps {
        let fit_data = self.get_fit_data(&fit_uid);
        StatOutReps {
            shield: get_orps(ctx, calc, item_kinds, time_options, projectee_uid, &fit_data.orr_shield),
            armor: get_orps(ctx, calc, item_kinds, time_options, projectee_uid, &fit_data.orr_armor),
            hull: get_orps(ctx, calc, item_kinds, time_options, projectee_uid, &fit_data.orr_hull),
        }
    }
}

fn get_orps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatOutRepItemKinds,
    time_options: StatTimeOptions,
    projectee_uid: Option<UItemId>,
    fit_data: &RMapRMap<UItemId, REffectId, REffectProjOpcSpec<PValue>>,
) -> PValue {
    let mut orps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&item_uid, item_data) in fit_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        let u_item = ctx.u_data.items.get(item_uid);
        if !item_kinds.resolve(u_item) {
            continue;
        }
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_orps) = aggr_proj_first_ps(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        cseq,
                        ospec,
                        projectee_uid,
                        burst_opts.spool,
                    ) {
                        orps += effect_orps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => {
                        if let Some(effect_orps) =
                            aggr_proj_time_ps(ctx, calc, item_uid, effect, cseq, ospec, projectee_uid, time)
                        {
                            orps += effect_orps;
                        }
                    }
                    _ => {
                        if let Some(effect_orps) =
                            aggr_proj_looped_ps(ctx, calc, item_uid, effect, cseq, ospec, projectee_uid)
                        {
                            orps += effect_orps;
                        }
                    }
                },
            }
        }
    }
    orps
}
