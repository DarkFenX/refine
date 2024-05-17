use crate::{
    defs::{EItemId, SolItemId},
    sol::{
        item::{SolItem, SolProjEffect},
        item_info::SolProjEffectInfo,
        SolView, SolarSystem,
    },
    util::{Error, ErrorKind, Result},
};

impl SolarSystem {
    // Public
    pub fn get_proj_effect_info(&self, item_id: &SolItemId) -> Result<SolProjEffectInfo> {
        Ok(self.items.get_proj_effect(item_id)?.into())
    }
    pub fn get_proj_effect_infos(&self) -> Vec<SolProjEffectInfo> {
        self.proj_effects
            .iter()
            .map(|v| self.items.get_proj_effect(v).unwrap().into())
            .collect()
    }
    pub fn add_proj_effect(&mut self, a_item_id: EItemId, state: bool) -> Result<SolProjEffectInfo> {
        let item_id = self.items.alloc_item_id()?;
        let proj_effect = SolProjEffect::new(&self.src, item_id, a_item_id, state);
        let info = SolProjEffectInfo::from(&proj_effect);
        let item = SolItem::ProjEffect(proj_effect);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_proj_effect_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        let proj_effect = self.items.get_proj_effect_mut(item_id)?;
        let old_state = proj_effect.state;
        proj_effect.set_bool_state(state);
        let new_state = proj_effect.state;
        if new_state != old_state {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs.switch_item_state(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                item,
                old_state,
                new_state,
            );
        }
        Ok(())
    }
    pub fn add_proj_effect_tgt(&mut self, item_id: &SolItemId, tgt_item_id: SolItemId) -> Result<()> {
        let proj_effect = self.items.get_proj_effect(item_id)?;
        if proj_effect.projs.contains(&tgt_item_id) {
            return Ok(());
        }
        let tgt_item = self.items.get_item(&tgt_item_id)?;
        if !tgt_item.is_targetable() {
            return Err(Error::new(ErrorKind::ItemNotTargetable(tgt_item_id)));
        }
        self.tgt_tracker.reg_tgt(*item_id, tgt_item_id);
        let proj_effect = self.items.get_proj_effect_mut(item_id)?;
        proj_effect.projs.add(tgt_item_id, None);
        let item = self.items.get_item(item_id).unwrap();
        let tgt_item = self.items.get_item(&tgt_item_id).unwrap();
        self.svcs.add_item_tgt(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            &item,
            tgt_item,
            None,
        );
        Ok(())
    }
    pub fn remove_proj_effect_tgt(&mut self, item_id: &SolItemId, tgt_item_id: &SolItemId) -> Result<()> {
        // Check if target is defined
        let proj_effect = self.items.get_proj_effect(item_id)?;
        if !proj_effect.projs.contains(tgt_item_id) {
            return Err(Error::new(ErrorKind::TargetNotFound(*item_id, *tgt_item_id)));
        };
        let item = self.items.get_item(item_id)?;
        let tgt_item = self.items.get_item(tgt_item_id)?;
        self.svcs.remove_item_tgt(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item,
            tgt_item,
        );
        self.tgt_tracker.unreg_tgt(item_id, tgt_item_id);
        let proj_effect = self.items.get_proj_effect_mut(item_id)?;
        proj_effect.projs.remove(tgt_item_id);
        Ok(())
    }
}
