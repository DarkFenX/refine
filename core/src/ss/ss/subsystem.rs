use crate::{
    defs::{ReeId, ReeInt},
    ss::{
        info::SubsystemInfo,
        item::{Item, Subsystem},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
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
    pub fn add_subsystem(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<SubsystemInfo> {
        let item_id = self.alloc_item_id()?;
        let subsystem = Subsystem::new(&self.src, item_id, fit_id, type_id);
        let info = SubsystemInfo::from(&subsystem);
        let item = Item::Subsystem(subsystem);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_subsystem_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_subsystem_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    fn get_subsystem(&self, item_id: &ReeId) -> Result<&Subsystem> {
        let item = self.get_item(item_id)?;
        match item {
            Item::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                Subsystem::get_name(),
            ))),
        }
    }
    fn get_subsystem_mut(&mut self, item_id: &ReeId) -> Result<&mut Subsystem> {
        let item = self.get_item_mut(item_id)?;
        match item {
            Item::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                Subsystem::get_name(),
            ))),
        }
    }
}
