use super::shared::{VOLLEY_CYCLE_OPTIONS, get_dps_cycle_options};
use crate::{
    def::AttrVal,
    misc::{DmgKinds, Spool},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::get_item_cycle_info,
        err::StatItemCheckError,
        vast::{
            StatDmg, StatDmgApplied, StatDmgBreacher, Vast,
            shared::{BreacherAccum, apply_breacher},
            vaste_stats::item_checks::check_autocharge_charge_drone_fighter_module,
        },
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_dps_raw(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        let (dps_normal, breacher_accum) =
            Vast::internal_get_stat_item_dps(ctx, calc, item_key, reload, spool, include_charges, ignore_state, None)?;
        Ok(StatDmg::from((dps_normal, breacher_accum.get_dps())))
    }
    pub(in crate::svc) fn get_stat_item_dps_applied(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: UItemKey,
    ) -> Result<StatDmgApplied, StatItemCheckError> {
        let (dps_normal, breacher_accum) = Vast::internal_get_stat_item_dps(
            ctx,
            calc,
            item_key,
            reload,
            spool,
            include_charges,
            ignore_state,
            Some(projectee_key),
        )?;
        Ok(StatDmgApplied::from((
            dps_normal,
            breacher_accum
                .get_dps()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_key)),
        )))
    }
    fn internal_get_stat_item_dps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: Option<UItemKey>,
    ) -> Result<(DmgKinds<AttrVal>, BreacherAccum), StatItemCheckError> {
        let mut dps_normal = DmgKinds::new();
        let mut breacher_accum = BreacherAccum::new();
        Vast::internal_get_stat_item_dps_checked(
            ctx,
            calc,
            &mut dps_normal,
            &mut breacher_accum,
            item_key,
            reload,
            spool,
            include_charges,
            ignore_state,
            projectee_key,
        )?;
        Ok((dps_normal, breacher_accum))
    }
    fn internal_get_stat_item_dps_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<AttrVal>,
        breacher_accum: &mut BreacherAccum,
        item_key: UItemKey,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: Option<UItemKey>,
    ) -> Result<(), StatItemCheckError> {
        check_autocharge_charge_drone_fighter_module(ctx.u_data, item_key)?;
        let options = get_dps_cycle_options(reload);
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, options, ignore_state) {
            Some(cycle_map) => cycle_map,
            None => return Ok(()),
        };
        for (effect_key, cycle) in cycle_map {
            let effect = ctx.u_data.src.get_effect(effect_key);
            if let Some(dmg_getter) = effect.normal_dmg_opc_getter
                && let Some(cycle_looped) = cycle.get_looped_part()
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_key, effect, spool, projectee_key)
            {
                *dps_normal += dmg_opc.get_total() / cycle_looped.get_average_cycle_time();
            }
            if let Some(dmg_getter) = effect.breacher_dmg_opc_getter
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_key, effect, projectee_key)
            {
                breacher_accum.add(dmg_opc, cycle);
            }
        }
        if include_charges {
            for charge_key in ctx.u_data.items.get(item_key).iter_charges() {
                let _ = Vast::internal_get_stat_item_dps_checked(
                    ctx,
                    calc,
                    dps_normal,
                    breacher_accum,
                    charge_key,
                    reload,
                    spool,
                    false,
                    ignore_state,
                    projectee_key,
                );
            }
        }
        Ok(())
    }
    pub(in crate::svc) fn get_stat_item_volley_raw(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        let (volley_normal, volley_breacher) =
            Vast::internal_get_stat_item_volley(ctx, calc, item_key, spool, include_charges, ignore_state, None)?;
        Ok(StatDmg::from((volley_normal, Some(volley_breacher))))
    }
    pub(in crate::svc) fn get_stat_item_volley_applied(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: UItemKey,
    ) -> Result<StatDmgApplied, StatItemCheckError> {
        let (volley_normal, volley_breacher) = Vast::internal_get_stat_item_volley(
            ctx,
            calc,
            item_key,
            spool,
            include_charges,
            ignore_state,
            Some(projectee_key),
        )?;
        Ok(StatDmgApplied::from((
            volley_normal,
            volley_breacher
                .nullified()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_key)),
        )))
    }
    fn internal_get_stat_item_volley(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: Option<UItemKey>,
    ) -> Result<(DmgKinds<AttrVal>, StatDmgBreacher), StatItemCheckError> {
        let mut volley_normal = DmgKinds::new();
        let mut volley_breacher = StatDmgBreacher::new();
        Vast::internal_get_stat_item_volley_checked(
            ctx,
            calc,
            &mut volley_normal,
            &mut volley_breacher,
            item_key,
            spool,
            include_charges,
            ignore_state,
            projectee_key,
        )?;
        Ok((volley_normal, volley_breacher))
    }
    fn internal_get_stat_item_volley_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        volley_normal: &mut DmgKinds<AttrVal>,
        volley_breacher: &mut StatDmgBreacher,
        item_key: UItemKey,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: Option<UItemKey>,
    ) -> Result<(), StatItemCheckError> {
        check_autocharge_charge_drone_fighter_module(ctx.u_data, item_key)?;
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, VOLLEY_CYCLE_OPTIONS, ignore_state) {
            Some(cycle_map) => cycle_map,
            None => return Ok(()),
        };
        for (effect_key, _cycle) in cycle_map {
            let effect = ctx.u_data.src.get_effect(effect_key);
            if let Some(dmg_getter) = effect.normal_dmg_opc_getter
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_key, effect, spool, projectee_key)
            {
                *volley_normal += dmg_opc.get_amount();
            }
            if let Some(dmg_getter) = effect.breacher_dmg_opc_getter
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_key, effect, projectee_key)
            {
                volley_breacher.stack_instance_output(dmg_opc);
            }
        }
        if include_charges {
            for charge_key in ctx.u_data.items.get(item_key).iter_charges() {
                let _ = Vast::internal_get_stat_item_volley_checked(
                    ctx,
                    calc,
                    volley_normal,
                    volley_breacher,
                    charge_key,
                    spool,
                    false,
                    ignore_state,
                    projectee_key,
                );
            }
        }
        Ok(())
    }
}
