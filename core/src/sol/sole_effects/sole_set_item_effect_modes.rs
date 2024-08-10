use crate::{
    defs::{EEffectId, SolItemId},
    sol::{SolEffectMode, SolView, SolarSystem},
};

impl SolarSystem {
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
