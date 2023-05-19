use crate::{
    defs::{ReeId, ReeInt},
    ss::{
        info::ImplantInfo,
        item::{Implant, Item},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_implant_info(&self, item_id: &ReeId) -> Result<ImplantInfo> {
        Ok(self.get_implant(item_id)?.into())
    }
    pub fn get_fit_implant_infos(&self, fit_id: &ReeId) -> Vec<ImplantInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Implant(i) if i.fit_id == *fit_id => Some(i.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_implant(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ImplantInfo> {
        let item_id = self.alloc_item_id()?;
        let implant = Implant::new(&self.src, item_id, fit_id, type_id);
        let info = ImplantInfo::from(&implant);
        let item = Item::Implant(implant);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_implant_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_implant_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    fn get_implant(&self, item_id: &ReeId) -> Result<&Implant> {
        let item = self.get_item(item_id)?;
        match item {
            Item::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                Implant::get_name(),
            ))),
        }
    }
    fn get_implant_mut(&mut self, item_id: &ReeId) -> Result<&mut Implant> {
        let item = self.get_item_mut(item_id)?;
        match item {
            Item::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                Implant::get_name(),
            ))),
        }
    }
}
