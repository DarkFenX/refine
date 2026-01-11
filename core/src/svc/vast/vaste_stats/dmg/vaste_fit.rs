use super::{
    breacher::{BreacherAccum, apply_breacher},
    shared::{VOLLEY_CYCLE_OPTIONS, get_dps_cycling_options},
};
use crate::{
    misc::{DmgKinds, Spool},
    num::PValue,
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_max, aggr_proj_first_ps, aggr_proj_looped_ps},
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        vast::{StatDmg, StatDmgApplied, StatDmgBreacher, StatDmgItemKinds, Vast, VastFitData},
    },
    ud::{UFitId, UItemId},
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_dps_raw(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
    ) -> StatDmg {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fits_dps(ctx, calc, fit_uids, item_kinds, reload, spool, None);
        StatDmg::from_dmgs(dps_normal, breacher_accum.get_dps())
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
    ) -> StatDmgApplied {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fits_dps(ctx, calc, fit_uids, item_kinds, reload, spool, Some(projectee_uid));
        StatDmgApplied::from_dmgs(
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
    ) -> StatDmg {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fit_dps(ctx, calc, fit_uid, item_kinds, reload, spool, None);
        StatDmg::from_dmgs(dps_normal, breacher_accum.get_dps())
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
    ) -> StatDmgApplied {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fit_dps(ctx, calc, fit_uid, item_kinds, reload, spool, Some(projectee_uid));
        StatDmgApplied::from_dmgs(
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
    ) -> StatDmg {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fits_volley(ctx, calc, fit_uids, item_kinds, spool, None);
        StatDmg::from_dmgs(volley_normal, Some(volley_breacher))
    }
    pub(in crate::svc) fn get_stat_fits_volley_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fits_volley(ctx, calc, fit_uids, item_kinds, spool, Some(projectee_uid));
        StatDmgApplied::from_dmgs(
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
    ) -> (DmgKinds<PValue>, StatDmgBreacher) {
        let mut volley_normal = DmgKinds::default();
        let mut volley_breacher = StatDmgBreacher::new();
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
    ) -> StatDmg {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fit_volley(ctx, calc, fit_uid, item_kinds, spool, None);
        StatDmg::from_dmgs(volley_normal, Some(volley_breacher))
    }
    pub(in crate::svc) fn get_stat_fit_volley_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fit_volley(ctx, calc, fit_uid, item_kinds, spool, Some(projectee_uid));
        StatDmgApplied::from_dmgs(
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
    ) -> (DmgKinds<PValue>, StatDmgBreacher) {
        let mut volley_normal = DmgKinds::default();
        let mut volley_breacher = StatDmgBreacher::new();
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
                        if let Some(effect_dps) =
                            aggr_proj_first_ps(ctx, calc, item_uid, effect, cseq, ospec, projectee_uid, spool)
                        {
                            *dps_normal += effect_dps;
                        }
                    }
                    CyclingOptions::Sim(_) => {
                        if let Some(effect_dps) =
                            aggr_proj_looped_ps(ctx, calc, item_uid, effect, cseq, ospec, projectee_uid)
                        {
                            *dps_normal += effect_dps;
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
                breacher_accum.add(opc, cseq.convert());
            }
        }
    }
    fn fill_stat_volley(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        volley_normal: &mut DmgKinds<PValue>,
        volley_breacher: &mut StatDmgBreacher,
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
                if let Some(dmg_max) =
                    aggr_proj_first_max(ctx, calc, item_uid, effect, cseq, ospec, projectee_uid, spool)
                {
                    *volley_normal += dmg_max;
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
