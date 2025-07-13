use crate::{
    def::{AttrVal, ItemKey},
    svc::{Svc, SvcCtx, err::StatItemCheckError, vast::Vast},
    uad::Uad,
};

impl Svc {
    pub(crate) fn get_stat_item_speed(&mut self, uad: &Uad, item_key: ItemKey) -> Result<AttrVal, StatItemCheckError> {
        Vast::get_stat_item_speed_checked(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_agility(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_agility_checked(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_align_time(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_align_time_checked(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
}
