use crate::{
    misc::Spool,
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatDmg, StatDmgApplied, StatDmgEntry, StatDmgEntryApplied, StatDmgItemKinds, StatTimeOptions, Vast},
    },
    ud::{UData, UFitId, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_fits_dmg_raw(
        &mut self,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
    ) -> StatDmg {
        self.vast.get_stat_fits_dmg_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            time_options,
        )
    }
    pub(crate) fn get_stat_fits_dmg_applied(
        &mut self,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        self.vast.get_stat_fits_dmg_applied(
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
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
    ) -> StatDmg {
        self.vast.get_stat_fit_dmg_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            time_options,
        )
    }
    pub(crate) fn get_stat_fit_dmg_applied(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: UItemId,
    ) -> StatDmgApplied {
        self.vast.get_stat_fit_dmg_applied(
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
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        Vast::get_stat_item_dmg_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            include_charges,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_item_dmg_applied(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: UItemId,
    ) -> Result<StatDmgApplied, StatItemCheckError> {
        Vast::get_stat_item_dmg_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            include_charges,
            ignore_state,
            projectee_uid,
        )
    }
}

impl Svc {
    pub(crate) fn get_stat_fits_dps_raw(
        &mut self,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
    ) -> StatDmgEntry {
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
        spool: Option<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgEntryApplied {
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
        spool: Option<Spool>,
    ) -> StatDmgEntry {
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
        spool: Option<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgEntryApplied {
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
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmgEntry, StatItemCheckError> {
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
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: UItemId,
    ) -> Result<StatDmgEntryApplied, StatItemCheckError> {
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
        spool: Option<Spool>,
    ) -> StatDmgEntry {
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
        spool: Option<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgEntryApplied {
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
        spool: Option<Spool>,
    ) -> StatDmgEntry {
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
        spool: Option<Spool>,
        projectee_uid: UItemId,
    ) -> StatDmgEntryApplied {
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
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmgEntry, StatItemCheckError> {
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
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_uid: UItemId,
    ) -> Result<StatDmgEntryApplied, StatItemCheckError> {
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
