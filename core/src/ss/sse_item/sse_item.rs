use itertools::Itertools;

use crate::{
    defs::SsItemId,
    ss::{item::SsItem, item_info::SsItemInfo, SolarSystem, SsView},
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_item_info(&self, item_id: &SsItemId) -> Result<SsItemInfo> {
        self.items.get_item(item_id).map(|v| SsItemInfo::from_ss_item(v, self))
    }
    pub fn remove_item(&mut self, item_id: &SsItemId) -> Result<()> {
        // Gather info to remove child or update parent items
        let main = self.items.get_item(item_id)?;
        let charge_id_opt = match main {
            SsItem::Module(m) => m.charge_item_id,
            _ => None,
        };
        let parent_id_opt = match main {
            SsItem::Charge(charge) => Some(charge.cont_id),
            _ => None,
        };
        let proj_item_ids = self.tgt_tracker.iter_srcs(item_id).map(|v| *v).collect_vec();
        let mut targeted_item_ids = Vec::new();
        match main {
            SsItem::ProjEffect(proj_effect) => targeted_item_ids.extend(proj_effect.tgts.iter_tgts()),
            _ => (),
        };
        // Remove projections which target item being removed
        for proj_item_id in proj_item_ids.iter() {
            let _ = self.remove_proj_effect_tgt(proj_item_id, item_id);
        }
        // Remove projections of current item upon others
        for targeted_item_id in targeted_item_ids.iter() {
            let _ = self.remove_proj_effect_tgt(item_id, targeted_item_id);
        }
        // Remove child items
        if let Some(charge_id) = charge_id_opt {
            let charge = self.items.get_item(&charge_id)?;
            self.svcs
                .remove_item(&SsView::new(&self.src, &self.fleets, &self.fits, &self.items), charge);
            self.items.remove_item(&charge_id);
        };
        // Handle item itself
        let main = self.items.get_item(item_id)?;
        self.svcs
            .remove_item(&SsView::new(&self.src, &self.fleets, &self.fits, &self.items), &main);
        if let Some(fit_id) = main.get_fit_id() {
            let fit = self.fits.get_fit_mut(&fit_id)?;
            fit.remove_item(main);
        }
        match main {
            SsItem::SwEffect(_) => {
                self.sw_effects.remove(item_id);
                ()
            }
            SsItem::ProjEffect(_) => {
                self.proj_effects.remove(item_id);
                ()
            }
            _ => (),
        }
        self.items.remove_item(item_id);
        // Update parent item
        if let Some(parent_id) = parent_id_opt {
            let parent = self.items.get_item_mut(&parent_id)?;
            if let SsItem::Module(m) = parent {
                m.charge_item_id = None
            }
        }
        Ok(())
    }
    // Non-public
    pub(in crate::ss::sse_item) fn add_item(&mut self, item: SsItem) {
        let item_id = item.get_id();
        match item {
            SsItem::SwEffect(_) => {
                self.sw_effects.insert(item_id);
                ()
            }
            SsItem::ProjEffect(_) => {
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
        let item = self.items.get_item(&item_id).unwrap();
        self.svcs
            .add_item(&SsView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
    }
}
