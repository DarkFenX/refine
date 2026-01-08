use crate::{
    misc::{Spool, StOption},
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatDmg, StatDmgApplied, StatDmgItemKinds, Vast},
    },
    ud::{UData, UFitId, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_fits_dps_raw(
        &mut self,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: StOption<Spool>,
    ) -> StatDmg {
        self.vast.get_stat_fits_dps_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            reload,
            spool,
        )
    }
    pub(crate) fn get_stat_fits_dps_applied(
        &mut self,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: StOption<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        self.vast.get_stat_fits_dps_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            reload,
            spool,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_fit_dps_raw(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: StOption<Spool>,
    ) -> StatDmg {
        self.vast.get_stat_fit_dps_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            reload,
            spool,
        )
    }
    pub(crate) fn get_stat_fit_dps_applied(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: StOption<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        self.vast.get_stat_fit_dps_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            reload,
            spool,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_item_dps_raw(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        reload: bool,
        spool: StOption<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        Vast::get_stat_item_dps_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            reload,
            spool,
            include_charges,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_item_dps_applied(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        reload: bool,
        spool: StOption<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: UItemId,
    ) -> Result<StatDmgApplied, StatItemCheckError> {
        Vast::get_stat_item_dps_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            reload,
            spool,
            include_charges,
            ignore_state,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_fits_volley_raw(
        &mut self,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        spool: StOption<Spool>,
    ) -> StatDmg {
        self.vast.get_stat_fits_volley_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            spool,
        )
    }
    pub(crate) fn get_stat_fits_volley_applied(
        &mut self,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        spool: StOption<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        self.vast.get_stat_fits_volley_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            spool,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_fit_volley_raw(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        spool: StOption<Spool>,
    ) -> StatDmg {
        self.vast.get_stat_fit_volley_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            spool,
        )
    }
    pub(crate) fn get_stat_fit_volley_applied(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        spool: StOption<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        self.vast.get_stat_fit_volley_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            spool,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_item_volley_raw(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        spool: StOption<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        Vast::get_stat_item_volley_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            spool,
            include_charges,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_item_volley_applied(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        spool: StOption<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: UItemId,
    ) -> Result<StatDmgApplied, StatItemCheckError> {
        Vast::get_stat_item_volley_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            spool,
            include_charges,
            ignore_state,
            projectee_uid,
        )
    }
}
