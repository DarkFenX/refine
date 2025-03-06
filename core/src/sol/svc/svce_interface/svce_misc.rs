use crate::{
    defs::{EAttrId, SolItemId},
    sol::{
        svc::SolSvc,
        uad::{
            SolUad,
            item::{SolFighter, SolSkill},
        },
    },
    src::Src,
};

impl SolSvc {
    pub(in crate::sol) fn src_changed(&mut self, src: &Src) {
        self.notify_src_changed(src);
    }
    pub(in crate::sol) fn item_base_attr_value_changed(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        self.notify_base_attr_value_changed(uad, item_id, attr_id);
    }
    pub(in crate::sol) fn fighter_count_changed(&mut self, uad: &SolUad, fighter: &SolFighter) {
        self.notify_fighter_count_changed(uad, fighter);
    }
    pub(in crate::sol) fn skill_level_changed(&mut self, uad: &SolUad, skill: &SolSkill) {
        self.notify_skill_level_changed(uad, skill);
    }
}
