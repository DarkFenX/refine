use crate::{
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_subsystem_info(&self, item_id: &ReeId) -> Result<ssn::SsSubsystemInfo> {
        Ok(self.get_subsystem(item_id)?.into())
    }
    pub fn get_fit_subsystem_infos(&self, fit_id: &ReeId) -> Vec<ssn::SsSubsystemInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                ssi::SsItem::Subsystem(s) if s.fit_id == *fit_id => Some(s.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_subsystem(&mut self, fit_id: ReeId, a_item_id: ReeInt, state: bool) -> Result<ssn::SsSubsystemInfo> {
        let item_id = self.alloc_item_id()?;
        let subsystem = ssi::SsSubsystem::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsSubsystemInfo::from(&subsystem);
        let item = ssi::SsItem::Subsystem(subsystem);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_subsystem_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_subsystem_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    fn get_subsystem(&self, item_id: &ReeId) -> Result<&ssi::SsSubsystem> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSubsystem::get_name(),
            ))),
        }
    }
    fn get_subsystem_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::SsSubsystem> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSubsystem::get_name(),
            ))),
        }
    }
}
