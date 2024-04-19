use crate::{
    defs::{EAttrId, EEffectId, SsItemId},
    src::Src,
    ss::{
        fit::SsFits, fleet::SsFleets, item::SsItems, misc::TgtTracker, svc::SsSvcs, EffectInfo, EffectMode, SsAttrVal,
        SsView,
    },
    util::{Result, StSet},
    SsModInfo,
};

// Solar system glues everything together and is actual "god object" of the lib. It controls source
// which will be used for data and general item structure - including their type, IDs, which fit
// they belong to, which charges they have etc. But all the processing for those items (e.g.
// attribute calculation) happens in services, which are also stored on solar system, but are
// somewhat isolated.
pub struct SolarSystem {
    pub(in crate::ss) src: Src,
    pub(in crate::ss) fleets: SsFleets,
    pub(in crate::ss) fits: SsFits,
    pub(in crate::ss) items: SsItems,
    pub(in crate::ss) sw_effects: StSet<SsItemId>,
    pub(in crate::ss) proj_effects: StSet<SsItemId>,
    pub(in crate::ss) tgt_tracker: TgtTracker,
    pub(in crate::ss) svcs: SsSvcs,
}
impl SolarSystem {
    pub fn new(src: Src) -> Self {
        Self {
            src,
            fleets: SsFleets::new(),
            fits: SsFits::new(),
            items: SsItems::new(),
            sw_effects: StSet::new(),
            proj_effects: StSet::new(),
            tgt_tracker: TgtTracker::new(),
            svcs: SsSvcs::new(),
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
    // Item attributes
    pub fn get_item_attr(&mut self, item_id: &SsItemId, attr_id: &EAttrId) -> Result<SsAttrVal> {
        self.svcs.calc_get_item_attr_val(
            &SsView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item_id,
            attr_id,
        )
    }
    pub fn get_item_attrs(
        &mut self,
        item_id: &SsItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, SsAttrVal)>> {
        self.svcs
            .calc_iter_item_attr_vals(&SsView::new(&self.src, &self.fleets, &self.fits, &self.items), item_id)
    }
    // Item modifications
    pub fn iter_item_modifiers(
        &mut self,
        item_id: &SsItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, Vec<SsModInfo>)>> {
        self.svcs
            .calc_iter_item_mods(&SsView::new(&self.src, &self.fleets, &self.fits, &self.items), item_id)
    }
    // Item effects
    pub fn iter_item_effects<'a>(
        &'a self,
        item_id: &'a SsItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EEffectId, EffectInfo)> + 'a> {
        let item = self.items.get_item(item_id)?;
        let a_effect_ids = item.get_effect_datas()?.keys();
        let effect_infos = a_effect_ids.map(move |v| {
            let running = self.svcs.is_effect_running(item_id, v);
            let mode = item.get_effect_modes().get(v);
            (*v, EffectInfo::new(running, *mode))
        });
        Ok(effect_infos)
    }
    pub fn set_item_effect_mode(&mut self, item_id: &SsItemId, effect_id: &EEffectId, mode: EffectMode) -> Result<()> {
        self.items
            .get_item_mut(item_id)?
            .get_effect_modes_mut()
            .set(*effect_id, mode);
        let item = self.items.get_item(item_id).unwrap();
        self.svcs.process_effects(
            &SsView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item,
            item.get_state(),
        );
        Ok(())
    }
    pub fn set_item_effect_modes(
        &mut self,
        item_id: &SsItemId,
        modes: impl Iterator<Item = (EEffectId, EffectMode)>,
    ) -> Result<()> {
        let effect_modes = self.items.get_item_mut(item_id)?.get_effect_modes_mut();
        for (effect_id, effect_mode) in modes {
            effect_modes.set(effect_id, effect_mode)
        }
        let item = self.items.get_item(item_id).unwrap();
        self.svcs.process_effects(
            &SsView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item,
            item.get_state(),
        );
        Ok(())
    }
}
