use crate::sol::{fit::SolFit, item::SolItem, svc::svce_calc::SolLocType};

// Iterator over item's parent location types which are actually assigned to a fit
pub(super) struct ActiveLocations<'a> {
    item: &'a SolItem,
    fit: &'a SolFit,
    char_done: bool,
    ship_done: bool,
    struct_done: bool,
}
impl<'a> ActiveLocations<'a> {
    pub(super) fn new(item: &'a SolItem, fit: &'a SolFit) -> Self {
        Self {
            item,
            fit,
            char_done: false,
            ship_done: false,
            struct_done: false,
        }
    }
}
impl<'a> Iterator for ActiveLocations<'a> {
    type Item = SolLocType;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.char_done {
            self.char_done = true;
            if self.item.can_have_parent_char() && self.fit.character.is_some() {
                return Some(SolLocType::Character);
            }
        }
        if !self.ship_done {
            self.ship_done = true;
            if self.item.can_have_parent_ship() && self.fit.ship.is_some() {
                return Some(SolLocType::Ship);
            }
        }
        if !self.struct_done {
            self.struct_done = true;
            if self.item.can_have_parent_struct() && self.fit.structure.is_some() {
                return Some(SolLocType::Structure);
            }
        }
        None
    }
}
