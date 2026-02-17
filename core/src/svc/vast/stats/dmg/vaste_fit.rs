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
        vast::{
            StatDmg, StatDmgApplied, StatDmgEntry, StatDmgEntryApplied, StatDmgEntryBreacher, StatDmgItemKinds,
            StatTimeOptions, Vast, VastFitData,
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
        let (dps_normal, volley_normal, breacher_accum) =
            self.internal_get_stat_fits_dmg(ctx, calc, fit_uids, item_kinds, time_options, None);
        StatDmg {
            dps: StatDmgEntry::from_dmgs(dps_normal, None),
            volley: StatDmgEntry::from_dmgs(volley_normal, None),
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
        let (dps_normal, volley_normal, breacher_accum) =
            self.internal_get_stat_fits_dmg(ctx, calc, fit_uids, item_kinds, time_options, Some(projectee_uid));
        StatDmgApplied {
            dps: StatDmgEntryApplied::from_dmgs(dps_normal, None),
            volley: StatDmgEntryApplied::from_dmgs(volley_normal, None),
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
        let (dps_normal, volley_normal, breacher_accum) =
            self.internal_get_stat_fit_dmg(ctx, calc, fit_uid, item_kinds, time_options, None);
        StatDmg {
            dps: StatDmgEntry::from_dmgs(dps_normal, None),
            volley: StatDmgEntry::from_dmgs(volley_normal, None),
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
        let (dps_normal, volley_normal, breacher_accum) =
            self.internal_get_stat_fit_dmg(ctx, calc, fit_uid, item_kinds, time_options, Some(projectee_uid));
        StatDmgApplied {
            dps: StatDmgEntryApplied::from_dmgs(dps_normal, None),
            volley: StatDmgEntryApplied::from_dmgs(volley_normal, None),
        }
    }
    fn internal_get_stat_fits_dmg(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> (DmgKinds<PValue>, DmgKinds<PValue>, BreacherAccum) {
        let cycling_options = CyclingOptions::from_time_options(time_options);
        let mut dps_normal = DmgKinds::default();
        let mut volley_normal = DmgKinds::default();
        let mut breacher_accum = BreacherAccum::new();
        for fit_uid in fit_uids {
            self.get_fit_data(&fit_uid).fill_stat_dmg(
                ctx,
                calc,
                &mut dps_normal,
                &mut volley_normal,
                &mut breacher_accum,
                item_kinds,
                time_options,
                cycling_options,
                projectee_uid,
            );
        }
        (dps_normal, volley_normal, breacher_accum)
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
        self.get_fit_data(&fit_uid).fill_stat_dmg(
            ctx,
            calc,
            &mut dps_normal,
            &mut volley_normal,
            &mut breacher_accum,
            item_kinds,
            time_options,
            cycling_options,
            projectee_uid,
        );
        (dps_normal, volley_normal, breacher_accum)
    }
}

impl VastFitData {
    fn fill_stat_dmg(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<PValue>,
        volley_normal: &mut DmgKinds<PValue>,
        breacher_accum: &mut BreacherAccum,
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
}

impl Vast {
    pub(in crate::svc) fn get_stat_fits_dps_raw(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
    ) -> StatDmgEntry {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fits_dps(ctx, calc, fit_uids, item_kinds, reload, spool, None);
        StatDmgEntry::from_dmgs(dps_normal, breacher_accum.get_dps())
    }
    pub(in crate::svc) fn get_stat_fits_dps_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgEntryApplied {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fits_dps(ctx, calc, fit_uids, item_kinds, reload, spool, Some(projectee_uid));
        StatDmgEntryApplied::from_dmgs(
            dps_normal,
            breacher_accum
                .get_dps()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_uid)),
        )
    }
    fn internal_get_stat_fits_dps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_uid: Option<UItemId>,
    ) -> (DmgKinds<PValue>, BreacherAccum) {
        let cycling_options = get_dps_cycling_options(reload);
        let mut dps_normal = DmgKinds::default();
        let mut breacher_accum = BreacherAccum::new();
        for fit_uid in fit_uids {
            self.get_fit_data(&fit_uid).fill_stat_dps(
                ctx,
                calc,
                &mut dps_normal,
                &mut breacher_accum,
                item_kinds,
                cycling_options,
                spool,
                projectee_uid,
            );
        }
        (dps_normal, breacher_accum)
    }
    pub(in crate::svc) fn get_stat_fit_dps_raw(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
    ) -> StatDmgEntry {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fit_dps(ctx, calc, fit_uid, item_kinds, reload, spool, None);
        StatDmgEntry::from_dmgs(dps_normal, breacher_accum.get_dps())
    }
    pub(in crate::svc) fn get_stat_fit_dps_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgEntryApplied {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fit_dps(ctx, calc, fit_uid, item_kinds, reload, spool, Some(projectee_uid));
        StatDmgEntryApplied::from_dmgs(
            dps_normal,
            breacher_accum
                .get_dps()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_uid)),
        )
    }
    fn internal_get_stat_fit_dps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_uid: Option<UItemId>,
    ) -> (DmgKinds<PValue>, BreacherAccum) {
        let mut dps_normal = DmgKinds::default();
        let mut breacher_accum = BreacherAccum::new();
        let cycling_options = get_dps_cycling_options(reload);
        self.get_fit_data(&fit_uid).fill_stat_dps(
            ctx,
            calc,
            &mut dps_normal,
            &mut breacher_accum,
            item_kinds,
            cycling_options,
            spool,
            projectee_uid,
        );
        (dps_normal, breacher_accum)
    }
    pub(in crate::svc) fn get_stat_fits_volley_raw(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
    ) -> StatDmgEntry {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fits_volley(ctx, calc, fit_uids, item_kinds, spool, None);
        StatDmgEntry::from_dmgs(volley_normal, Some(volley_breacher))
    }
    pub(in crate::svc) fn get_stat_fits_volley_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgEntryApplied {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fits_volley(ctx, calc, fit_uids, item_kinds, spool, Some(projectee_uid));
        StatDmgEntryApplied::from_dmgs(
            volley_normal,
            volley_breacher
                .nullified()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_uid)),
        )
    }
    fn internal_get_stat_fits_volley(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_uid: Option<UItemId>,
    ) -> (DmgKinds<PValue>, StatDmgEntryBreacher) {
        let mut volley_normal = DmgKinds::default();
        let mut volley_breacher = StatDmgEntryBreacher::new();
        for fit_uid in fit_uids {
            self.get_fit_data(&fit_uid).fill_stat_volley(
                ctx,
                calc,
                &mut volley_normal,
                &mut volley_breacher,
                item_kinds,
                spool,
                projectee_uid,
            );
        }
        (volley_normal, volley_breacher)
    }
    pub(in crate::svc) fn get_stat_fit_volley_raw(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
    ) -> StatDmgEntry {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fit_volley(ctx, calc, fit_uid, item_kinds, spool, None);
        StatDmgEntry::from_dmgs(volley_normal, Some(volley_breacher))
    }
    pub(in crate::svc) fn get_stat_fit_volley_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgEntryApplied {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fit_volley(ctx, calc, fit_uid, item_kinds, spool, Some(projectee_uid));
        StatDmgEntryApplied::from_dmgs(
            volley_normal,
            volley_breacher
                .nullified()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_uid)),
        )
    }
    fn internal_get_stat_fit_volley(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_uid: Option<UItemId>,
    ) -> (DmgKinds<PValue>, StatDmgEntryBreacher) {
        let mut volley_normal = DmgKinds::default();
        let mut volley_breacher = StatDmgEntryBreacher::new();
        self.get_fit_data(&fit_uid).fill_stat_volley(
            ctx,
            calc,
            &mut volley_normal,
            &mut volley_breacher,
            item_kinds,
            spool,
            projectee_uid,
        );
        (volley_normal, volley_breacher)
    }
}

