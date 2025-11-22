use crate::{
    svc::calc::LocationKind,
    ud::{UFit, UItem, UShipKind},
};

// Iterator over item's root location kinds which are actually assigned to a fit
pub(super) struct ActiveLocations<'a> {
    item: &'a UItem,
    fit: &'a UFit,
    index: usize,
}
impl<'a> ActiveLocations<'a> {
    pub(super) fn new(item: &'a UItem, fit: &'a UFit) -> Self {
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
                    // Character is considered as always-active, even if fit has no character
                    if self.item.is_on_char_root() {
                        return Some(LocationKind::Character);
                    }
                }
                1 => {
                    self.index += 1;
                    if self.item.is_on_ship_root() && matches!(self.fit.kind, UShipKind::Ship) {
                        return Some(LocationKind::Ship);
                    }
                }
                2 => {
                    self.index += 1;
                    if self.item.is_on_struct_root() && matches!(self.fit.kind, UShipKind::Structure) {
                        return Some(LocationKind::Structure);
                    }
                }
                _ => return None,
            }
        }
    }
}
