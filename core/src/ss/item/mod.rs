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
    ss::{SolarSystem, SsView},
    ssi, ssn,
    util::{Error, ErrorKind, Result},
};

impl SolarSystem {
    // Public
    pub fn get_item_info(&self, item_id: &ReeId) -> Result<ssn::SsItemInfo> {
        self.get_item(item_id).map(|v| ssn::SsItemInfo::from_ss_item(v, self))
    }
    pub fn remove_item(&mut self, item_id: &ReeId) -> Result<()> {
        let main = match self.items.get(item_id) {
            Some(item) => item,
            None => return Err(Error::new(ErrorKind::ItemIdNotFound(*item_id))),
        };
        self.svcs.remove_item(&SsView::new(&self.src, &self.items), &main);
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
            ssi::SsItem::Module(m) => match m.charge_a_item_id {
                Some(other_id) => match self.items.remove(&other_id) {
                    Some(charge) => self.svcs.remove_item(&SsView::new(&self.src, &self.items), &charge),
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        };
        self.items.remove(item_id);
        Ok(())
    }
    // Non-public
    fn alloc_item_id(&mut self) -> Result<ReeId> {
        let start = self.item_cnt;
        while self.items.contains_key(&self.item_cnt.0) {
            self.item_cnt += 1;
            if start == self.item_cnt {
                return Err(Error::new(ErrorKind::ItemIdAllocFailed));
            }
        }
        let item_id = self.item_cnt.0;
        self.item_cnt += 1;
        Ok(item_id)
    }
    fn add_item(&mut self, item: ssi::SsItem) {
        let item_id = item.get_id();
        self.items.insert(item_id, item);
        let item = self.items.get(&item_id).unwrap();
        self.svcs.add_item(&SsView::new(&self.src, &self.items), item);
    }
    fn get_item(&self, item_id: &ReeId) -> Result<&ssi::SsItem> {
        self.items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    fn get_item_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::SsItem> {
        self.items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
}
