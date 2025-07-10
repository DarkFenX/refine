use crate::{
    def::{AttrVal, ItemKey},
    misc::{DmgKinds, DpsProfile},
    svc::{
        Svc, SvcCtx,
        vast::{StatLayerEhp, StatLayerHp, StatTank, Vast},
    },
    uad::Uad,
};

impl Svc {
    pub(crate) fn get_stat_item_hp(&mut self, uad: &Uad, item_key: ItemKey) -> Option<StatTank<StatLayerHp>> {
        self.vast
            .get_stat_item_hp(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_ehp(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
        incoming_dps: Option<&DpsProfile>,
    ) -> Option<StatTank<StatLayerEhp>> {
        self.vast
            .get_stat_item_ehp(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key, incoming_dps)
    }
    pub(crate) fn get_stat_item_wc_ehp(&mut self, uad: &Uad, item_key: ItemKey) -> Option<StatTank<StatLayerEhp>> {
        self.vast
            .get_stat_item_wc_ehp(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_resists(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
    ) -> Option<StatTank<DmgKinds<AttrVal>>> {
        Vast::get_stat_item_resists(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
}
