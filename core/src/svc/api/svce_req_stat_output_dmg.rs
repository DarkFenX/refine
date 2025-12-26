use crate::{
    misc::Spool,
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatDmg, StatDmgApplied, StatDmgItemKinds, Vast},
    },
    ud::{UData, UFitKey, UItemKey},
};

impl Svc {
    pub(crate) fn get_stat_fits_dps_raw(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
    ) -> StatDmg {
        self.vast.get_stat_fits_dps_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            item_kinds,
            reload,
            spool,
        )
    }
    pub(crate) fn get_stat_fits_dps_applied(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_key: UItemKey,
    ) -> StatDmgApplied {
        self.vast.get_stat_fits_dps_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            item_kinds,
            reload,
            spool,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_fit_dps_raw(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
    ) -> StatDmg {
        self.vast.get_stat_fit_dps_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
            reload,
            spool,
        )
    }
    pub(crate) fn get_stat_fit_dps_applied(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_key: UItemKey,
    ) -> StatDmgApplied {
        self.vast.get_stat_fit_dps_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
            reload,
            spool,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_item_dps_raw(
        &mut self,
        u_data: &UData,
        item_key: UFitKey,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        Vast::get_stat_item_dps_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            reload,
            spool,
            include_charges,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_item_dps_applied(
        &mut self,
        u_data: &UData,
        item_key: UFitKey,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: UItemKey,
    ) -> Result<StatDmgApplied, StatItemCheckError> {
        Vast::get_stat_item_dps_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            reload,
            spool,
            include_charges,
            ignore_state,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_fits_volley_raw(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
    ) -> StatDmg {
        self.vast.get_stat_fits_volley_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            item_kinds,
            spool,
        )
    }
    pub(crate) fn get_stat_fits_volley_applied(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_key: UItemKey,
    ) -> StatDmgApplied {
        self.vast.get_stat_fits_volley_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            item_kinds,
            spool,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_fit_volley_raw(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
    ) -> StatDmg {
        self.vast.get_stat_fit_volley_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
            spool,
        )
    }
    pub(crate) fn get_stat_fit_volley_applied(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_key: UItemKey,
    ) -> StatDmgApplied {
        self.vast.get_stat_fit_volley_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
            spool,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_item_volley_raw(
        &mut self,
        u_data: &UData,
        item_key: UFitKey,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        Vast::get_stat_item_volley_raw(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            spool,
            include_charges,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_item_volley_applied(
        &mut self,
        u_data: &UData,
        item_key: UFitKey,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: UItemKey,
    ) -> Result<StatDmgApplied, StatItemCheckError> {
        Vast::get_stat_item_volley_applied(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            spool,
            include_charges,
            ignore_state,
            projectee_key,
        )
    }
}
