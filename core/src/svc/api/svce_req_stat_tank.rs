use crate::{
    misc::DpsProfile,
    num::UnitInterval,
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatEhp, StatErps, StatHp, StatResists, StatRps, StatTimeOptions, Vast},
    },
    ud::{UData, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_item_hp(&mut self, u_data: &UData, item_uid: UItemId) -> Result<StatHp, StatItemCheckError> {
        self.vast
            .get_stat_item_hp(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_ehp(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        incoming_dps: Option<DpsProfile>,
    ) -> Result<StatEhp, StatItemCheckError> {
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
    ) -> Result<StatEhp, StatItemCheckError> {
        self.vast
            .get_stat_item_wc_ehp(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_rps(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatRps, StatItemCheckError> {
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
    ) -> Result<StatErps, StatItemCheckError> {
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
    ) -> Result<StatResists, StatItemCheckError> {
        Vast::get_stat_item_resists(SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
}
