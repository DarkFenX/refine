use super::{
    breacher::{BreacherAccum, apply_breacher},
    shared::{VOLLEY_CYCLE_OPTIONS, get_dps_cycling_options},
};
use crate::{
    misc::{DmgKinds, Spool},
    num::PValue,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            StatDmg, StatDmgApplied, StatDmgEntry, StatDmgEntryApplied, StatDmgEntryBreacher, StatTimeOptions, Vast,
            aggr::{SeqAccum, aggr_proj_first, aggr_proj_looped, aggr_proj_time},
            stats::item_checks::check_autocharge_charge_drone_fighter_module,
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_dmg_raw(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        let (dps_normal, volley_normal, breacher_accum) =
            Vast::internal_get_stat_item_dmg(ctx, calc, item_uid, time_options, include_charges, ignore_state, None)?;
        Ok(StatDmg {
            dps: StatDmgEntry::from_dmgs(dps_normal, None),
            volley: StatDmgEntry::from_dmgs(volley_normal, None),
        })
    }
    pub(in crate::svc) fn get_stat_item_dmg_applied(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: UItemId,
    ) -> Result<StatDmgApplied, StatItemCheckError> {
        let (dps_normal, volley_normal, breacher_accum) = Vast::internal_get_stat_item_dmg(
            ctx,
            calc,
            item_uid,
            time_options,
            include_charges,
            ignore_state,
            Some(projectee_uid),
        )?;
        Ok(StatDmgApplied {
            dps: StatDmgEntryApplied::from_dmgs(dps_normal, None),
            volley: StatDmgEntryApplied::from_dmgs(volley_normal, None),
        })
    }
    fn internal_get_stat_item_dmg(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: Option<UItemId>,
    ) -> Result<(DmgKinds<PValue>, DmgKinds<PValue>, BreacherAccum), StatItemCheckError> {
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = BreacherAccum::new();
        Vast::internal_get_stat_item_dmg_checked(
            ctx,
            calc,
            &mut dps_normal,
            &mut volley_normal,
            &mut breacher_accum,
            item_uid,
            time_options,
            include_charges,
            ignore_state,
            projectee_uid,
        )?;
        Ok((dps_normal, volley_normal, breacher_accum))
    }
    fn internal_get_stat_item_dmg_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<PValue>,
        volley_normal: &mut DmgKinds<PValue>,
        breacher_accum: &mut BreacherAccum,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: Option<UItemId>,
    ) -> Result<(), StatItemCheckError> {
        check_autocharge_charge_drone_fighter_module(ctx.u_data, item_uid)?;
        let cycling_options = CyclingOptions::from_time_options(time_options);
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, ignore_state) {
            Some(cseq_map) => cseq_map,
            None => return Ok(()),
        };
        for (effect_rid, cseq) in cseq_map {
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(ospec) = &effect.normal_dmg_opc_spec {
                let mut accum = SeqAccum::new_stack_max();
                if match time_options {
                    StatTimeOptions::Burst(burst_opts) => aggr_proj_first(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        &cseq,
                        ospec,
                        projectee_uid,
                        burst_opts.spool,
                        &mut accum,
                    ),
                    StatTimeOptions::Sim(sim_options) => match sim_options.time {
                        Some(time) if time > PValue::ZERO => aggr_proj_time(
                            ctx,
                            calc,
                            item_uid,
                            effect,
                            &cseq,
                            ospec,
                            projectee_uid,
                            &mut accum,
                            time,
                        ),
                        _ => aggr_proj_looped(ctx, calc, item_uid, effect, &cseq, ospec, projectee_uid, &mut accum),
                    },
                } {
                    *volley_normal += accum.instances.max;
                    *dps_normal += accum.get_per_second();
                }
            }
            if let Some(dmg_getter) = effect.breacher_dmg_opc_getter
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_uid, effect, projectee_uid)
            {
                breacher_accum.add(dmg_opc, cseq.convert_and_optimize());
            }
        }
        if include_charges {
            for charge_uid in ctx.u_data.items.get(item_uid).iter_charges() {
                let _ = Vast::internal_get_stat_item_dmg_checked(
                    ctx,
                    calc,
                    dps_normal,
                    volley_normal,
                    breacher_accum,
                    charge_uid,
                    time_options,
                    false,
                    ignore_state,
                    projectee_uid,
                );
            }
        }
        Ok(())
    }
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_dps_raw(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmgEntry, StatItemCheckError> {
        let (dps_normal, breacher_accum) =
            Vast::internal_get_stat_item_dps(ctx, calc, item_uid, reload, spool, include_charges, ignore_state, None)?;
        Ok(StatDmgEntry::from_dmgs(dps_normal, breacher_accum.get_dps()))
    }
    pub(in crate::svc) fn get_stat_item_dps_applied(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: UItemId,
    ) -> Result<StatDmgEntryApplied, StatItemCheckError> {
        let (dps_normal, breacher_accum) = Vast::internal_get_stat_item_dps(
            ctx,
            calc,
            item_uid,
            reload,
            spool,
            include_charges,
            ignore_state,
            Some(projectee_uid),
        )?;
        Ok(StatDmgEntryApplied::from_dmgs(
            dps_normal,
            breacher_accum
                .get_dps()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_uid)),
        ))
    }
    fn internal_get_stat_item_dps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: Option<UItemId>,
    ) -> Result<(DmgKinds<PValue>, BreacherAccum), StatItemCheckError> {
        let mut dps_normal = DmgKinds::default();
        let mut breacher_accum = BreacherAccum::new();
        Vast::internal_get_stat_item_dps_checked(
            ctx,
            calc,
            &mut dps_normal,
            &mut breacher_accum,
            item_uid,
            reload,
            spool,
            include_charges,
            ignore_state,
            projectee_uid,
        )?;
        Ok((dps_normal, breacher_accum))
    }
    fn internal_get_stat_item_dps_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<PValue>,
        breacher_accum: &mut BreacherAccum,
        item_uid: UItemId,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: Option<UItemId>,
    ) -> Result<(), StatItemCheckError> {
        check_autocharge_charge_drone_fighter_module(ctx.u_data, item_uid)?;
        let options = get_dps_cycling_options(reload);
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, options, ignore_state) {
            Some(cseq_map) => cseq_map,
            None => return Ok(()),
        };
        for (effect_rid, cseq) in cseq_map {
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(ospec) = effect.normal_dmg_opc_spec {
                match reload {
                    true => {
                        let mut accum = SeqAccum::new_stack();
                        if aggr_proj_looped(ctx, calc, item_uid, effect, &cseq, &ospec, projectee_uid, &mut accum) {
                            *dps_normal += accum.get_per_second();
                        }
                    }
                    false => {
                        let mut accum = SeqAccum::new_stack();
                        if aggr_proj_first(
                            ctx,
                            calc,
                            item_uid,
                            effect,
                            &cseq,
                            &ospec,
                            projectee_uid,
                            spool,
                            &mut accum,
                        ) {
                            *dps_normal += accum.get_per_second();
                        }
                    }
                }
            }
            if let Some(dmg_getter) = effect.breacher_dmg_opc_getter
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_uid, effect, projectee_uid)
            {
                breacher_accum.add(dmg_opc, cseq.convert_and_optimize());
            }
        }
        if include_charges {
            for charge_uid in ctx.u_data.items.get(item_uid).iter_charges() {
                let _ = Vast::internal_get_stat_item_dps_checked(
                    ctx,
                    calc,
                    dps_normal,
                    breacher_accum,
                    charge_uid,
                    reload,
                    spool,
                    false,
                    ignore_state,
                    projectee_uid,
                );
            }
        }
        Ok(())
    }
    pub(in crate::svc) fn get_stat_item_volley_raw(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmgEntry, StatItemCheckError> {
        let (volley_normal, volley_breacher) =
            Vast::internal_get_stat_item_volley(ctx, calc, item_uid, spool, include_charges, ignore_state, None)?;
        Ok(StatDmgEntry::from_dmgs(volley_normal, Some(volley_breacher)))
    }
    pub(in crate::svc) fn get_stat_item_volley_applied(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: UItemId,
    ) -> Result<StatDmgEntryApplied, StatItemCheckError> {
        let (volley_normal, volley_breacher) = Vast::internal_get_stat_item_volley(
            ctx,
            calc,
            item_uid,
            spool,
            include_charges,
            ignore_state,
            Some(projectee_uid),
        )?;
        Ok(StatDmgEntryApplied::from_dmgs(
            volley_normal,
            volley_breacher
                .nullified()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_uid)),
        ))
    }
    fn internal_get_stat_item_volley(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: Option<UItemId>,
    ) -> Result<(DmgKinds<PValue>, StatDmgEntryBreacher), StatItemCheckError> {
        let mut volley_normal = DmgKinds::default();
        let mut volley_breacher = StatDmgEntryBreacher::new();
        Vast::internal_get_stat_item_volley_checked(
            ctx,
            calc,
            &mut volley_normal,
            &mut volley_breacher,
            item_uid,
            spool,
            include_charges,
            ignore_state,
            projectee_uid,
        )?;
        Ok((volley_normal, volley_breacher))
    }
    fn internal_get_stat_item_volley_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        volley_normal: &mut DmgKinds<PValue>,
        volley_breacher: &mut StatDmgEntryBreacher,
        item_uid: UItemId,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: Option<UItemId>,
    ) -> Result<(), StatItemCheckError> {
        check_autocharge_charge_drone_fighter_module(ctx.u_data, item_uid)?;
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, VOLLEY_CYCLE_OPTIONS, ignore_state) {
            Some(cseq_map) => cseq_map,
            None => return Ok(()),
        };
        for (effect_rid, cseq) in cseq_map {
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(ospec) = &effect.normal_dmg_opc_spec {
                let mut accum = SeqAccum::new_stack_max();
                if aggr_proj_first(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    &cseq,
                    ospec,
                    projectee_uid,
                    spool,
                    &mut accum,
                ) {
                    *volley_normal += accum.instances.max;
                }
            }
            if let Some(dmg_getter) = effect.breacher_dmg_opc_getter
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_uid, effect, projectee_uid)
            {
                volley_breacher.stack_instance_output(dmg_opc);
            }
        }
        if include_charges {
            for charge_uid in ctx.u_data.items.get(item_uid).iter_charges() {
                let _ = Vast::internal_get_stat_item_volley_checked(
                    ctx,
                    calc,
                    volley_normal,
                    volley_breacher,
                    charge_uid,
                    spool,
                    false,
                    ignore_state,
                    projectee_uid,
                );
            }
        }
        Ok(())
    }
}
