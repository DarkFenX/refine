use crate::{
    def::AttrVal,
    misc::{DmgKinds, DpsProfile, Spool},
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{
            StatLayerEhp, StatLayerErps, StatLayerErpsRegen, StatLayerHp, StatLayerRps, StatLayerRpsRegen, StatTank,
            StatTankRegen, Vast,
        },
    },
    ud::{UData, UItemKey},
    util::UnitInterval,
};

impl Svc {
    pub(crate) fn get_stat_item_hp(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<StatTank<StatLayerHp>, StatItemCheckError> {
        self.vast
            .get_stat_item_hp(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_ehp(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        incoming_dps: Option<DpsProfile>,
    ) -> Result<StatTank<Option<StatLayerEhp>>, StatItemCheckError> {
        self.vast.get_stat_item_ehp(
            SvcCtx::new(u_data, &self.eff_projs),
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
            .get_stat_item_wc_ehp(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_rps(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        shield_perc: UnitInterval,
        spool: Option<Spool>,
    ) -> Result<StatTankRegen<StatLayerRps, StatLayerRpsRegen>, StatItemCheckError> {
        self.vast.get_stat_item_rps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            shield_perc,
            spool,
        )
    }
    pub(crate) fn get_stat_item_erps(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
        incoming_dps: Option<DpsProfile>,
        shield_perc: UnitInterval,
        spool: Option<Spool>,
    ) -> Result<StatTankRegen<Option<StatLayerErps>, Option<StatLayerErpsRegen>>, StatItemCheckError> {
        self.vast.get_stat_item_erps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            incoming_dps,
            shield_perc,
            spool,
        )
    }
    pub(crate) fn get_stat_item_resists(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Result<StatTank<DmgKinds<AttrVal>>, StatItemCheckError> {
        Vast::get_stat_item_resists(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_key)
    }
}
