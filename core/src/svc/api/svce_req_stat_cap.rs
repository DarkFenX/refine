use crate::{
    misc::{PValue, UnitInterval, Value},
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatCapSim, StatCapSimStaggerInt, StatCapSrcKinds, StatTimeOptions, Vast},
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
        u_data: &UData,
        item_uid: UItemId,
        src_kinds: StatCapSrcKinds,
        time_options: StatTimeOptions,
    ) -> Result<Value, StatItemCheckError> {
        self.vast.get_stat_item_cap_balance(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            src_kinds,
            time_options,
        )
    }
    pub(crate) fn get_stat_item_cap_sim(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        cap_perc: UnitInterval,
        reload_optionals: Option<bool>,
        stagger: StatCapSimStaggerInt,
    ) -> Result<StatCapSim, StatItemCheckError> {
        self.vast.get_stat_item_cap_sim(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            cap_perc,
            reload_optionals,
            stagger,
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
