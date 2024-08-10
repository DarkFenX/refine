use crate::{
    defs::{EEffectId, SolItemId},
    sol::{SolEffectMode, SolView, SolarSystem},
};

impl SolarSystem {
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
}
