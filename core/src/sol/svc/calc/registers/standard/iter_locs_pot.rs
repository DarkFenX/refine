use crate::sol::{svc::calc::LocationKind, uad::item::Item};

// Iterator over item's potential location roots
pub(super) struct PotentialLocations<'a> {
    item: &'a Item,
    index: usize,
}
impl<'a> PotentialLocations<'a> {
    pub(super) fn new(item: &'a Item) -> Self {
        Self { item, index: 0 }
    }
}
impl Iterator for PotentialLocations<'_> {
    type Item = LocationKind;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.index {
                0 => {
                    self.index += 1;
                    if self.item.is_on_char_root() {
                        return Some(LocationKind::Character);
                    }
                }
                1 => {
                    self.index += 1;
                    if self.item.is_on_ship_root() {
                        return Some(LocationKind::Ship);
                    }
                }
                2 => {
                    self.index += 1;
                    if self.item.is_on_struct_root() {
                        return Some(LocationKind::Structure);
                    }
                }
                _ => return None,
            }
        }
    }
}
