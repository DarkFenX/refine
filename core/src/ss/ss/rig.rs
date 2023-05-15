use crate::{
    defs::{ReeId, ReeInt},
    ss::{
        info::RigInfo,
        item::{Item, Rig},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_rig_info(&self, item_id: &ReeId) -> Result<RigInfo> {
        Ok(self.get_rig(item_id)?.into())
    }
    pub fn get_fit_rig_infos(&self, fit_id: &ReeId) -> Vec<RigInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Rig(r) if r.fit_id == *fit_id => Some(r.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_rig(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<RigInfo> {
        let item_id = self.alloc_item_id()?;
        let rig = Rig::new(&self.src, item_id, fit_id, type_id);
        let info = RigInfo::from(&rig);
        let item = Item::Rig(rig);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_rig_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_rig_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    fn get_rig(&self, item_id: &ReeId) -> Result<&Rig> {
        match self.get_item(item_id)? {
            Item::Rig(r) => Ok(r),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Rig::get_name(), item_id),
            )),
        }
    }
    fn get_rig_mut(&mut self, item_id: &ReeId) -> Result<&mut Rig> {
        match self.get_item_mut(item_id)? {
            Item::Rig(r) => Ok(r),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Rig::get_name(), item_id),
            )),
        }
    }
}
