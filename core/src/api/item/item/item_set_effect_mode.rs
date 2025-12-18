use itertools::Itertools;

use crate::{
    ad::AEffectId,
    misc::EffectMode,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_effect_id_mode(
        &mut self,
        item_key: UItemKey,
        effect_id: AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get_mut(item_key);
        u_item.set_effect_mode(effect_id, effect_mode, &self.u_data.src);
        u_item.update_reffs(reuse_eupdates, &self.u_data.src);
        self.effect_mode_update_postprocess(item_key, reuse_eupdates);
    }
    pub(in crate::api) fn internal_set_effect_id_modes(
        &mut self,
        item_key: UItemKey,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get_mut(item_key);
        u_item.set_effect_modes(effect_modes, &self.u_data.src);
        u_item.update_reffs(reuse_eupdates, &self.u_data.src);
        self.effect_mode_update_postprocess(item_key, reuse_eupdates);
    }
    fn effect_mode_update_postprocess(&mut self, item_key: UItemKey, reuse_eupdates: &mut UEffectUpdates) {
        SolarSystem::util_process_effect_updates(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
        let u_item = self.u_data.items.get(item_key);
        let charge_key = u_item.get_charge_key();
        // Autocharges
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
        // Charge
        if let Some(charge_activated) = reuse_eupdates.charge
            && let Some(charge_key) = charge_key
        {
            SolarSystem::util_process_charge_activation(
                &mut self.u_data,
                &mut self.svc,
                charge_key,
                charge_activated,
                reuse_eupdates,
            );
        }
    }
}
