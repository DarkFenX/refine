use crate::{
    misc::{DmgKinds, DpsProfile},
    num::UnitInterval,
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{
            StatLayerEhp, StatLayerErps, StatLayerErpsRegen, StatLayerHp, StatLayerRps, StatLayerRpsRegen, StatTank,
            StatTankRegen, StatTimeOptions, Vast,
        },
    },
    ud::{UData, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_item_hp(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<StatTank<StatLayerHp>, StatItemCheckError> {
        self.vast
            .get_stat_item_hp(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_ehp(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        incoming_dps: Option<DpsProfile>,
    ) -> Result<StatTank<Option<StatLayerEhp>>, StatItemCheckError> {
        self.vast.get_stat_item_ehp(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            incoming_dps,
        )
    }
    pub(crate) fn get_stat_item_wc_ehp(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<StatTank<Option<StatLayerEhp>>, StatItemCheckError> {
        self.vast
            .get_stat_item_wc_ehp(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_rps(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatTankRegen<StatLayerRps, StatLayerRpsRegen>, StatItemCheckError> {
        self.vast.get_stat_item_rps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            shield_perc,
        )
    }
    pub(crate) fn get_stat_item_erps(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        incoming_dps: Option<DpsProfile>,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatTankRegen<Option<StatLayerErps>, Option<StatLayerErpsRegen>>, StatItemCheckError> {
        self.vast.get_stat_item_erps(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            incoming_dps,
            time_options,
            shield_perc,
        )
    }
    pub(crate) fn get_stat_item_resists(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<StatTank<DmgKinds<UnitInterval>>, StatItemCheckError> {
        Vast::get_stat_item_resists(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
}
