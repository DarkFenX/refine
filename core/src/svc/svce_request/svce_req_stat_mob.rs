use crate::{
    def::{AttrVal, ItemKey},
    svc::{Svc, SvcCtx, vast::Vast},
    uad::Uad,
};

impl Svc {
    pub(crate) fn get_stat_item_speed(&mut self, uad: &Uad, item_key: ItemKey) -> Option<AttrVal> {
        Vast::get_stat_item_speed(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_agility(&mut self, uad: &Uad, item_key: ItemKey) -> Option<AttrVal> {
        Vast::get_stat_item_agility(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
    pub(crate) fn get_stat_item_align_time(&mut self, uad: &Uad, item_key: ItemKey) -> Option<AttrVal> {
        Vast::get_stat_item_align_time(SvcCtx::new(uad, &self.eprojs), &mut self.calc, item_key)
    }
}
