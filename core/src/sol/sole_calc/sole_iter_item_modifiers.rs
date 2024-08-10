use crate::{
    defs::{EAttrId, SolItemId},
    sol::{svc::SolModificationInfo, SolView, SolarSystem},
};

impl SolarSystem {
    pub fn iter_item_modifiers(
        &mut self,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, Vec<SolModificationInfo>)>> {
        self.svcs
            .calc_iter_item_mods(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item_id)
    }
}
