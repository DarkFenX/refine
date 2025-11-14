use crate::{
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatMining, StatMiningItemKinds, Vast},
    },
    ud::{UData, UFitKey, UItemKey},
};

impl Svc {
    pub(crate) fn get_stat_fits_mps(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatMiningItemKinds,
    ) -> StatMining {
        self.vast.get_stat_fits_mps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            item_kinds,
        )
    }
    pub(crate) fn get_stat_fit_mps(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        item_kinds: StatMiningItemKinds,
    ) -> StatMining {
        self.vast.get_stat_fit_mps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
        )
    }
    pub(crate) fn get_stat_item_mps(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        ignore_state: bool,
    ) -> Result<StatMining, StatItemCheckError> {
        Vast::get_stat_item_mps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            ignore_state,
        )
    }
}
