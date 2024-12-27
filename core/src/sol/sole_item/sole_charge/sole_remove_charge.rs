use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_charge(&mut self, item_id: &SolItemId) -> Result<(), RemoveChargeError> {
        let item = self.items.get_item(item_id)?;
        let charge = item.get_charge()?;
        // Remove outgoing projections
        for projectee_item_id in charge.get_projs().iter_items() {
            // Update services for charge
            let projectee_item = self.items.get_item(projectee_item_id).unwrap();
            self.svcs.remove_item_projection(
                &SolView::new(
                    &self.src,
                    &self.fleets,
                    &self.fits,
                    &self.items,
                    &self.default_incoming_dmg,
                ),
                item,
                projectee_item,
            );
            // Update skeleton for charge - do not touch projections container on charge itself,
            // because we're removing it anyway
            self.proj_tracker.unreg_projectee(item_id, projectee_item_id);
        }
        // Update services
        self.svcs.remove_item(
            &SolView::new(
                &self.src,
                &self.fleets,
                &self.fits,
                &self.items,
                &self.default_incoming_dmg,
            ),
            item,
        );
        // Update skeleton
        let module_item_id = charge.get_cont_id();
        let module = self
            .items
            .get_item_mut(&module_item_id)
            .unwrap()
            .get_module_mut()
            .unwrap();
        module.set_charge_id(None);
        self.items.remove_item(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveChargeError {
    ItemNotFound(ItemFoundError),
    ItemIsNotCharge(ItemKindMatchError),
}
impl std::error::Error for RemoveChargeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotCharge(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveChargeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotCharge(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveChargeError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveChargeError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotCharge(error)
    }
}
