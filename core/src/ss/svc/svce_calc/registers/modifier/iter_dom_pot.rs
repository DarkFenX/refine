use crate::{shr::ModDomain, ss::item::SsItem};

// Iterator over item's potential parent domains
pub(super) struct DomsPot<'a> {
    item: &'a SsItem,
    char_done: bool,
    ship_done: bool,
    struct_done: bool,
}
impl<'a> DomsPot<'a> {
    pub(super) fn new(item: &'a SsItem) -> Self {
        Self {
            item,
            char_done: false,
            ship_done: false,
            struct_done: false,
        }
    }
}
impl<'a> Iterator for DomsPot<'a> {
    type Item = ModDomain;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.char_done {
            self.char_done = true;
            if self.item.can_have_parent_char() {
                return Some(ModDomain::Char);
            }
        }
        if !self.ship_done {
            self.ship_done = true;
            if self.item.can_have_parent_ship() {
                return Some(ModDomain::Ship);
            }
        }
        if !self.struct_done {
            self.struct_done = true;
            if self.item.can_have_parent_struct() {
                return Some(ModDomain::Structure);
            }
        }
        None
    }
}
