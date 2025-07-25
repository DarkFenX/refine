use crate::{
    def::AttrVal,
    misc::{DmgKinds, DpsProfile, Spool},
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatLayerEhp, StatLayerErps, StatLayerHp, StatLayerRps, StatTank, Vast},
    },
    ud::{UData, UItemKey},
};

impl Svc {
    pub(crate) fn get_stat_item_hp(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<StatTank<StatLayerHp>, StatItemCheckError> {
        self.vast
            .get_stat_item_hp_checked(SvcCtx::new(u_data, &self.eprojs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_ehp(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        incoming_dps: Option<DpsProfile>,
    ) -> Result<StatTank<Option<StatLayerEhp>>, StatItemCheckError> {
        self.vast.get_stat_item_ehp_checked(
            SvcCtx::new(u_data, &self.eprojs),
            &mut self.calc,
            item_key,
            incoming_dps,
        )
    }
    pub(crate) fn get_stat_item_wc_ehp(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<StatTank<Option<StatLayerEhp>>, StatItemCheckError> {
        self.vast
            .get_stat_item_wc_ehp_checked(SvcCtx::new(u_data, &self.eprojs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_rps(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        spool: Option<Spool>,
    ) -> Result<StatTank<StatLayerRps>, StatItemCheckError> {
        self.vast
            .get_stat_item_rps_checked(SvcCtx::new(u_data, &self.eprojs), &mut self.calc, item_key, spool)
    }
    pub(crate) fn get_stat_item_erps(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        incoming_dps: Option<DpsProfile>,
        spool: Option<Spool>,
    ) -> Result<StatTank<Option<StatLayerErps>>, StatItemCheckError> {
        self.vast.get_stat_item_erps_checked(
            SvcCtx::new(u_data, &self.eprojs),
            &mut self.calc,
            item_key,
            incoming_dps,
            spool,
        )
    }
    pub(crate) fn get_stat_item_resists(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<StatTank<DmgKinds<AttrVal>>, StatItemCheckError> {
        Vast::get_stat_item_resists_checked(SvcCtx::new(u_data, &self.eprojs), &mut self.calc, item_key)
    }
}
