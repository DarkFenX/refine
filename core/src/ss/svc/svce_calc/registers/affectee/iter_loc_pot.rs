use crate::ss::{item::SsItem, svc::svce_calc::SsLocType};

// Iterator over item's potential parent domains
pub(super) struct PotentialLocations<'a> {
    item: &'a SsItem,
    char_done: bool,
    ship_done: bool,
    struct_done: bool,
}
impl<'a> PotentialLocations<'a> {
    pub(super) fn new(item: &'a SsItem) -> Self {
        Self {
            item,
            char_done: false,
            ship_done: false,
            struct_done: false,
        }
    }
}
impl<'a> Iterator for PotentialLocations<'a> {
    type Item = SsLocType;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.char_done {
            self.char_done = true;
            if self.item.can_have_parent_char() {
                return Some(SsLocType::Character);
            }
        }
        if !self.ship_done {
            self.ship_done = true;
            if self.item.can_have_parent_ship() {
                return Some(SsLocType::Ship);
            }
        }
        if !self.struct_done {
            self.struct_done = true;
            if self.item.can_have_parent_struct() {
                return Some(SsLocType::Structure);
            }
        }
        None
    }
}
