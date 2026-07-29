use crate::{
    ad::AEffectId,
    rd::{REffectId, RItemEffectData},
    util::RMap,
};

pub(in crate::rd::data::item::base) fn has_online_effect(
    item_effects: &RMap<REffectId, RItemEffectData>,
    effect_aid_rid_map: &RMap<AEffectId, REffectId>,
) -> bool {
    has_effect(item_effects, effect_aid_rid_map, &AEffectId::ONLINE)
}

pub(in crate::rd::data::item::base) fn has_turret_effect(
    item_effects: &RMap<REffectId, RItemEffectData>,
    effect_aid_rid_map: &RMap<AEffectId, REffectId>,
) -> bool {
    has_effect(item_effects, effect_aid_rid_map, &AEffectId::TURRET_FITTED)
}

pub(in crate::rd::data::item::base) fn has_launcher_effect(
    item_effects: &RMap<REffectId, RItemEffectData>,
    effect_aid_rid_map: &RMap<AEffectId, REffectId>,
) -> bool {
    has_effect(item_effects, effect_aid_rid_map, &AEffectId::LAUNCHER_FITTED)
}

fn has_effect(
    item_effects: &RMap<REffectId, RItemEffectData>,
    effect_aid_rid_map: &RMap<AEffectId, REffectId>,
    effect_id: &AEffectId,
) -> bool {
    let Some(effect_rid) = effect_aid_rid_map.get(effect_id) else {
        return false;
    };
    item_effects.contains_key(effect_rid)
}
