use crate::{
    defs::{EAttrId, EEffectId, SolItemId},
    sol::{
        fit::SolFits, fleet::SolFleets, item::SolItems, svc::SolSvcs, SolAttrVal, SolEffectInfo, SolEffectMode,
        SolProjTracker, SolView,
    },
    src::Src,
    util::{Result, StSet},
    SolModificationInfo,
};

// Solar system glues everything together and is actual "god object" of the lib. It controls source
// which will be used for data and general item structure - including their kind, type IDs, which
// fit they belong to, which charges they have etc. But all the processing for those items (e.g.
// attribute calculation) happens in services, which are also stored on solar system, but are
// somewhat isolated.
pub struct SolarSystem {
    pub(in crate::sol) src: Src,
    pub(in crate::sol) fleets: SolFleets,
    pub(in crate::sol) fits: SolFits,
    pub(in crate::sol) items: SolItems,
    pub(in crate::sol) sw_effects: StSet<SolItemId>,
    pub(in crate::sol) proj_effects: StSet<SolItemId>,
    pub(in crate::sol) proj_tracker: SolProjTracker,
    pub(in crate::sol) svcs: SolSvcs,
}
impl SolarSystem {
    pub fn new(src: Src) -> Self {
        Self {
            src,
            fleets: SolFleets::new(),
            fits: SolFits::new(),
            items: SolItems::new(),
            sw_effects: StSet::new(),
            proj_effects: StSet::new(),
            proj_tracker: SolProjTracker::new(),
            svcs: SolSvcs::new(),
        }
    }
    // Item attributes
    pub fn get_item_attr(&mut self, item_id: &SolItemId, attr_id: &EAttrId) -> Result<SolAttrVal> {
        self.svcs.calc_get_item_attr_val(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item_id,
            attr_id,
        )
    }
    pub fn get_item_attrs(
        &mut self,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, SolAttrVal)>> {
        self.svcs
            .calc_iter_item_attr_vals(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item_id)
    }
    // Item modifications
    pub fn iter_item_modifiers(
        &mut self,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, Vec<SolModificationInfo>)>> {
        self.svcs
            .calc_iter_item_mods(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item_id)
    }
    // Item effects
    pub fn iter_item_effects<'a>(
        &'a self,
        item_id: &'a SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EEffectId, SolEffectInfo)> + 'a> {
        let item = self.items.get_item(item_id)?;
        let a_effect_ids = item.get_effect_datas()?.keys();
        let effect_infos = a_effect_ids.map(move |v| {
            let running = self.svcs.is_effect_running(item_id, v);
            let mode = item.get_effect_modes().get(v);
            (*v, SolEffectInfo::new(running, *mode))
        });
        Ok(effect_infos)
    }
    pub fn set_item_effect_mode(
        &mut self,
        item_id: &SolItemId,
        effect_id: &EEffectId,
        mode: SolEffectMode,
    ) -> Result<()> {
        self.items
            .get_item_mut(item_id)?
            .get_effect_modes_mut()
            .set(*effect_id, mode);
        let item = self.items.get_item(item_id).unwrap();
        self.svcs.process_effects(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item,
            item.get_state(),
        );
        Ok(())
    }
    pub fn set_item_effect_modes(
        &mut self,
        item_id: &SolItemId,
        modes: impl Iterator<Item = (EEffectId, SolEffectMode)>,
    ) -> Result<()> {
        let effect_modes = self.items.get_item_mut(item_id)?.get_effect_modes_mut();
        for (effect_id, effect_mode) in modes {
            effect_modes.set(effect_id, effect_mode)
        }
        let item = self.items.get_item(item_id).unwrap();
        self.svcs.process_effects(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item,
            item.get_state(),
        );
        Ok(())
    }
}
