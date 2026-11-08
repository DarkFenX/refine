use std::iter::once;

use super::{
    breacher::{AppliedBreacherAccum, BreacherAccum, RegularBreacherAccum},
    shared::add_normal_dmg,
};
use crate::{
    PValue,
    misc::DmgKinds,
    stats::{
        StatCritOptions, StatDmg, StatDmgApplied, StatDmgEntry, StatDmgEntryApplied, StatDmgItemKinds, StatTimeOptions,
    },
    svc::{
        Calc, SvcCtx, Vast,
        cycle::{CseqCache, CyclingOptions, get_item_cseq_map},
        vast::VastFitData,
    },
    ud::{UFitId, UItemId},
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_dmg_raw(
        &self,
        cseq_cache: &mut CseqCache,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit_uids: impl Iterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
    ) -> StatDmg {
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = RegularBreacherAccum::new();
        self.fill_stat_fits_dmg(
            cseq_cache,
            ctx,
            calc,
            &mut dps_normal,
            &mut volley_normal,
            &mut breacher_accum,
            fit_uids,
            item_kinds,
            time_options,
            crit_options,
            None,
        );
        let (dps_breacher, volley_breacher) = breacher_accum.get_dps_and_volley(time_options);
        StatDmg {
            dps: StatDmgEntry::from_dmgs(dps_normal, dps_breacher),
            volley: StatDmgEntry::from_dmgs(volley_normal, volley_breacher),
        }
    }
    pub(in crate::svc) fn get_stat_fits_dmg_applied(
        &self,
        cseq_cache: &mut CseqCache,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit_uids: impl Iterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = AppliedBreacherAccum::new(crit_options, projectee_uid);
        self.fill_stat_fits_dmg(
            cseq_cache,
            ctx,
            calc,
            &mut dps_normal,
            &mut volley_normal,
            &mut breacher_accum,
            fit_uids,
            item_kinds,
            time_options,
            crit_options,
            Some(projectee_uid),
        );
        let (dps_breacher, volley_breacher) = breacher_accum.get_dps_and_volley(time_options);
        StatDmgApplied {
            dps: StatDmgEntryApplied::from_dmgs(dps_normal, dps_breacher),
            volley: StatDmgEntryApplied::from_dmgs(volley_normal, volley_breacher),
        }
    }
    pub(in crate::svc) fn get_stat_fit_dmg_raw(
        &self,
        cseq_cache: &mut CseqCache,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
    ) -> StatDmg {
        self.get_stat_fits_dmg_raw(
            cseq_cache,
            ctx,
            calc,
            once(fit_uid),
            item_kinds,
            time_options,
            crit_options,
        )
    }
    pub(in crate::svc) fn get_stat_fit_dmg_applied(
        &self,
        cseq_cache: &mut CseqCache,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        self.get_stat_fits_dmg_applied(
            cseq_cache,
            ctx,
            calc,
            once(fit_uid),
            item_kinds,
            time_options,
            crit_options,
            projectee_uid,
        )
    }
    fn fill_stat_fits_dmg<I, B>(
        &self,
        cseq_cache: &mut CseqCache,
        ctx: &SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<PValue>,
        volley_normal: &mut DmgKinds<PValue>,
        breacher_accum: &mut B,
        fit_uids: I,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        projectee_uid: Option<UItemId>,
    ) where
        I: Iterator<Item = UFitId>,
        B: BreacherAccum,
    {
        let cycling_options = CyclingOptions::from_time_options(time_options);
        for fit_uid in fit_uids {
            let fit_data = self.get_fit_data(fit_uid);
            fit_data.fill_stat_dmg_normal(
                cseq_cache,
                ctx,
                calc,
                dps_normal,
                volley_normal,
                item_kinds,
                time_options,
                crit_options,
                cycling_options,
                projectee_uid,
            );
            fit_data.fill_stat_dmg_breacher(cseq_cache, ctx, calc, breacher_accum, item_kinds, cycling_options);
        }
    }
}

impl VastFitData {
    fn fill_stat_dmg_normal(
        &self,
        cseq_cache: &mut CseqCache,
        ctx: &SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<PValue>,
        volley_normal: &mut DmgKinds<PValue>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        cycling_options: CyclingOptions,
        projectee_uid: Option<UItemId>,
    ) {
        for (&item_uid, item_data) in self.dmg_normal.iter() {
            let Some(cseq_map) = get_item_cseq_map(cseq_cache, ctx, calc, item_uid, cycling_options) else {
                continue;
            };
            let item = ctx.u_data.items.get(item_uid);
            for (&effect_rid, ospec) in item_data.iter() {
                let effect = ctx.u_data.r_data.get_effect_by_rid(effect_rid);
                if !item_kinds.resolve(item, effect) {
                    continue;
                }
                let Some(cseq) = cseq_map.get(effect_rid) else {
                    continue;
                };
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
        }
    }
    fn fill_stat_dmg_breacher<B>(
        &self,
        cseq_cache: &mut CseqCache,
        ctx: &SvcCtx,
        calc: &mut Calc,
        breacher_accum: &mut B,
        item_kinds: StatDmgItemKinds,
        cycling_options: CyclingOptions,
    ) where
        B: BreacherAccum,
    {
        for (&item_uid, item_data) in self.dmg_breacher.iter() {
            let Some(cseq_map) = get_item_cseq_map(cseq_cache, ctx, calc, item_uid, cycling_options) else {
                continue;
            };
            let item = ctx.u_data.items.get(item_uid);
            for (&effect_rid, ospec) in item_data.iter() {
                let effect = ctx.u_data.r_data.get_effect_by_rid(effect_rid);
                if !item_kinds.resolve(item, effect) {
                    continue;
                }
                let Some(cseq) = cseq_map.get(effect_rid) else {
                    continue;
                };
                breacher_accum.add(ctx, calc, item_uid, effect, cseq, ospec);
            }
        }
    }
}
