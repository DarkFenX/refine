use itertools::Itertools;

use crate::{
    ad::AEffectId,
    misc::EffectMode,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_effect_id_mode(
        &mut self,
        item_key: UItemKey,
        effect_id: AEffectId,
        effect_mode: EffectMode,
    ) where
        Self: Sized,
    {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.u_data.items.get_mut(item_key).set_effect_mode(
            effect_id,
            effect_mode,
            &mut reuse_eupdates,
            &self.u_data.src,
        );
        self.effect_mode_update_postprocess(item_key, &mut reuse_eupdates);
    }
    pub(in crate::sol::api) fn internal_set_effect_id_modes(
        &mut self,
        item_key: UItemKey,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
    ) where
        Self: Sized,
    {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.u_data
            .items
            .get_mut(item_key)
            .set_effect_modes(effect_modes, &mut reuse_eupdates, &self.u_data.src);
        self.effect_mode_update_postprocess(item_key, &mut reuse_eupdates);
    }
    fn effect_mode_update_postprocess(&mut self, item_key: UItemKey, reuse_eupdates: &mut UEffectUpdates) {
        let u_item = self.u_data.items.get(item_key);
        SolarSystem::util_process_effect_updates(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        if !reuse_eupdates.autocharges.is_empty()
            && let Some(autocharges) = u_item.get_autocharges()
        {
            let ac_activations = reuse_eupdates
                .autocharges
                .iter()
                .filter_map(|ac_act| {
                    autocharges
                        .get_ac_key(&ac_act.effect_key)
                        .map(|ac_key| (ac_key, ac_act.active))
                })
                .collect_vec();
            SolarSystem::util_process_autocharge_activations(
                &mut self.u_data,
                &mut self.svc,
                ac_activations,
                reuse_eupdates,
            );
        }
    }
}
