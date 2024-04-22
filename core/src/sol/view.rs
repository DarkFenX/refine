use crate::{
    sol::{fit::SolFits, fleet::SolFleets, item::SolItems},
    src::Src,
};

// Should include all the solar system data, except for services
// (since it's intended to be passed to services)
pub(in crate::sol) struct SolView<'a> {
    pub(in crate::sol) src: &'a Src,
    pub(in crate::sol) fleets: &'a SolFleets,
    pub(in crate::sol) fits: &'a SolFits,
    pub(in crate::sol) items: &'a SolItems,
}
impl<'a> SolView<'a> {
    pub(in crate::sol) fn new(src: &'a Src, fleets: &'a SolFleets, fits: &'a SolFits, items: &'a SolItems) -> Self {
        Self {
            src,
            fleets,
            fits,
            items,
        }
    }
}
