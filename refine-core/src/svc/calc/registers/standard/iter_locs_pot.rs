use crate::{svc::calc::LocationKind, ud::UItem, util::State4};

// Iterator over item's potential location roots
pub(super) struct PotentialLocations<'a> {
    item: &'a UItem,
    state: State4,
}
impl<'a> PotentialLocations<'a> {
    pub(super) fn new(item: &'a UItem) -> Self {
        Self {
            item,
            state: State4::One,
        }
    }
}
impl Iterator for PotentialLocations<'_> {
    type Item = LocationKind;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State4::One => {
                    self.state = State4::Two;
                    if self.item.is_on_char_root() {
                        return Some(LocationKind::Character);
                    }
                }
                State4::Two => {
                    self.state = State4::Three;
                    if self.item.is_on_ship_root() {
                        return Some(LocationKind::Ship);
                    }
                }
                State4::Three => {
                    self.state = State4::Four;
                    if self.item.is_on_struct_root() {
                        return Some(LocationKind::Structure);
                    }
                }
                State4::Four => return None,
            }
        }
    }
}
