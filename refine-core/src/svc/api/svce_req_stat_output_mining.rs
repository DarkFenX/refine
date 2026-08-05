use crate::{
    stats::{StatMining, StatMiningItemKinds, StatMiningResourceKind, StatTimeOptions},
    svc::{Svc, SvcCtx, Vast, cycle::CseqMap, err::IntStatItemError},
    ud::{UData, UFitId, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_fits_mps(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        fit_uids: impl ExactSizeIterator<Item = UFitId>,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
        resource_kind: StatMiningResourceKind,
    ) -> StatMining {
        self.vast.get_stat_fits_mps(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uids,
            item_kinds,
            time_options,
            resource_kind,
        )
    }
    pub(crate) fn get_stat_fit_mps(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        fit_uid: UFitId,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
        resource_kind: StatMiningResourceKind,
    ) -> StatMining {
        self.vast.get_stat_fit_mps(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            item_kinds,
            time_options,
            resource_kind,
        )
    }
    pub(crate) fn get_stat_item_mps(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        resource_kind: StatMiningResourceKind,
    ) -> Result<StatMining, IntStatItemError<!>> {
        Vast::get_stat_item_mps(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            resource_kind,
        )
    }
}
