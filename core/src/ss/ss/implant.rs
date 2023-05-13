use crate::{
    defines::{ReeId, ReeInt},
    ss::{
        item::{Implant, ImplantInfo, Item},
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
        match self.get_item(item_id)? {
            Item::Implant(i) => Ok(i),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Implant::get_name(), item_id),
            )),
        }
    }
    fn get_implant_mut(&mut self, item_id: &ReeId) -> Result<&mut Implant> {
        match self.get_item_mut(item_id)? {
            Item::Implant(i) => Ok(i),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Implant::get_name(), item_id),
            )),
        }
    }
}
