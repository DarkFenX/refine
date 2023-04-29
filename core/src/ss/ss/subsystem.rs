use crate::{
    ss::item::{Item, Subsystem},
    ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_subsystems(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Subsystem(s) if s.fit_id == fit_id => Some(s.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_subsystem(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let subsystem = Item::Subsystem(Subsystem::new(&self.src, item_id, fit_id, type_id));
        self.add_item(subsystem);
        Ok(item_id)
    }
}
