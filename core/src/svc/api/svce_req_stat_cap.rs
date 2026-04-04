use crate::{
    misc::OptionalReload,
    num::{PValue, UnitInterval, Value},
    svc::{
        Svc, SvcCtx,
        cycle::CseqMap,
        err::StatItemCheckError,
        vast::{StatCapBlcSrcKindsInt, StatCapSim, StatCapSimStaggerInt, StatTimeOptions, Vast},
    },
    ud::{UData, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_item_cap_amount(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        Vast::get_stat_item_cap_amount(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_cap_balance(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        item_uid: UItemId,
        src_kinds: StatCapBlcSrcKindsInt,
        time_options: StatTimeOptions,
    ) -> Result<Value, StatItemCheckError> {
        self.vast.get_stat_item_cap_balance(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            src_kinds,
            time_options,
        )
    }
    pub(crate) fn get_stat_item_cap_sim(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        item_uid: UItemId,
        cap_perc: UnitInterval,
        optional_reloads: Option<OptionalReload>,
        stagger: &StatCapSimStaggerInt,
        nosf_projectee_item_uid: Option<UItemId>,
    ) -> Result<StatCapSim, StatItemCheckError> {
        self.vast.get_stat_item_cap_sim(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            cap_perc,
            optional_reloads,
            stagger,
            nosf_projectee_item_uid,
        )
    }
    pub(crate) fn get_stat_item_neut_resist(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<UnitInterval, StatItemCheckError> {
        Vast::get_stat_item_neut_resist(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
}
