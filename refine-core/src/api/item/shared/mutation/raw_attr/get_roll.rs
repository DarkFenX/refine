use crate::{api::RawMAttr, num::UnitInterval};

impl<'s> RawMAttr<'s> {
    /// Return roll quality for the mutated attribute.
    pub fn get_roll(&self) -> UnitInterval {
        let attr_aid = &self.attr_aid;
        self.sol
            .u_data
            .items
            .get(self.item_uid)
            .get_mutation_data()
            .unwrap()
            .get_attr_rolls()
            .get(attr_aid)
            .copied()
            // Raw mutated attributes are not exposed for attributes without mutation data
            .unwrap()
    }
}
