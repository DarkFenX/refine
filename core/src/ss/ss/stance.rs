use itertools::Itertools;

use crate::{
    defs::{ReeId, ReeInt},
    ss::{
        item::{Item, Stance, StanceInfo},
        SolarSystem,
    },
    util::{Error, ErrorKind, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_stance_info(&self, fit_id: &ReeId) -> Option<StanceInfo> {
        self.get_fit_stance(fit_id).map(|v| v.into())
    }
    pub fn set_fit_stance(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<StanceInfo> {
        match self.remove_fit_stance(&fit_id) {
            Ok(_) => (),
            // Suppress ItemNotFound error, since this method is supposed to be used
            // even when no stance is set
            Err(e) => match e.kind {
                ErrorKind::ItemNotFound => (),
                _ => return Err(e),
            },
        };
        let item_id = self.alloc_item_id()?;
        let stance = Stance::new(&self.src, item_id, fit_id, type_id);
        let info = StanceInfo::from(&stance);
        let item = Item::Stance(stance);
        self.add_item(item);
        Ok(info)
    }
    pub fn remove_fit_stance(&mut self, fit_id: &ReeId) -> Result<()> {
        self.check_fit(fit_id)?;
        let removed = self
            .items
            .drain_filter(|_, v| match v {
                Item::Stance(s) if s.fit_id == *fit_id => true,
                _ => false,
            })
            .collect_vec();
        match removed.is_empty() {
            true => Err(Error::new(ErrorKind::ItemNotFound, "stance not found")),
            false => Ok(()),
        }
    }
    // Non-public
    fn get_fit_stance(&self, fit_id: &ReeId) -> Option<&Stance> {
        self.items.values().find_map(|v| match v {
            Item::Stance(s) if s.fit_id == *fit_id => Some(s),
            _ => None,
        })
    }
}
