use crate::sol::{
    svc::calc::LocationKind,
    uad::{
        fit::Fit,
        item::{Item, ShipKind},
    },
};

// Iterator over item's root location kinds which are actually assigned to a fit
pub(super) struct ActiveLocations<'a> {
    item: &'a Item,
    fit: &'a Fit,
    index: usize,
}
impl<'a> ActiveLocations<'a> {
    pub(super) fn new(item: &'a Item, fit: &'a Fit) -> Self {
        Self { item, fit, index: 0 }
    }
}
impl Iterator for ActiveLocations<'_> {
    type Item = LocationKind;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.index {
                0 => {
                    self.index += 1;
                    if self.item.is_on_char_root() && self.fit.character.is_some() {
                        return Some(LocationKind::Character);
                    }
                }
                1 => {
                    self.index += 1;
                    if self.item.is_on_ship_root() && matches!(self.fit.kind, ShipKind::Ship) {
                        return Some(LocationKind::Ship);
                    }
                }
                2 => {
                    self.index += 1;
                    if self.item.is_on_struct_root() && matches!(self.fit.kind, ShipKind::Structure) {
                        return Some(LocationKind::Structure);
                    }
                }
                _ => return None,
            }
        }
    }
}
