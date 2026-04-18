use crate::{
    svc::{
        Svc, SvcCtx,
        cycle::CseqMap,
        err::StatItemCheckError,
        vast::{StatDmg, StatDmgApplied, StatDmgItemKinds, StatTimeOptions, Vast},
    },
    ud::{UData, UFitId, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_fits_dmg_raw(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
    ) -> StatDmg {
        self.vast.get_stat_fits_dmg_raw(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            time_options,
        )
    }
    pub(crate) fn get_stat_fits_dmg_applied(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        self.vast.get_stat_fits_dmg_applied(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            time_options,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_fit_dmg_raw(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
    ) -> StatDmg {
        self.vast.get_stat_fit_dmg_raw(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            time_options,
        )
    }
    pub(crate) fn get_stat_fit_dmg_applied(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        self.vast.get_stat_fit_dmg_applied(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            time_options,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_item_dmg_raw(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        Vast::get_stat_item_dmg_raw(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            include_charges,
        )
    }
    pub(crate) fn get_stat_item_dmg_applied(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
        projectee_uid: UItemId,
    ) -> Result<StatDmgApplied, StatItemCheckError> {
        Vast::get_stat_item_dmg_applied(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            include_charges,
            projectee_uid,
        )
    }
}
