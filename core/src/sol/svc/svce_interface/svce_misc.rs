use crate::{
    defs::{EAttrId, SkillLevel, SolItemId},
    sol::{svc::SolSvc, uad::SolUad},
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
    pub(in crate::sol) fn skill_level_changed(&mut self, uad: &SolUad, item_id: &SolItemId, level: SkillLevel) {
        self.notify_skill_level_changed(uad, item_id, level);
    }
}
