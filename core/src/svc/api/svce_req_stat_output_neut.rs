use crate::{
    def::AttrVal,
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatNeutItemKinds, StatTimeOptions, Vast},
    },
    ud::{UData, UFitId, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_fits_outgoing_nps(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatNeutItemKinds,
        time_options: StatTimeOptions,
        projectee_key: Option<UItemId>,
    ) -> AttrVal {
        self.vast.get_stat_fits_outgoing_nps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            item_kinds,
            time_options,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_fit_outgoing_nps(
        &mut self,
        u_data: &UData,
        fit_key: UFitId,
        item_kinds: StatNeutItemKinds,
        time_options: StatTimeOptions,
        projectee_key: Option<UItemId>,
    ) -> AttrVal {
        self.vast.get_stat_fit_outgoing_nps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
            time_options,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_item_outgoing_nps(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
        time_options: StatTimeOptions,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: Option<UItemId>,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_outgoing_nps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            time_options,
            include_charges,
            ignore_state,
            projectee_key,
        )
    }
}
