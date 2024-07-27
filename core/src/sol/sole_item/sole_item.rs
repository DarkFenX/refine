use itertools::Itertools;

use crate::{
    defs::SolItemId,
    sol::{item::SolItem, item_info::SolItemInfo, SolView, SolarSystem},
    util::{Error, ErrorKind, Result},
};

impl SolarSystem {
    // Public
    pub fn get_item_info(&self, item_id: &SolItemId) -> Result<SolItemInfo> {
        self.items
            .get_item(item_id)
            .map(|v| SolItemInfo::from_sol_item(v, self))
    }
    pub fn remove_item(&mut self, item_id: &SolItemId) -> Result<()> {
        // Gather info for further process
        let main = self.items.get_item(item_id)?;
        if matches!(main, SolItem::AutoCharge(_)) {
            return Err(Error::new(ErrorKind::UnremovableItemKind(main.get_name())));
        }
        let charge_id = match main {
            SolItem::Module(m) => m.charge_item_id,
            _ => None,
        };
        let autocharge_ids = main.get_autocharges().map(|v| v.values().map(|w| *w).collect_vec());
        let parent_id = match main {
            SolItem::Charge(charge) => Some(charge.cont_id),
            _ => None,
        };
        // Remove outgoing projections
        match main {
            SolItem::ProjEffect(proj_effect) => {
                let proj_outgoing = proj_effect.projs.iter_items().map(|v| *v).collect_vec();
                for projectee_item_id in proj_outgoing.iter() {
                    self.remove_proj_effect_proj(item_id, projectee_item_id).unwrap();
                }
            }
            SolItem::Module(module) => {
                let proj_outgoing = module.projs.iter_items().map(|v| *v).collect_vec();
                for projectee_item_id in proj_outgoing.iter() {
                    self.remove_module_proj(item_id, projectee_item_id).unwrap();
                }
            }
            _ => (),
        };
        // Remove incoming projections
        let proj_incoming = self.proj_tracker.iter_projectors(item_id).map(|v| *v).collect_vec();
        for proj_item_id in proj_incoming.iter() {
            let proj_item = self.items.get_item(proj_item_id).unwrap();
            match proj_item {
                SolItem::Module(_) => self.remove_module_proj(proj_item_id, item_id).unwrap(),
                SolItem::ProjEffect(_) => self.remove_proj_effect_proj(proj_item_id, item_id).unwrap(),
                _ => (),
            }
        }
        // Remove child items
        if let Some(charge_id) = charge_id {
            let charge = self.items.get_item(&charge_id).unwrap();
            self.svcs
                .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), charge);
            self.items.remove_item(&charge_id);
        };
        if let Some(autocharge_ids) = autocharge_ids {
            for autocharge_id in autocharge_ids {
                let autocharge = self.items.get_item(&autocharge_id).unwrap();
                self.svcs.remove_item(
                    &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                    autocharge,
                );
                self.items.remove_item(&autocharge_id);
            }
        }
        // Handle item itself
        let main = self.items.get_item(item_id)?;
        self.svcs
            .remove_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), &main);
        if let Some(fit_id) = main.get_fit_id() {
            let fit = self.fits.get_fit_mut(&fit_id)?;
            fit.remove_item(main);
        }
        match main {
            SolItem::SwEffect(_) => {
                self.sw_effects.remove(item_id);
                ()
            }
            SolItem::ProjEffect(_) => {
                self.proj_effects.remove(item_id);
                ()
            }
            _ => (),
        }
        self.items.remove_item(item_id);
        // Update parent item
        if let Some(parent_id) = parent_id {
            let parent = self.items.get_item_mut(&parent_id)?;
            if let SolItem::Module(m) = parent {
                m.charge_item_id = None
            }
        }
        Ok(())
    }
    // Non-public
    pub(in crate::sol::sole_item) fn add_item(&mut self, item: SolItem) {
        let item_id = item.get_id();
        match item {
            SolItem::SwEffect(_) => {
                self.sw_effects.insert(item_id);
                ()
            }
            SolItem::ProjEffect(_) => {
                self.proj_effects.insert(item_id);
                ()
            }
            _ => (),
        }
        if let Some(fit_id) = item.get_fit_id() {
            let fit = self.fits.get_fit_mut(&fit_id).unwrap();
            fit.add_item(&item)
        }
        self.items.add_item(item);
        self.add_item_to_svc(&item_id);
    }
    pub(in crate::sol::sole_item) fn add_item_to_svc(&mut self, item_id: &SolItemId) {
        let item = self.items.get_item(&item_id).unwrap();
        self.svcs
            .add_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
    }
}
