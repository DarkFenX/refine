use crate::sol::{svc::calc::SolLocationKind, uad::item::SolItem};

// Iterator over item's potential location roots
pub(super) struct SolPotentialLocations<'a> {
    item: &'a SolItem,
    index: usize,
}
impl<'a> SolPotentialLocations<'a> {
    pub(super) fn new(item: &'a SolItem) -> Self {
        Self { item, index: 0 }
    }
}
impl<'a> Iterator for SolPotentialLocations<'a> {
    type Item = SolLocationKind;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.index {
                0 => {
                    self.index += 1;
                    if self.item.is_on_char_root() {
                        return Some(SolLocationKind::Character);
                    }
                }
                1 => {
                    self.index += 1;
                    if self.item.is_on_ship_root() {
                        return Some(SolLocationKind::Ship);
                    }
                }
                2 => {
                    self.index += 1;
                    if self.item.is_on_struct_root() {
                        return Some(SolLocationKind::Structure);
                    }
                }
                _ => return None,
            }
        }
    }
}
