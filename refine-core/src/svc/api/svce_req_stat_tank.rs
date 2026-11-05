use crate::{
    DpsProfile, UnitInterval,
    stats::{StatEhp, StatErps, StatHp, StatResists, StatRps, StatTimeOptions},
    svc::{Svc, SvcCtx, cycle::CseqCache, err::IntStatItemError, vast::Vast},
    ud::{UData, UItemId},
};

impl Svc {
    pub(crate) fn get_stat_item_hp(
        &mut self,
        cseq_cache: &mut CseqCache,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<StatHp, IntStatItemError<!>> {
        self.vast.get_stat_item_hp(
            cseq_cache,
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
        )
    }
    pub(crate) fn get_stat_item_ehp(
        &mut self,
        cseq_cache: &mut CseqCache,
        u_data: &UData,
        item_uid: UItemId,
        incoming_dps: Option<DpsProfile>,
    ) -> Result<StatEhp, IntStatItemError<!>> {
        self.vast.get_stat_item_ehp(
            cseq_cache,
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            incoming_dps,
        )
    }
    pub(crate) fn get_stat_item_wc_ehp(
        &mut self,
        cseq_cache: &mut CseqCache,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<StatEhp, IntStatItemError<!>> {
        self.vast.get_stat_item_wc_ehp(
            cseq_cache,
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
        )
    }
    pub(crate) fn get_stat_item_rps(
        &mut self,
        cseq_cache: &mut CseqCache,
        u_data: &UData,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatRps, IntStatItemError<!>> {
        self.vast.get_stat_item_rps(
            cseq_cache,
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            time_options,
            shield_perc,
        )
    }
    pub(crate) fn get_stat_item_erps(
        &mut self,
        cseq_cache: &mut CseqCache,
        u_data: &UData,
        item_uid: UItemId,
        incoming_dps: Option<DpsProfile>,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatErps, IntStatItemError<!>> {
        self.vast.get_stat_item_erps(
            cseq_cache,
            &SvcCtx::new(u_data, &self.eff_projs),
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
    ) -> Result<StatResists, IntStatItemError<!>> {
        Vast::get_stat_item_resists(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
    pub(crate) fn get_stat_item_breach_resist(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Result<UnitInterval, IntStatItemError<!>> {
        Vast::get_stat_item_breach_resist(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, item_uid)
    }
}
