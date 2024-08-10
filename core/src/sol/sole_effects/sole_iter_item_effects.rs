use crate::{
    defs::{EEffectId, SolItemId},
    sol::{SolEffectInfo, SolarSystem},
};

impl SolarSystem {
    pub fn iter_item_effects<'a>(
        &'a self,
        item_id: &SolItemId,
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
}
