use std::collections::{HashMap, HashSet};

use crate::{
    consts::DEFAULT_EFFECT_MODE,
    defs::{AttrId, EffectId, SsFitId, SsItemId},
    src::Src,
    ss::{effect_info::EffectInfo, fit::SsFits, item::SsItems, svc::SsSvcs, SsAttrVal, SsView},
    util::Result,
};

mod item;

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
        self.fits.add_fit()
    }
    pub fn remove_fit(&mut self, fit_id: &SsFitId) -> Result<()> {
        self.fits.remove_fit(fit_id)?;
        self.items.remove_fit_items(fit_id);
        Ok(())
    }
    pub fn get_fit_ids(&self) -> Vec<SsFitId> {
        self.fits.get_fit_ids()
    }
    // Item attributes
    pub fn get_item_attr(&mut self, item_id: &SsItemId, attr_id: &AttrId) -> Result<SsAttrVal> {
        self.svcs
            .calc_get_item_attr_val(&SsView::new(&self.src, &self.fits, &self.items), item_id, attr_id)
    }
    pub fn get_item_attrs(&mut self, item_id: &SsItemId) -> Result<HashMap<AttrId, SsAttrVal>> {
        self.svcs
            .calc_get_item_attr_vals(&SsView::new(&self.src, &self.fits, &self.items), item_id)
    }
    // Item effects
    pub fn get_item_effects(&mut self, item_id: &SsItemId) -> Result<HashMap<EffectId, EffectInfo>> {
        let item = self.items.get_item(item_id)?;
        let a_effect_ids = item.get_effect_datas()?.keys();
        let running_effect_ids = self.svcs.get_running_effects(item_id);
        let effect_infos = a_effect_ids
            .map(|v| {
                let running = match running_effect_ids {
                    Some(effect_ids) => effect_ids.contains(v),
                    None => false,
                };
                let mode = item.get_effect_modes().get(v).unwrap_or(&DEFAULT_EFFECT_MODE);
                (*v, EffectInfo::new(running, *mode))
            })
            .collect();
        Ok(effect_infos)
    }
}
