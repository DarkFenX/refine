use itertools::Itertools;

use crate::{
    ad::AEffectId,
    misc::EffectMode,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_effect_id_mode(
        &mut self,
        item_uid: UItemId,
        effect_aid: AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get_mut(item_uid);
        u_item.set_effect_mode(effect_aid, effect_mode, &self.u_data.src);
        u_item.update_reffs(reuse_eupdates, &self.u_data.src);
        self.effect_mode_update_postprocess(item_uid, reuse_eupdates);
    }
    pub(in crate::api) fn internal_set_effect_id_modes(
        &mut self,
        item_uid: UItemId,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get_mut(item_uid);
        u_item.set_effect_modes(effect_modes, &self.u_data.src);
        u_item.update_reffs(reuse_eupdates, &self.u_data.src);
        self.effect_mode_update_postprocess(item_uid, reuse_eupdates);
    }
    fn effect_mode_update_postprocess(&mut self, item_uid: UItemId, reuse_eupdates: &mut UEffectUpdates) {
        SolarSystem::util_process_effect_updates(&self.u_data, &mut self.svc, item_uid, reuse_eupdates);
        let u_item = self.u_data.items.get(item_uid);
        let charge_uid = u_item.get_charge_uid();
        // Autocharges
        if !reuse_eupdates.autocharges.is_empty()
            && let Some(autocharges) = u_item.get_autocharges()
        {
            let ac_activations = reuse_eupdates
                .autocharges
                .iter()
                .filter_map(|ac_act| {
                    autocharges
                        .get_ac_uid(&ac_act.effect_rid)
                        .map(|ac_uid| (ac_uid, ac_act.active))
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
            && let Some(charge_uid) = charge_uid
        {
            SolarSystem::util_process_charge_activation(
                &mut self.u_data,
                &mut self.svc,
                charge_uid,
                charge_activated,
                reuse_eupdates,
            );
        }
    }
}
