use super::{
    breacher::{AppliedBreacherAccum, BreacherAccum, RegularBreacherAccum},
    shared::add_normal_dmg,
};
use crate::{
    PValue,
    misc::DmgKinds,
    stats::{
        StatCritOptions, StatDmg, StatDmgApplied, StatDmgEntry, StatDmgEntryApplied, StatItemChargeOptions,
        StatTimeOptions,
    },
    svc::{
        Calc, SvcCtx, Vast,
        cycle::{CseqCache, CyclingOptions, get_item_cseq_map},
        err::IntStatItemError,
        vast::stats::item_checks::check_autocharge_charge_drone_fighter_module,
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_dmg_raw(
        cseq_cache: &mut CseqCache,
        ctx: &SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        charge_options: StatItemChargeOptions,
    ) -> Result<StatDmg, IntStatItemError<!>> {
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = RegularBreacherAccum::new();
        Self::internal_get_stat_item_dmg_checked(
            cseq_cache,
            ctx,
            calc,
            &mut dps_normal,
            &mut volley_normal,
            &mut breacher_accum,
            item_uid,
            time_options,
            crit_options,
            charge_options,
            None,
        )?;
        let (dps_breacher, volley_breacher) = breacher_accum.get_dps_and_volley(time_options);
        Ok(StatDmg {
            dps: StatDmgEntry::from_dmgs(dps_normal, dps_breacher),
            volley: StatDmgEntry::from_dmgs(volley_normal, volley_breacher),
        })
    }
    pub(in crate::svc) fn get_stat_item_dmg_applied(
        cseq_cache: &mut CseqCache,
        ctx: &SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        charge_options: StatItemChargeOptions,
        projectee_uid: UItemId,
    ) -> Result<StatDmgApplied, IntStatItemError<!>> {
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = AppliedBreacherAccum::new(crit_options, projectee_uid);
        Self::internal_get_stat_item_dmg_checked(
            cseq_cache,
            ctx,
            calc,
            &mut dps_normal,
            &mut volley_normal,
            &mut breacher_accum,
            item_uid,
            time_options,
            crit_options,
            charge_options,
            Some(projectee_uid),
        )?;
        let (dps_breacher, volley_breacher) = breacher_accum.get_dps_and_volley(time_options);
        Ok(StatDmgApplied {
            dps: StatDmgEntryApplied::from_dmgs(dps_normal, dps_breacher),
            volley: StatDmgEntryApplied::from_dmgs(volley_normal, volley_breacher),
        })
    }
    fn internal_get_stat_item_dmg_checked<B>(
        cseq_cache: &mut CseqCache,
        ctx: &SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<PValue>,
        volley_normal: &mut DmgKinds<PValue>,
        breacher_accum: &mut B,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        charge_options: StatItemChargeOptions,
        projectee_uid: Option<UItemId>,
    ) -> Result<(), IntStatItemError<!>>
    where
        B: BreacherAccum,
    {
        check_autocharge_charge_drone_fighter_module(ctx.u_data, item_uid)?;
        let cycling_options = CyclingOptions::from_time_options(time_options);
        if !get_item_cseq_map(cseq_cache, ctx, calc, item_uid, cycling_options) {
            return Ok(());
        }
        for (&effect_rid, cseq) in cseq_cache.iter() {
            let effect = ctx.u_data.r_data.get_effect_by_rid(effect_rid);
            if let Some(ospec) = &effect.normal_dmg {
                add_normal_dmg(
                    ctx,
                    calc,
                    dps_normal,
                    volley_normal,
                    item_uid,
                    effect,
                    cseq,
                    ospec,
                    time_options,
                    crit_options,
                    projectee_uid,
                );
            }
            if let Some(ospec) = &effect.breacher_dmg {
                breacher_accum.add(ctx, calc, item_uid, effect, cseq, ospec);
            }
        }
        if charge_options.is_enabled() {
            for charge_uid in ctx.u_data.items.get(item_uid).iter_charges() {
                let _ = Self::internal_get_stat_item_dmg_checked(
                    cseq_cache,
                    ctx,
                    calc,
                    dps_normal,
                    volley_normal,
                    breacher_accum,
                    charge_uid,
                    time_options,
                    crit_options,
                    StatItemChargeOptions::Exclude,
                    projectee_uid,
                );
            }
        }
        Ok(())
    }
}
