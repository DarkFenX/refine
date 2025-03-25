use crate::{
    ad,
    sol::{
        ItemId,
        svc::Svc,
        uad::{
            Uad,
            item::{Fighter, Skill},
        },
    },
    src::Src,
};

impl Svc {
    pub(in crate::sol) fn src_changed(&mut self, src: &Src) {
        self.notify_src_changed(src);
    }
    pub(in crate::sol) fn item_base_attr_value_changed(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
        a_attr_id: &ad::AAttrId,
    ) {
        self.notify_base_attr_value_changed(uad, item_id, a_attr_id);
    }
    pub(in crate::sol) fn fighter_count_changed(&mut self, uad: &Uad, fighter: &Fighter) {
        self.notify_fighter_count_changed(uad, fighter);
    }
    pub(in crate::sol) fn skill_level_changed(&mut self, uad: &Uad, skill: &Skill) {
        self.notify_skill_level_changed(uad, skill);
    }
}
