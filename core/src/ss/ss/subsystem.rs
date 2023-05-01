use crate::{
    ss::item::{Item, Subsystem},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_subsystem(&self, item_id: &ReeId) -> Result<&Subsystem> {
        match self.get_item(item_id)? {
            Item::Subsystem(s) => Ok(s),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected Subsystem as item with ID {item_id}"),
            )),
        }
    }
    pub fn get_subsystems(&self, fit_id: ReeId) -> Vec<&Subsystem> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Subsystem(s) if s.fit_id == fit_id => Some(s),
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
