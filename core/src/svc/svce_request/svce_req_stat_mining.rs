use crate::{
    misc::MiningKinds,
    svc::{Svc, SvcCtx, vast::StatMiningItemKinds},
    ud::{UData, UFitKey},
};

impl Svc {
    pub(crate) fn get_stat_fits_mps(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatMiningItemKinds,
    ) -> MiningKinds {
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
    ) -> MiningKinds {
        self.vast.get_stat_fit_mps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
        )
    }
}