impl VastFitData {
    fn fill_stat_dps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<PValue>,
        breacher_accum: &mut BreacherAccum,
        item_kinds: StatDmgItemKinds,
        cycling_options: CyclingOptions,
        spool: Option<Spool>,
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
                match cycling_options {
                    CyclingOptions::Burst => {
                        let mut accum = SeqAccum::new_stack();
                        if aggr_proj_first(
                            ctx,
                            calc,
                            item_uid,
                            effect,
                            cseq,
                            ospec,
                            projectee_uid,
                            spool,
                            &mut accum,
                        ) {
                            *dps_normal += accum.get_per_second();
                        }
                    }
                    CyclingOptions::Sim(_) => {
                        let mut accum = SeqAccum::new_stack();
                        if aggr_proj_looped(ctx, calc, item_uid, effect, cseq, ospec, projectee_uid, &mut accum) {
                            *dps_normal += accum.get_per_second();
                        }
                    }
                }
            }
        }
        for (&item_uid, item_data) in self.dmg_breacher.iter() {
            let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
                Some(cseq_map) => cseq_map,
                None => continue,
            };
            let item = ctx.u_data.items.get(item_uid);
            for (&effect_rid, dmg_getter) in item_data.iter() {
                let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
                if !item_kinds.resolve(ctx, item, effect) {
                    continue;
                }
                let cseq = match cseq_map.get(&effect_rid) {
                    Some(cseq) => cseq,
                    None => continue,
                };
                let opc = match dmg_getter(ctx, calc, item_uid, effect, projectee_uid) {
                    Some(opc) => opc,
                    None => continue,
                };
                breacher_accum.add(opc, cseq.convert_and_optimize());
            }
        }
    }
    fn fill_stat_volley(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        volley_normal: &mut DmgKinds<PValue>,
        volley_breacher: &mut StatDmgEntryBreacher,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_uid: Option<UItemId>,
    ) {
        for (&item_uid, item_data) in self.dmg_normal.iter() {
            let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, VOLLEY_CYCLE_OPTIONS, false) {
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
                if aggr_proj_first(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    cseq,
                    ospec,
                    projectee_uid,
                    spool,
                    &mut accum,
                ) {
                    *volley_normal += accum.instances.max;
                }
            }
        }
        for (&item_uid, item_data) in self.dmg_breacher.iter() {
            let cycle_map = match get_item_cseq_map(ctx, calc, item_uid, VOLLEY_CYCLE_OPTIONS, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            let u_item = ctx.u_data.items.get(item_uid);
            for (&effect_rid, dmg_getter) in item_data.iter() {
                let r_effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
                if !item_kinds.resolve(ctx, u_item, r_effect) {
                    continue;
                }
                let opc = match dmg_getter(ctx, calc, item_uid, r_effect, projectee_uid) {
                    Some(opc) => opc,
                    None => continue,
                };
                if !cycle_map.contains_key(&effect_rid) {
                    continue;
                };
                volley_breacher.stack_instance_output(opc);
            }
        }
    }
}
