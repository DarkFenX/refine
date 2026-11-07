use crate::{
    EffectMode,
    rd::{REffect, RItemBase, RItemEffectOptions, RState},
};

/// Rules deciding which effects run on an item.
pub(crate) struct RItemEffectsResolver;
impl RItemEffectsResolver {
    /// Whether the online effect runs on an item of the given type.
    pub(crate) fn resolve_online_effect_status(
        item_base: &RItemBase,
        online_effect_mode: EffectMode,
        item_state: RState,
    ) -> bool {
        if !item_base.has_online_effect {
            return false;
        }
        match online_effect_mode {
            // Since other effects from online category depend on the online effect in full
            // compliance mode, use simplified resolution for the online effect itself
            EffectMode::FullCompliance | EffectMode::StateCompliance => item_state >= RState::Online,
            // Shouldn't run anything in ghost state even with force run mode
            EffectMode::ForceRun => true,
            EffectMode::ForceStop => false,
        }
    }
    /// Whether a regular effect runs on an item of the given type.
    pub(crate) fn resolve_effect_status(
        item_base: &RItemBase,
        effect: &REffect,
        effect_mode: EffectMode,
        item_state: RState,
        effect_options: RItemEffectOptions,
        online_running: bool,
    ) -> bool {
        // Ghosted items should never affect anything regardless of effect mode, so check it first
        // wherever applicable
        match effect_mode {
            EffectMode::FullCompliance => {
                Self::resolve_effect_status_full(item_base, effect, item_state, effect_options, online_running)
            }
            EffectMode::StateCompliance => item_state >= effect.state,
            EffectMode::ForceRun => true,
            EffectMode::ForceStop => false,
        }
    }
    fn resolve_effect_status_full(
        item_base: &RItemBase,
        effect: &REffect,
        item_state: RState,
        effect_options: RItemEffectOptions,
        online_running: bool,
    ) -> bool {
        match effect.state {
            RState::Ghost => false,
            // All effects with disabled state are run if item is in disabled+ state, unless caller
            // requested extra requirement for them to be default
            RState::Disabled => {
                if effect.state > item_state {
                    return false;
                };
                match effect_options.contains(RItemEffectOptions::REQUIRE_DISABLED_DEFEFF) {
                    true => match item_base.defeff_rid {
                        Some(defeff_rid) => defeff_rid == effect.rid,
                        None => false,
                    },
                    false => true,
                }
            }
            // Offline effects require item in offline+ state, and no fitting usage chance attribute
            // (not to run booster side effects by default)
            RState::Offline => item_state >= effect.state && effect.chance_attr_rid.is_none(),
            // Online effects depend on 'online' effect, ignoring everything else
            RState::Online => online_running,
            // Only default active effect is run, and only if item is in active+ state, except for
            // the case when all active effects are requested to run
            RState::Active => {
                if effect.state > item_state {
                    return false;
                };
                match effect_options.contains(RItemEffectOptions::FORCE_ACTIVE_NONDEFEFF) {
                    true => true,
                    false => match item_base.defeff_rid {
                        Some(defeff_rid) => defeff_rid == effect.rid,
                        None => false,
                    },
                }
            }
            // No additional restrictions for overload effects except for item being overloaded
            RState::Overload => item_state >= effect.state,
        }
    }
}
