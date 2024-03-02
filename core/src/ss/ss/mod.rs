use std::collections::{HashMap, HashSet};

use crate::{
    defs::{EAttrId, EEffectId, SsFitId, SsItemId},
    src::Src,
    ss::{fit::SsFits, item::SsItems, svc::SsSvcs, EffectInfo, EffectMode, SsAttrVal, SsView},
    util::Result,
};
pub use ord_modes::{OrdAddMode, OrdRmMode};

mod item;
mod ord_modes;

pub struct SolarSystem {
    src: Src,
    fits: SsFits,
    // fleets will go here
    items: SsItems,
    svcs: SsSvcs,
    sw_effects: HashSet<SsItemId>,
}
impl SolarSystem {
    pub fn new(src: Src) -> Self {
        Self {
            src,
            fits: SsFits::new(),
            items: SsItems::new(),
            svcs: SsSvcs::new(),
            sw_effects: HashSet::new(),
        }
    }
    pub fn set_src(&mut self, src: Src) {
        for item in self.items.iter_mut() {
            item.reload_a_item(&src)
        }
        self.src = src;
        // TODO: make sure attributes and attribute caps are cleared when source
        // is switched or item is reloaded (as well as stuff in other services)
    }
    // Fits
    pub fn add_fit(&mut self) -> Result<SsFitId> {
        let fit_id = self.fits.add_fit()?;
        self.svcs.add_fit(&fit_id);
        Ok(fit_id)
    }
    pub fn remove_fit(&mut self, fit_id: &SsFitId) -> Result<()> {
        for item_id in self.fits.get_fit(fit_id)?.all_items().iter() {
            self.remove_item(item_id).unwrap();
        }
        self.svcs.remove_fit(&fit_id);
        self.fits.remove_fit(fit_id)?;
        Ok(())
    }
    pub fn get_fit_ids(&self) -> Vec<SsFitId> {
        self.fits.iter_fit_ids().map(|v| *v).collect()
    }
    // Item attributes
    pub fn get_item_attr(&mut self, item_id: &SsItemId, attr_id: &EAttrId) -> Result<SsAttrVal> {
        self.svcs
            .calc_get_item_attr_val(&SsView::new(&self.src, &self.fits, &self.items), item_id, attr_id)
    }
    pub fn get_item_attrs(&mut self, item_id: &SsItemId) -> Result<HashMap<EAttrId, SsAttrVal>> {
        self.svcs
            .calc_get_item_attr_vals(&SsView::new(&self.src, &self.fits, &self.items), item_id)
    }
    // Item effects
    pub fn get_item_effects(&self, item_id: &SsItemId) -> Result<HashMap<EEffectId, EffectInfo>> {
        let item = self.items.get_item(item_id)?;
        let a_effect_ids = item.get_effect_datas()?.keys();
        let effect_infos = a_effect_ids
            .map(|v| {
                let running = self.svcs.is_effect_running(item_id, v);
                let mode = item.get_effect_modes().get(v);
                (*v, EffectInfo::new(running, *mode))
            })
            .collect();
        Ok(effect_infos)
    }
    pub fn set_item_effect_mode(&mut self, item_id: &SsItemId, effect_id: &EEffectId, mode: EffectMode) -> Result<()> {
        self.items
            .get_item_mut(item_id)?
            .get_effect_modes_mut()
            .set(*effect_id, mode);
        let item = self.items.get_item(item_id).unwrap();
        self.svcs
            .process_effects(&SsView::new(&self.src, &self.fits, &self.items), item, item.get_state());
        Ok(())
    }
    pub fn set_item_effect_modes(
        &mut self,
        item_id: &SsItemId,
        mode_map: &HashMap<EEffectId, EffectMode>,
    ) -> Result<()> {
        let effect_modes = self.items.get_item_mut(item_id)?.get_effect_modes_mut();
        for (effect_id, effect_mode) in mode_map.iter() {
            effect_modes.set(*effect_id, *effect_mode)
        }
        let item = self.items.get_item(item_id).unwrap();
        self.svcs
            .process_effects(&SsView::new(&self.src, &self.fits, &self.items), item, item.get_state());
        Ok(())
    }
}
