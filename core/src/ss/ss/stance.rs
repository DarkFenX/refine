use itertools::Itertools;

use crate::{
    ss::item::{Item, Stance},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_stance_id(&self, fit_id: ReeId) -> Option<ReeId> {
        self.items
            .values()
            .find(|v| match v {
                Item::Stance(s) if s.fit_id == fit_id => true,
                _ => false,
            })
            .map(|v| v.get_id())
    }
    pub fn set_stance(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        self.remove_stance(fit_id)?;
        let item_id = self.alloc_item_id()?;
        let stance = Item::Stance(Stance::new(&self.src, item_id, fit_id, type_id));
        self.add_item(stance);
        Ok(item_id)
    }
    pub fn remove_stance(&mut self, fit_id: ReeId) -> Result<bool> {
        if !self.fits.contains(&fit_id) {
            return Err(Error::new(ErrorKind::FitNotFound, "fit not found"));
        }
        let removed = self
            .items
            .drain_filter(|_, v| match v {
                Item::Stance(s) if s.fit_id == fit_id => true,
                _ => false,
            })
            .collect_vec();
        Ok(!removed.is_empty())
    }
}
