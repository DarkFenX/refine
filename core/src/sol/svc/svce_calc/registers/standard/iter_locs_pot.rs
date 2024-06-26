use crate::sol::{item::SolItem, svc::svce_calc::SolLocationKind};

// Iterator over item's potential location roots
pub(super) struct PotentialLocations<'a> {
    item: &'a SolItem,
    index: usize,
}
impl<'a> PotentialLocations<'a> {
    pub(super) fn new(item: &'a SolItem) -> Self {
        Self { item, index: 0 }
    }
}
impl<'a> Iterator for PotentialLocations<'a> {
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
