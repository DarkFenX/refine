use crate::{
    def::AttrVal,
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatOutRepItemKinds, StatTank, StatTimeOptions, Vast},
    },
    ud::{UData, UFitId, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_fits_outgoing_rps(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
        projectee_key: Option<UItemId>,
    ) -> StatTank<AttrVal> {
        self.vast.get_stat_fits_outgoing_rps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            item_kinds,
            time_options,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_fit_outgoing_rps(
        &mut self,
        u_data: &UData,
        fit_key: UFitId,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
        projectee_key: Option<UItemId>,
    ) -> StatTank<AttrVal> {
        self.vast.get_stat_fit_outgoing_rps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
            time_options,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_item_outgoing_rps(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
        time_options: StatTimeOptions,
        ignore_state: bool,
        projectee_key: Option<UItemId>,
    ) -> Result<StatTank<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_outgoing_rps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            time_options,
            ignore_state,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_fits_outgoing_cps(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitId>,
        time_options: StatTimeOptions,
        projectee_key: Option<UItemId>,
    ) -> AttrVal {
        self.vast.get_stat_fits_outgoing_cps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            time_options,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_fit_outgoing_cps(
        &mut self,
        u_data: &UData,
        fit_key: UFitId,
        time_options: StatTimeOptions,
        projectee_key: Option<UItemId>,
    ) -> AttrVal {
        self.vast.get_stat_fit_outgoing_cps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            time_options,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_item_outgoing_cps(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
        time_options: StatTimeOptions,
        ignore_state: bool,
        projectee_key: Option<UItemId>,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_outgoing_cps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            time_options,
            ignore_state,
            projectee_key,
        )
    }
}
