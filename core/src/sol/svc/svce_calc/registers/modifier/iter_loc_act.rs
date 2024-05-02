use crate::sol::{fit::SolFit, item::SolItem, svc::svce_calc::SolLocationKind};

// Iterator over item's root location kinds which are actually assigned to a fit
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
    type Item = SolLocationKind;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.char_done {
            self.char_done = true;
            if self.item.is_on_char_root() && self.fit.character.is_some() {
                return Some(SolLocationKind::Character);
            }
        }
        if !self.ship_done {
            self.ship_done = true;
            if self.item.is_on_ship_root() && self.fit.ship.is_some() {
                return Some(SolLocationKind::Ship);
            }
        }
        if !self.struct_done {
            self.struct_done = true;
            if self.item.is_on_struct_root() && self.fit.structure.is_some() {
                return Some(SolLocationKind::Structure);
            }
        }
        None
    }
}
