use itertools::Itertools;

use crate::{
    EffectMode,
    ad::AEffectId,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

impl CoreInner {
    pub(in crate::api) fn internal_set_effect_id_mode(
        &mut self,
        item_uid: UItemId,
        effect_aid: AEffectId,
        effect_mode: EffectMode,
    ) {
        let u_item = self.u_data.items.get_mut(item_uid);
        let old_effect_mode = u_item.set_effect_mode(effect_aid, effect_mode, &self.u_data.r_data);
        if old_effect_mode == effect_mode {
            return;
        }
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::ItemEffectMode(item_uid, effect_aid, old_effect_mode));
        }
        u_item.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        self.effect_mode_update_postprocess(item_uid);
    }
    pub(in crate::api::item) fn internal_set_effect_id_modes(
        &mut self,
        item_uid: UItemId,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
    ) {
        let u_item = self.u_data.items.get_mut(item_uid);
        let changed = match self.undo.is_recording() {
            true => {
                let mut changed = false;
                for (effect_aid, effect_mode) in effect_modes {
                    let old_effect_mode = u_item.set_effect_mode(effect_aid, effect_mode, &self.u_data.r_data);
                    if old_effect_mode != effect_mode {
                        changed = true;
                        self.undo
                            .record(UndoStep::ItemEffectMode(item_uid, effect_aid, old_effect_mode));
                    }
                }
                changed
            }
            false => u_item.set_effect_modes(effect_modes, &self.u_data.r_data),
        };
        if !changed {
            return;
        }
        u_item.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        self.effect_mode_update_postprocess(item_uid);
    }
    fn effect_mode_update_postprocess(&mut self, item_uid: UItemId) {
        CoreInner::util_process_effect_updates(&self.u_data, &mut self.svc, item_uid, &self.reuse.eupdates);
        let u_item = self.u_data.items.get(item_uid);
        let charge_uid = u_item.get_charge_uid();
        // Autocharges - use secondary effect updates container for their effects
        if !self.reuse.eupdates.autocharges.is_empty()
            && let Some(autocharges) = u_item.get_autocharges()
        {
            let ac_activations = self
                .reuse
                .eupdates
                .autocharges
                .iter()
                .filter_map(|ac_act| {
                    autocharges
                        .get_ac_uid(&ac_act.effect_rid)
                        .map(|ac_uid| (ac_uid, ac_act.active))
                })
                .collect_vec();
            CoreInner::util_process_autocharge_activations(
                &mut self.u_data,
                &mut self.svc,
                ac_activations,
                &mut self.reuse.eupdates_secondary,
            );
        }
        // Charge - use secondary effect updates container for their effects
        if let Some(charge_activated) = self.reuse.eupdates.charge
            && let Some(charge_uid) = charge_uid
        {
            CoreInner::util_process_charge_activation(
                &mut self.u_data,
                &mut self.svc,
                charge_uid,
                charge_activated,
                &mut self.reuse.eupdates_secondary,
            );
        }
    }
}
