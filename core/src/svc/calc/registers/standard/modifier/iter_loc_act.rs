use crate::{
    svc::calc::LocationKind,
    ud::{UFit, UItem, UShipKind},
    util::State4,
};

// Iterator over item's root location kinds which are actually assigned to a fit
pub(super) struct ActiveLocations<'a> {
    item: &'a UItem,
    fit: &'a UFit,
    state: State4,
}
impl<'a> ActiveLocations<'a> {
    pub(super) fn new(item: &'a UItem, fit: &'a UFit) -> Self {
        Self {
            item,
            fit,
            state: State4::One,
        }
    }
}
impl Iterator for ActiveLocations<'_> {
    type Item = LocationKind;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State4::One => {
                    self.state = State4::Two;
                    // Character is considered as always-active, even if fit has no character, since
                    // there is no logic in the register which depends on character attributes
                    if self.item.is_on_char_root() {
                        return Some(LocationKind::Character);
                    }
                }
                State4::Two => {
                    self.state = State4::Three;
                    if self.item.is_on_ship_root() && matches!(self.fit.ship_kind, UShipKind::Ship) {
                        return Some(LocationKind::Ship);
                    }
                }
                State4::Three => {
                    self.state = State4::Four;
                    if self.item.is_on_struct_root() && matches!(self.fit.ship_kind, UShipKind::Structure) {
                        return Some(LocationKind::Structure);
                    }
                }
                State4::Four => return None,
            }
        }
    }
}
