mod booster;
mod character;
mod charge;
mod drone;
mod fighter;
mod implant;
mod module;
mod rig;
mod ship;
mod skill;
mod stance;
mod subsystem;
mod sw_effect;

use crate::{
    defs::ReeId,
    ss::{info::SsItemInfo, item::SsItem, SolarSystem, SsView},
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_item_info(&self, item_id: &ReeId) -> Result<SsItemInfo> {
        self.items.get_item(item_id).map(|v| SsItemInfo::from_ss_item(v, self))
    }
    pub fn remove_item(&mut self, item_id: &ReeId) -> Result<()> {
        let main = self.items.get_item(item_id)?;
        self.svcs
            .remove_item(&SsView::new(&self.src, &self.fits, &self.items), &main);
        if let Some(fit_id) = main.get_fit_id() {
            self.fits.get_fit_mut(&fit_id)?.remove_item(main);
        }
        match main {
            // Remove reference to charge if it's charge which we're removing
            // Item::Charge(c) => match self.items.get_mut(&c.cont_id) {
            //     None => {
            //         self.items.remove(item_id);
            //         return Ok(())
            //     },
            //     Some(other) => match other {
            //         Item::Module(m) => m.charge_id = None,
            //         _ => (),
            //     },
            // },
            // Remove charge if we're removing a module, charges cannot exist without their carrier
            SsItem::Module(m) => match m.charge_a_item_id {
                Some(other_id) => match self.items.remove_item(&other_id) {
                    Some(charge) => self
                        .svcs
                        .remove_item(&SsView::new(&self.src, &self.fits, &self.items), &charge),
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        };
        Ok(())
    }
    // Non-public
    fn add_item(&mut self, item: SsItem) {
        let item_id = item.get_id();
        if let Some(fit_id) = item.get_fit_id() {
            let fit = self.fits.get_fit_mut(&fit_id).unwrap();
            fit.add_item(&item)
        }
        self.items.add_item(item);
        let item = self.items.get_item(&item_id).unwrap();
        self.svcs
            .add_item(&SsView::new(&self.src, &self.fits, &self.items), item);
    }
}
