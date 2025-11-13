use crate::{
    misc::Mining,
    svc::{Svc, SvcCtx, vast::StatMiningItemKinds},
    ud::{UData, UFitKey},
};

impl Svc {
    pub(crate) fn get_stat_fits_mps_ore(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        self.vast.get_stat_fits_mps_ore(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            item_kinds,
        )
    }
    pub(crate) fn get_stat_fits_mps_ice(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        self.vast.get_stat_fits_mps_ice(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            item_kinds,
        )
    }
    pub(crate) fn get_stat_fits_mps_gas(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        self.vast.get_stat_fits_mps_gas(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            item_kinds,
        )
    }
    pub(crate) fn get_stat_fit_mps_ore(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        self.vast.get_stat_fit_mps_ore(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
        )
    }
    pub(crate) fn get_stat_fit_mps_ice(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        self.vast.get_stat_fit_mps_ice(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
        )
    }
    pub(crate) fn get_stat_fit_mps_gas(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        self.vast.get_stat_fit_mps_gas(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
        )
    }
}
