use crate::sol::{
    svc::svce_calc::SolLocationKind,
    uad::{
        fit::SolFit,
        item::{SolItem, SolShipKind},
    },
};

// Iterator over item's root location kinds which are actually assigned to a fit
pub(super) struct SolActiveLocations<'a> {
    item: &'a SolItem,
    fit: &'a SolFit,
    index: usize,
}
impl<'a> SolActiveLocations<'a> {
    pub(super) fn new(item: &'a SolItem, fit: &'a SolFit) -> Self {
        Self { item, fit, index: 0 }
    }
}
impl<'a> Iterator for SolActiveLocations<'a> {
    type Item = SolLocationKind;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.index {
                0 => {
                    self.index += 1;
                    if self.item.is_on_char_root() && self.fit.character.is_some() {
                        return Some(SolLocationKind::Character);
                    }
                }
                1 => {
                    self.index += 1;
                    if self.item.is_on_ship_root() && matches!(self.fit.kind, SolShipKind::Ship) {
                        return Some(SolLocationKind::Ship);
                    }
                }
                2 => {
                    self.index += 1;
                    if self.item.is_on_struct_root() && matches!(self.fit.kind, SolShipKind::Structure) {
                        return Some(SolLocationKind::Structure);
                    }
                }
                _ => return None,
            }
        }
    }
}
