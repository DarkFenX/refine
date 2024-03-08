use crate::{
    shr::ModDomain,
    ss::{fit::SsFit, item::SsItem},
};

// Iterator over item's parent domains which are actually assigned to a fit
pub(super) struct DomsAct<'a> {
    item: &'a SsItem,
    fit: &'a SsFit,
    char_done: bool,
    ship_done: bool,
    struct_done: bool,
}
impl<'a> DomsAct<'a> {
    pub(super) fn new(item: &'a SsItem, fit: &'a SsFit) -> Self {
        Self {
            item,
            fit,
            char_done: false,
            ship_done: false,
            struct_done: false,
        }
    }
}
impl<'a> Iterator for DomsAct<'a> {
    type Item = ModDomain;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.char_done {
            self.char_done = true;
            if self.item.can_have_parent_char() && self.fit.character.is_some() {
                return Some(ModDomain::Char);
            }
        }
        if !self.ship_done {
            self.ship_done = true;
            if self.item.can_have_parent_ship() && self.fit.ship.is_some() {
                return Some(ModDomain::Ship);
            }
        }
        if !self.struct_done {
            self.struct_done = true;
            if self.item.can_have_parent_struct() && self.fit.structure.is_some() {
                return Some(ModDomain::Structure);
            }
        }
        None
    }
}
