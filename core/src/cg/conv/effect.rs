use crate::{
    consts::{effcats, State, TgtMode},
    ct,
    defines::ReeInt,
    dh,
    util::Named,
};

use super::Data;

pub(super) fn conv_effects(data: &Data, warns: &mut Vec<String>) -> Vec<ct::Effect> {
    let mut effects = Vec::new();
    for effect_data in data.effects.iter() {
        let (state, tgt_mode) = match effect_data.category_id {
            effcats::PASSIVE => (State::Offline, TgtMode::None),
            effcats::ACTIVE => (State::Active, TgtMode::None),
            effcats::TARGET => (State::Active, TgtMode::Point),
            effcats::ONLINE => (State::Online, TgtMode::None),
            effcats::OVERLOAD => (State::Overload, TgtMode::None),
            effcats::SYSTEM => (State::Offline, TgtMode::None),
            _ => {
                let msg = format!(
                    "{} {} uses unknown effect category {}",
                    dh::Effect::get_name(),
                    effect_data.id,
                    effect_data.category_id
                );
                log::warn!("{}", &msg);
                warns.push(msg);
                continue;
            }
        };
        let mut effect = ct::Effect::new(
            effect_data.id,
            state,
            tgt_mode,
            effect_data.is_assistance,
            effect_data.is_offensive,
            None,
            None,
            effect_data.discharge_attr_id,
            effect_data.duration_attr_id,
            effect_data.range_attr_id,
            effect_data.falloff_attr_id,
            effect_data.tracking_attr_id,
            effect_data.usage_chance_attr_id,
            effect_data.resist_attr_id,
            Vec::new(),
            Vec::new(),
        );
        effects.push(effect);
    }
    effects
}
