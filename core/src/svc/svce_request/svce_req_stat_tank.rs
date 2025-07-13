use crate::{
    def::{AttrVal, ItemKey},
    misc::{DmgKinds, DpsProfile, Spool},
    svc::{
        Svc, SvcCtx,
        err::StatItemCheckError,
        vast::{StatLayerEhp, StatLayerHp, StatLayerReps, StatTank, Vast},
    },
    uad::Uad,
};

impl Svc {
    pub(crate) fn get_stat_item_hp(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
    ) -> Result<StatTank<StatLayerHp>, StatItemCheckError> {
        self.vast
            .get_stat_item_hp_checked(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_ehp(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
        incoming_dps: Option<&DpsProfile>,
    ) -> Result<Option<StatTank<StatLayerEhp>>, StatItemCheckError> {
        self.vast
            .get_stat_item_ehp_checked(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key, incoming_dps)
    }
    pub(crate) fn get_stat_item_wc_ehp(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
    ) -> Result<Option<StatTank<StatLayerEhp>>, StatItemCheckError> {
        self.vast
            .get_stat_item_wc_ehp_checked(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_reps(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
        spool: Option<Spool>,
    ) -> Result<StatTank<StatLayerReps>, StatItemCheckError> {
        self.vast
            .get_stat_item_reps_checked(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key, spool)
    }
    pub(crate) fn get_stat_item_resists(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
    ) -> Result<StatTank<DmgKinds<AttrVal>>, StatItemCheckError> {
        Vast::get_stat_item_resists_checked(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
}
