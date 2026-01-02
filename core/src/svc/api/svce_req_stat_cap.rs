use crate::{
    def::AttrVal,
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatCapSim, StatCapSimStaggerInt, StatCapSrcKinds, StatTimeOptions, Vast},
    },
    ud::{UData, UItemId},
    util::UnitInterval,
};

impl Svc {
    pub(crate) fn get_stat_item_cap_amount(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_cap_amount(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_cap_balance(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
        src_kinds: StatCapSrcKinds,
        time_options: StatTimeOptions,
    ) -> Result<AttrVal, StatItemCheckError> {
        self.vast.get_stat_item_cap_balance(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            src_kinds,
            time_options,
        )
    }
    pub(crate) fn get_stat_item_cap_sim(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
        cap_perc: UnitInterval,
        reload_optionals: Option<bool>,
        stagger: StatCapSimStaggerInt,
    ) -> Result<StatCapSim, StatItemCheckError> {
        self.vast.get_stat_item_cap_sim(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            cap_perc,
            reload_optionals,
            stagger,
        )
    }
    pub(crate) fn get_stat_item_neut_resist(
        &mut self,
        u_data: &UData,
        item_key: UItemId,
    ) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_neut_resist(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
}
