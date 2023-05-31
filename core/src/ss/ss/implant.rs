use crate::{
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_implant_info(&self, item_id: &ReeId) -> Result<ssn::ImplantInfo> {
        Ok(self.get_implant(item_id)?.into())
    }
    pub fn get_fit_implant_infos(&self, fit_id: &ReeId) -> Vec<ssn::ImplantInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                ssi::Item::Implant(i) if i.fit_id == *fit_id => Some(i.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_implant(&mut self, fit_id: ReeId, type_id: ReeInt, state: bool) -> Result<ssn::ImplantInfo> {
        let item_id = self.alloc_item_id()?;
        let implant = ssi::Implant::new(&self.src, item_id, fit_id, type_id, state);
        let info = ssn::ImplantInfo::from(&implant);
        let item = ssi::Item::Implant(implant);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_implant_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_implant_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    fn get_implant(&self, item_id: &ReeId) -> Result<&ssi::Implant> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::Item::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::Implant::get_name(),
            ))),
        }
    }
    fn get_implant_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::Implant> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::Item::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::Implant::get_name(),
            ))),
        }
    }
}
