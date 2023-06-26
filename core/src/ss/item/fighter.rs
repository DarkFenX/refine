use crate::{
    consts::State,
    defs::{ReeInt, SsFitId, SsItemId},
    ss::SolarSystem,
    ssi, ssn,
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_fighter_info(&self, item_id: &SsItemId) -> Result<ssn::SsFighterInfo> {
        Ok(self.items.get_fighter(item_id)?.into())
    }
    pub fn get_fit_fighter_infos(&self, fit_id: &SsFitId) -> Result<Vec<ssn::SsFighterInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let fighter_infos = fit
            .fighters
            .iter()
            .map(|v| self.items.get_fighter(v).unwrap().into())
            .collect();
        Ok(fighter_infos)
    }
    pub fn add_fighter(&mut self, fit_id: SsFitId, a_item_id: ReeInt, state: State) -> Result<ssn::SsFighterInfo> {
        let item_id = self.items.alloc_item_id()?;
        let fighter = ssi::SsFighter::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsFighterInfo::from(&fighter);
        let item = ssi::SsItem::Fighter(fighter);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_fighter_state(&mut self, item_id: &SsItemId, state: State) -> Result<()> {
        self.items.get_fighter_mut(item_id)?.state = state;
        Ok(())
    }
}
