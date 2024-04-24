use crate::sol::{item::SolItem, svc::svce_calc::SolLocType};

// Iterator over item's potential location roots
pub(super) struct PotentialLocations<'a> {
    item: &'a SolItem,
    char_done: bool,
    ship_done: bool,
    struct_done: bool,
}
impl<'a> PotentialLocations<'a> {
    pub(super) fn new(item: &'a SolItem) -> Self {
        Self {
            item,
            char_done: false,
            ship_done: false,
            struct_done: false,
        }
    }
}
impl<'a> Iterator for PotentialLocations<'a> {
    type Item = SolLocType;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.char_done {
            self.char_done = true;
            if self.item.is_on_char_root() {
                return Some(SolLocType::Character);
            }
        }
        if !self.ship_done {
            self.ship_done = true;
            if self.item.is_on_ship_root() {
                return Some(SolLocType::Ship);
            }
        }
        if !self.struct_done {
            self.struct_done = true;
            if self.item.is_on_struct_root() {
                return Some(SolLocType::Structure);
            }
        }
        None
    }
}
