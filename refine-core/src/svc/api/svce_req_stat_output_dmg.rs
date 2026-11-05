use crate::{
    stats::{StatCritOptions, StatDmg, StatDmgApplied, StatDmgItemKinds, StatItemChargeOptions, StatTimeOptions},
    svc::{Svc, SvcCtx, Vast, cycle::CseqCache, err::IntStatItemError},
    ud::{UData, UFitId, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_fits_dmg_raw(
        &mut self,
        cseq_cache: &mut CseqCache,
        u_data: &UData,
        fit_uids: impl Iterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
    ) -> StatDmg {
        self.vast.get_stat_fits_dmg_raw(
            cseq_cache,
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            time_options,
            crit_options,
        )
    }
    pub(crate) fn get_stat_fits_dmg_applied(
        &mut self,
        cseq_cache: &mut CseqCache,
        u_data: &UData,
        fit_uids: impl Iterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        self.vast.get_stat_fits_dmg_applied(
            cseq_cache,
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            time_options,
            crit_options,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_fit_dmg_raw(
        &mut self,
        cseq_cache: &mut CseqCache,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
    ) -> StatDmg {
        self.vast.get_stat_fit_dmg_raw(
            cseq_cache,
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            time_options,
            crit_options,
        )
    }
    pub(crate) fn get_stat_fit_dmg_applied(
        &mut self,
        cseq_cache: &mut CseqCache,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        self.vast.get_stat_fit_dmg_applied(
            cseq_cache,
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            time_options,
            crit_options,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_item_dmg_raw(
        &mut self,
        cseq_cache: &mut CseqCache,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        charge_options: StatItemChargeOptions,
    ) -> Result<StatDmg, IntStatItemError<!>> {
        Vast::get_stat_item_dmg_raw(
            cseq_cache,
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            crit_options,
            charge_options,
        )
    }
    pub(crate) fn get_stat_item_dmg_applied(
        &mut self,
        cseq_cache: &mut CseqCache,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        charge_options: StatItemChargeOptions,
        projectee_uid: UItemId,
    ) -> Result<StatDmgApplied, IntStatItemError<!>> {
        Vast::get_stat_item_dmg_applied(
            cseq_cache,
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            crit_options,
            charge_options,
            projectee_uid,
        )
    }
}
