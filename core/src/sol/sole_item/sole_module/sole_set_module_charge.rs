use itertools::Itertools;

use crate::{
    defs::{EItemId, SolItemId},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        item::{SolCharge, SolItem},
        item_info::SolChargeInfo,
        SolView, SolarSystem,
    },
};

impl SolarSystem {
    pub fn set_module_charge(
        &mut self,
        item_id: &SolItemId,
        charge_type_id: EItemId,
    ) -> Result<SolChargeInfo, SetModuleChargeError> {
        let module = self.items.get_item(item_id)?.get_module()?;
        let module_projs = module.get_projs().iter().map(|(i, r)| (*i, *r)).collect_vec();
        // Remove old charge, if it was set
        if let Some(charge_id) = module.get_charge_id() {
            // Remove outgoing projections
            let charge_item = self.items.get_item(&charge_id).unwrap();
            // Use module projections because they should be identical
            for (projectee_item_id, _) in module_projs.iter() {
                // Update services for charge being removed
                let projectee_item = self.items.get_item(projectee_item_id).unwrap();
                self.svcs.remove_item_projection(
                    &SolView::new(
                        &self.src,
                        &self.fleets,
                        &self.fits,
                        &self.items,
                        &self.default_incoming_dmg,
                    ),
                    charge_item,
                    projectee_item,
                );
                // Update skeleton for charge - do not touch projections container on charge itself,
                // because we're removing it anyway
                self.proj_tracker.unreg_projectee(&charge_id, projectee_item_id);
            }
            // Update services for charge being removed
            self.svcs.remove_item(
                &SolView::new(
                    &self.src,
                    &self.fleets,
                    &self.fits,
                    &self.items,
                    &self.default_incoming_dmg,
                ),
                charge_item,
            );
            // Update skeleton for charge - do not update module<->charge references because charge
            // will be removed, and module will be updated later
            self.items.remove_item(&charge_id);
        };
        // Set new charge
        // Allocation can fail only if we didn't remove charge first, so if it fails - we don't need
        // to restore anything
        let charge_id = self.items.alloc_item_id();
        // Update skeleton
        let module = self.items.get_item_mut(item_id).unwrap().get_module_mut().unwrap();
        module.set_charge_id(Some(charge_id));
        let fit_id = module.get_fit_id();
        let charge = SolCharge::new(
            &self.src,
            charge_id,
            charge_type_id,
            fit_id,
            *item_id,
            module.get_state(),
            false,
        );
        let charge_info = SolChargeInfo::from(&charge);
        let charge_item = SolItem::Charge(charge);
        self.items.add_item(charge_item);
        // Update services
        self.add_item_id_to_svcs(&charge_id);
        // Reapply module projections to charge
        if !module_projs.is_empty() {
            let charge_projs = self
                .items
                .get_item_mut(&charge_id)
                .unwrap()
                .get_charge_mut()
                .unwrap()
                .get_projs_mut();
            // Update skeleton for charge
            for (projectee_item_id, range) in module_projs.iter() {
                self.proj_tracker.reg_projectee(charge_id, *projectee_item_id);
                charge_projs.add(*projectee_item_id, *range);
            }
            // Update services for charge
            let charge_item = self.items.get_item(&charge_id).unwrap();
            for (projectee_item_id, range) in module_projs {
                let projectee_item = self.items.get_item(&projectee_item_id).unwrap();
                self.svcs.add_item_projection(
                    &SolView::new(
                        &self.src,
                        &self.fleets,
                        &self.fits,
                        &self.items,
                        &self.default_incoming_dmg,
                    ),
                    charge_item,
                    projectee_item,
                    range,
                );
            }
        }
        Ok(charge_info)
    }
}

#[derive(Debug)]
pub enum SetModuleChargeError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
}
impl std::error::Error for SetModuleChargeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotModule(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetModuleChargeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotModule(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for SetModuleChargeError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetModuleChargeError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotModule(error)
    }
}
