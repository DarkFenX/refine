use crate::{
    EffectMode,
    rd::{REffectConsts, REffectId, RItemBase, RItemEffectOptions, RItemEffectsResolver, RState, RcEffect},
    util::PDenseSlab,
};

/// Effects an item type runs without effect mode overrides, ordered by the item state they start
/// running at, with the end of each state's group.
#[derive(Eq, PartialEq, Hash)]
pub(crate) struct RItemEffectsPregenEntry {
    effect_rids: Box<[REffectId]>,
    state_ends: [u16; RState::COUNT],
}
impl RItemEffectsPregenEntry {
    /// Effects which start running above `low`, up to `high`.
    pub(crate) fn get_above_up_to(&self, low: RState, high: RState) -> &[REffectId] {
        &self.effect_rids[self.state_ends[low as usize] as usize..self.state_ends[high as usize] as usize]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemEffectsPregenEntry {
    pub(super) fn new(
        reuse_state_effects: &mut Vec<(RState, REffectId)>,
        item_base: &RItemBase,
        effects: &PDenseSlab<REffectId, RcEffect>,
        effect_consts: &REffectConsts,
        effect_options: RItemEffectOptions,
    ) -> Self {
        reuse_state_effects.clear();
        for &effect_rid in item_base.effect_rids.iter() {
            let effect = effects.get(effect_rid).unwrap();
            // Find first state which runs the effect with default effect mode
            let start_state = RState::above_up_to(RState::Ghost, RState::Overload)
                .iter()
                .find(|&&state| {
                    let online_running =
                        RItemEffectsResolver::resolve_online_effect_status(item_base, EffectMode::default(), state);
                    match Some(effect_rid) == effect_consts.online {
                        true => online_running,
                        false => RItemEffectsResolver::resolve_effect_status(
                            item_base,
                            effect,
                            EffectMode::default(),
                            state,
                            effect_options,
                            online_running,
                        ),
                    }
                });
            // Store effect + state in a temporary container
            if let Some(&start_state) = start_state {
                reuse_state_effects.push((start_state, effect_rid));
            }
        }
        // Effects were filled using on-item order; do not break it here with unstable sort, to make
        // order of pregenerated effect list the same as generated on the fly (it ultimately
        // shouldn't matter for the lib services, but it doesn't cost much)
        reuse_state_effects.sort_by_key(|(state, _)| *state);
        // Get indices of state ends
        let mut state_ends = [0; RState::COUNT];
        for (state, _) in reuse_state_effects.iter() {
            state_ends[*state as usize] += 1;
        }
        for i in 1..RState::COUNT {
            state_ends[i] += state_ends[i - 1];
        }
        Self {
            effect_rids: reuse_state_effects.iter().map(|(_, effect_rid)| *effect_rid).collect(),
            state_ends,
        }
    }
}
