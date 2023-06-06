use crate::{
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_implant_info(&self, item_id: &ReeId) -> Result<ssn::SsImplantInfo> {
        Ok(self.get_implant(item_id)?.into())
    }
    pub fn get_fit_implant_infos(&self, fit_id: &ReeId) -> Vec<ssn::SsImplantInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                ssi::SsItem::Implant(i) if i.fit_id == *fit_id => Some(i.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_implant(&mut self, fit_id: ReeId, a_item_id: ReeInt, state: bool) -> Result<ssn::SsImplantInfo> {
        let item_id = self.alloc_item_id()?;
        let implant = ssi::SsImplant::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsImplantInfo::from(&implant);
        let item = ssi::SsItem::Implant(implant);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_implant_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_implant_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    fn get_implant(&self, item_id: &ReeId) -> Result<&ssi::SsImplant> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsImplant::get_name(),
            ))),
        }
    }
    fn get_implant_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::SsImplant> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsImplant::get_name(),
            ))),
        }
    }
}
