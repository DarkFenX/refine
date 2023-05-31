use crate::{
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_rig_info(&self, item_id: &ReeId) -> Result<ssn::RigInfo> {
        Ok(self.get_rig(item_id)?.into())
    }
    pub fn get_fit_rig_infos(&self, fit_id: &ReeId) -> Vec<ssn::RigInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                ssi::Item::Rig(r) if r.fit_id == *fit_id => Some(r.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_rig(&mut self, fit_id: ReeId, type_id: ReeInt, state: bool) -> Result<ssn::RigInfo> {
        let item_id = self.alloc_item_id()?;
        let rig = ssi::Rig::new(&self.src, item_id, fit_id, type_id, state);
        let info = ssn::RigInfo::from(&rig);
        let item = ssi::Item::Rig(rig);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_rig_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_rig_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    fn get_rig(&self, item_id: &ReeId) -> Result<&ssi::Rig> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::Item::Rig(rig) => Ok(rig),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::Rig::get_name(),
            ))),
        }
    }
    fn get_rig_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::Rig> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::Item::Rig(rig) => Ok(rig),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::Rig::get_name(),
            ))),
        }
    }
}
