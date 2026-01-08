use crate::{
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatMining, StatMiningItemKinds, StatTimeOptions, Vast},
    },
    ud::{UData, UFitId, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_fits_mps(
        &mut self,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
    ) -> StatMining {
        self.vast.get_stat_fits_mps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            time_options,
        )
    }
    pub(crate) fn get_stat_fit_mps(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
    ) -> StatMining {
        self.vast.get_stat_fit_mps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            time_options,
        )
    }
    pub(crate) fn get_stat_item_mps(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        ignore_state: bool,
    ) -> Result<StatMining, StatItemCheckError> {
        Vast::get_stat_item_mps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            ignore_state,
        )
    }
}
