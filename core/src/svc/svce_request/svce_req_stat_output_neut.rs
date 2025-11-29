use crate::{
    def::AttrVal,
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatNeutItemKinds, Vast},
    },
    ud::{UData, UFitKey, UItemKey},
};

impl Svc {
    pub(crate) fn get_stat_fits_outgoing_nps(
        &mut self,
        u_data: &UData,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatNeutItemKinds,
        projectee_key: Option<UItemKey>,
    ) -> AttrVal {
        self.vast.get_stat_fits_outgoing_nps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_keys,
            item_kinds,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_fit_outgoing_nps(
        &mut self,
        u_data: &UData,
        fit_key: UFitKey,
        item_kinds: StatNeutItemKinds,
        projectee_key: Option<UItemKey>,
    ) -> AttrVal {
        self.vast.get_stat_fit_outgoing_nps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_key,
            item_kinds,
            projectee_key,
        )
    }
    pub(crate) fn get_stat_item_outgoing_nps(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: Option<UItemKey>,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_outgoing_nps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            include_charges,
            ignore_state,
            projectee_key,
        )
    }
}
