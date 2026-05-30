use super::breacher::{AppliedBreacherAccum, BreacherAccum};
use crate::{
    misc::DmgKinds,
    num::PValue,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CseqMap, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            StatDmg, StatDmgApplied, StatDmgEntry, StatDmgEntryApplied, StatTimeOptions, Vast,
            aggr::{SeqAccum, aggr_proj_burst, aggr_proj_looped, aggr_proj_time},
            stats::item_checks::check_autocharge_charge_drone_fighter_module,
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_dmg_raw(
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = BreacherAccum::new();
        Self::internal_get_stat_item_dmg_checked(
            reuse_cseq_map,
            ctx,
            calc,
            &mut dps_normal,
            &mut volley_normal,
            &mut breacher_accum,
            item_uid,
            time_options,
            include_charges,
        )?;
        let (dps_breacher, volley_breacher) = match time_options {
            StatTimeOptions::Burst(_) => (breacher_accum.get_dps(), breacher_accum.get_volley()),
            StatTimeOptions::Sim(sim_options) => match sim_options.time {
                Some(time) if time > PValue::ZERO => (
                    breacher_accum.get_dps_by_time(time),
                    breacher_accum.get_volley_by_time(time),
                ),
                _ => (breacher_accum.get_dps(), breacher_accum.get_volley()),
            },
        };
        Ok(StatDmg {
            dps: StatDmgEntry::from_dmgs(dps_normal, dps_breacher),
            volley: StatDmgEntry::from_dmgs(volley_normal, volley_breacher),
        })
    }
    pub(in crate::svc) fn get_stat_item_dmg_applied(
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
        projectee_uid: UItemId,
    ) -> Result<StatDmgApplied, StatItemCheckError> {
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = AppliedBreacherAccum::new();
        Self::internal_get_stat_item_dmg_applied_checked(
            reuse_cseq_map,
            ctx,
            calc,
            &mut dps_normal,
            &mut volley_normal,
            &mut breacher_accum,
            item_uid,
            time_options,
            include_charges,
            projectee_uid,
        )?;
        let (dps_breacher, volley_breacher) = match time_options {
            StatTimeOptions::Burst(_) => (breacher_accum.get_dps(), breacher_accum.get_volley()),
            StatTimeOptions::Sim(sim_options) => match sim_options.time {
                Some(time) if time > PValue::ZERO => (
                    breacher_accum.get_dps_by_time(time),
                    breacher_accum.get_volley_by_time(time),
                ),
                _ => (breacher_accum.get_dps(), breacher_accum.get_volley()),
            },
        };
        Ok(StatDmgApplied {
            dps: StatDmgEntryApplied::from_dmgs(dps_normal, dps_breacher),
            volley: StatDmgEntryApplied::from_dmgs(volley_normal, volley_breacher),
        })
    }
    fn internal_get_stat_item_dmg_checked(
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<PValue>,
        volley_normal: &mut DmgKinds<PValue>,
        breacher_accum: &mut BreacherAccum,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
    ) -> Result<(), StatItemCheckError> {
        check_autocharge_charge_drone_fighter_module(ctx.u_data, item_uid)?;
        let cycling_options = CyclingOptions::from_time_options(time_options);
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, cycling_options) {
            return Ok(());
        }
        for (&effect_rid, cseq) in reuse_cseq_map.iter() {
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(ospec) = &effect.normal_dmg {
                let mut accum = SeqAccum::new_stack_max();
                if match time_options {
                    StatTimeOptions::Burst(burst_opts) => aggr_proj_burst(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        None,
                        burst_opts.spool,
                        &mut accum,
                    ),
                    StatTimeOptions::Sim(sim_options) => match sim_options.time {
                        Some(time) if time > PValue::ZERO => {
                            aggr_proj_time(ctx, calc, item_uid, effect, cseq, ospec, (), None, &mut accum, time)
                        }
                        _ => aggr_proj_looped(ctx, calc, item_uid, effect, cseq, ospec, (), None, &mut accum),
                    },
                } {
                    *volley_normal += accum.instances.max;
                    *dps_normal += accum.get_per_second();
                }
            }
            if let Some(ospec) = &effect.breacher_dmg {
                breacher_accum.add(ctx, calc, item_uid, effect, cseq, ospec);
            }
        }
        if include_charges {
            for charge_uid in ctx.u_data.items.get(item_uid).iter_charges() {
                let _ = Self::internal_get_stat_item_dmg_checked(
                    reuse_cseq_map,
                    ctx,
                    calc,
                    dps_normal,
                    volley_normal,
                    breacher_accum,
                    charge_uid,
                    time_options,
                    false,
                );
            }
        }
        Ok(())
    }
    fn internal_get_stat_item_dmg_applied_checked(
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<PValue>,
        volley_normal: &mut DmgKinds<PValue>,
        breacher_accum: &mut AppliedBreacherAccum,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
        projectee_uid: UItemId,
    ) -> Result<(), StatItemCheckError> {
        check_autocharge_charge_drone_fighter_module(ctx.u_data, item_uid)?;
        let cycling_options = CyclingOptions::from_time_options(time_options);
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, cycling_options) {
            return Ok(());
        }
        for (&effect_rid, cseq) in reuse_cseq_map.iter() {
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(ospec) = &effect.normal_dmg {
                let mut accum = SeqAccum::new_stack_max();
                if match time_options {
                    StatTimeOptions::Burst(burst_opts) => aggr_proj_burst(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        Some(projectee_uid),
                        burst_opts.spool,
                        &mut accum,
                    ),
                    StatTimeOptions::Sim(sim_options) => match sim_options.time {
                        Some(time) if time > PValue::ZERO => aggr_proj_time(
                            ctx,
                            calc,
                            item_uid,
                            effect,
                            cseq,
                            ospec,
                            (),
                            Some(projectee_uid),
                            &mut accum,
                            time,
                        ),
                        _ => aggr_proj_looped(
                            ctx,
                            calc,
                            item_uid,
                            effect,
                            cseq,
                            ospec,
                            (),
                            Some(projectee_uid),
                            &mut accum,
                        ),
                    },
                } {
                    *volley_normal += accum.instances.max;
                    *dps_normal += accum.get_per_second();
                }
            }
            if let Some(ospec) = &effect.breacher_dmg {
                breacher_accum.add(ctx, calc, item_uid, effect, cseq, ospec, projectee_uid);
            }
        }
        if include_charges {
            for charge_uid in ctx.u_data.items.get(item_uid).iter_charges() {
                let _ = Self::internal_get_stat_item_dmg_applied_checked(
                    reuse_cseq_map,
                    ctx,
                    calc,
                    dps_normal,
                    volley_normal,
                    breacher_accum,
                    charge_uid,
                    time_options,
                    false,
                    projectee_uid,
                );
            }
        }
        Ok(())
    }
}
