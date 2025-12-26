use crate::{
    def::AttrVal,
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatCapSim, StatCapSimStaggerInt, StatCapSrcKinds, Vast},
    },
    ud::{UData, UItemKey},
    util::UnitInterval,
};

impl Svc {
    pub(crate) fn get_stat_item_cap_amount(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_cap_amount(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_cap_balance(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        src_kinds: StatCapSrcKinds,
    ) -> Result<AttrVal, StatItemCheckError> {
        self.vast.get_stat_item_cap_balance(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            src_kinds,
        )
    }
    pub(crate) fn get_stat_item_cap_sim(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        cap_perc: UnitInterval,
        stagger: StatCapSimStaggerInt,
    ) -> Result<StatCapSim, StatItemCheckError> {
        self.vast.get_stat_item_cap_sim(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            cap_perc,
            stagger,
        )
    }
    pub(crate) fn get_stat_item_neut_resist(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_neut_resist(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
}
