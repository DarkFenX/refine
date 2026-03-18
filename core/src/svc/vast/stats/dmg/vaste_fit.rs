use super::breacher::{AppliedBreacherAccum, BreacherAccum};
use crate::{
    misc::DmgKinds,
    num::PValue,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        vast::{
            StatDmg, StatDmgApplied, StatDmgEntry, StatDmgEntryApplied, StatDmgItemKinds, StatTimeOptions, Vast,
            VastFitData,
            aggr::{SeqAccum, aggr_proj_first, aggr_proj_looped, aggr_proj_time},
        },
    },
    ud::{UFitId, UItemId},
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_dmg_raw(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
    ) -> StatDmg {
        let cycling_options = CyclingOptions::from_time_options(time_options);
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = BreacherAccum::new();
        for fit_uid in fit_uids {
            let fit_data = self.get_fit_data(&fit_uid);
            fit_data.fill_stat_dmg_normal(
                ctx,
                calc,
                &mut dps_normal,
                &mut volley_normal,
                item_kinds,
                time_options,
                cycling_options,
                None,
            );
            fit_data.fill_stat_dmg_breacher(ctx, calc, &mut breacher_accum, item_kinds, cycling_options);
        }
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
        StatDmg {
            dps: StatDmgEntry::from_dmgs(dps_normal, dps_breacher),
            volley: StatDmgEntry::from_dmgs(volley_normal, volley_breacher),
        }
    }
    pub(in crate::svc) fn get_stat_fits_dmg_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        let cycling_options = CyclingOptions::from_time_options(time_options);
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = AppliedBreacherAccum::new();
        for fit_uid in fit_uids {
            let fit_data = self.get_fit_data(&fit_uid);
            fit_data.fill_stat_dmg_normal(
                ctx,
                calc,
                &mut dps_normal,
                &mut volley_normal,
                item_kinds,
                time_options,
                cycling_options,
                Some(projectee_uid),
            );
            fit_data.fill_stat_dmg_breacher_applied(
                ctx,
                calc,
                &mut breacher_accum,
                item_kinds,
                cycling_options,
                projectee_uid,
            );
        }
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
        StatDmgApplied {
            dps: StatDmgEntryApplied::from_dmgs(dps_normal, dps_breacher),
            volley: StatDmgEntryApplied::from_dmgs(volley_normal, volley_breacher),
        }
    }
    pub(in crate::svc) fn get_stat_fit_dmg_raw(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
    ) -> StatDmg {
        let cycling_options = CyclingOptions::from_time_options(time_options);
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = BreacherAccum::new();
        let fit_data = self.get_fit_data(&fit_uid);
        fit_data.fill_stat_dmg_normal(
            ctx,
            calc,
            &mut dps_normal,
            &mut volley_normal,
            item_kinds,
            time_options,
            cycling_options,
            None,
        );
        fit_data.fill_stat_dmg_breacher(ctx, calc, &mut breacher_accum, item_kinds, cycling_options);
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
        StatDmg {
            dps: StatDmgEntry::from_dmgs(dps_normal, dps_breacher),
            volley: StatDmgEntry::from_dmgs(volley_normal, volley_breacher),
        }
    }
    pub(in crate::svc) fn get_stat_fit_dmg_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        let cycling_options = CyclingOptions::from_time_options(time_options);
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = AppliedBreacherAccum::new();
        let fit_data = self.get_fit_data(&fit_uid);
        fit_data.fill_stat_dmg_normal(
            ctx,
            calc,
            &mut dps_normal,
            &mut volley_normal,
            item_kinds,
            time_options,
            cycling_options,
            Some(projectee_uid),
        );
        fit_data.fill_stat_dmg_breacher_applied(
            ctx,
            calc,
            &mut breacher_accum,
            item_kinds,
            cycling_options,
            projectee_uid,
        );
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
        StatDmgApplied {
            dps: StatDmgEntryApplied::from_dmgs(dps_normal, dps_breacher),
            volley: StatDmgEntryApplied::from_dmgs(volley_normal, volley_breacher),
        }
    }
    fn internal_get_stat_fit_dmg(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> (DmgKinds<PValue>, DmgKinds<PValue>, BreacherAccum) {
        let cycling_options = CyclingOptions::from_time_options(time_options);
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = BreacherAccum::new();
        self.get_fit_data(&fit_uid).fill_stat_dmg_normal(
            ctx,
            calc,
            &mut dps_normal,
            &mut volley_normal,
            item_kinds,
            time_options,
            cycling_options,
            projectee_uid,
        );
        (dps_normal, volley_normal, breacher_accum)
    }
}

impl VastFitData {
    fn fill_stat_dmg_normal(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<PValue>,
        volley_normal: &mut DmgKinds<PValue>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        cycling_options: CyclingOptions,
        projectee_uid: Option<UItemId>,
    ) {
        for (&item_uid, item_data) in self.dmg_normal.iter() {
            let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
                Some(cseq_map) => cseq_map,
                None => continue,
            };
            let item = ctx.u_data.items.get(item_uid);
            for (&effect_rid, ospec) in item_data.iter() {
                let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
                if !item_kinds.resolve(ctx, item, effect) {
                    continue;
                }
                let cseq = match cseq_map.get(&effect_rid) {
                    Some(cseq) => cseq,
                    None => continue,
                };
                let mut accum = SeqAccum::new_stack_max();
                if match time_options {
                    StatTimeOptions::Burst(burst_opts) => aggr_proj_first(
                        ctx,
                        calc,
                        item_uid,
                        effect,
                        cseq,
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
                            cseq,
                            ospec,
                            projectee_uid,
                            &mut accum,
                            time,
                        ),
                        _ => aggr_proj_looped(ctx, calc, item_uid, effect, cseq, ospec, projectee_uid, &mut accum),
                    },
                } {
                    *volley_normal += accum.instances.max;
                    *dps_normal += accum.get_per_second();
                }
            }
        }
    }
    fn fill_stat_dmg_breacher(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        breacher_accum: &mut BreacherAccum,
        item_kinds: StatDmgItemKinds,
        cycling_options: CyclingOptions,
    ) {
        for (&item_uid, item_data) in self.dmg_breacher.iter() {
            let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
                Some(cseq_map) => cseq_map,
                None => continue,
            };
            let item = ctx.u_data.items.get(item_uid);
            for (&effect_rid, ospec) in item_data.iter() {
                let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
                if !item_kinds.resolve(ctx, item, effect) {
                    continue;
                }
                let cseq = match cseq_map.get(&effect_rid) {
                    Some(cseq) => cseq,
                    None => continue,
                };
                breacher_accum.add(ctx, calc, item_uid, effect, cseq, ospec);
            }
        }
    }
    fn fill_stat_dmg_breacher_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        breacher_accum: &mut AppliedBreacherAccum,
        item_kinds: StatDmgItemKinds,
        cycling_options: CyclingOptions,
        projectee_uid: UItemId,
    ) {
        for (&item_uid, item_data) in self.dmg_breacher.iter() {
            let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
                Some(cseq_map) => cseq_map,
                None => continue,
            };
            let item = ctx.u_data.items.get(item_uid);
            for (&effect_rid, ospec) in item_data.iter() {
                let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
                if !item_kinds.resolve(ctx, item, effect) {
                    continue;
                }
                let cseq = match cseq_map.get(&effect_rid) {
                    Some(cseq) => cseq,
                    None => continue,
                };
                breacher_accum.add(ctx, calc, item_uid, effect, cseq, ospec, projectee_uid);
            }
        }
    }
}
