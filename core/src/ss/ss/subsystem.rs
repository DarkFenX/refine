use crate::{
    ss::item::{Item, Subsystem, SubsystemInfo},
    util::Named,
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    // Public
    pub fn get_subsystem_info(&self, item_id: &ReeId) -> Result<SubsystemInfo> {
        Ok(self.get_subsystem(item_id)?.into())
    }
    pub fn get_fit_subsystem_infos(&self, fit_id: &ReeId) -> Vec<SubsystemInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Subsystem(s) if s.fit_id == *fit_id => Some(s.into()),
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
    // Non-public
    fn get_subsystem(&self, item_id: &ReeId) -> Result<&Subsystem> {
        match self.get_item(item_id)? {
            Item::Subsystem(s) => Ok(s),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Subsystem::get_name(), item_id),
            )),
        }
    }
    fn get_subsystem_mut(&mut self, item_id: &ReeId) -> Result<&mut Subsystem> {
        match self.get_item_mut(item_id)? {
            Item::Subsystem(s) => Ok(s),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Subsystem::get_name(), item_id),
            )),
        }
    }
}
