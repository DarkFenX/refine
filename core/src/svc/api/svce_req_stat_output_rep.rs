use crate::{
    misc::PValue,
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
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> StatTank<PValue> {
        self.vast.get_stat_fits_outgoing_rps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            time_options,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_fit_outgoing_rps(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> StatTank<PValue> {
        self.vast.get_stat_fit_outgoing_rps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            time_options,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_item_outgoing_rps(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        ignore_state: bool,
        projectee_uid: Option<UItemId>,
    ) -> Result<StatTank<PValue>, StatItemCheckError> {
        Vast::get_stat_item_outgoing_rps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            ignore_state,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_fits_outgoing_cps(
        &mut self,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> PValue {
        self.vast.get_stat_fits_outgoing_cps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            time_options,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_fit_outgoing_cps(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> PValue {
        self.vast.get_stat_fit_outgoing_cps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            time_options,
            projectee_uid,
        )
    }
    pub(crate) fn get_stat_item_outgoing_cps(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        ignore_state: bool,
        projectee_uid: Option<UItemId>,
    ) -> Result<PValue, StatItemCheckError> {
        Vast::get_stat_item_outgoing_cps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            ignore_state,
            projectee_uid,
        )
    }
}
