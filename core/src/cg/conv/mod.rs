mod attr;
mod buff;
mod item;
mod muta;

use super::{data::Support, Data};

// Convert data handler-provided entities into cacheable types.
pub(super) fn convert(data: &Data, supp: &Support, warns: &mut Vec<String>) {
    let items = item::conv_items(data, supp, warns);
    let attrs = attr::conv_attrs(data);
    let mutas = muta::conv_mutas(data);
    let buffs = buff::conv_buffs(data, warns);
}





// fn conv_effects(data: &Data) -> Vec<ct::Effect> {
//     data.effects
//         .iter()
//         .map(|v| ct::Effect::new(
//             v.id,
//             State::Active,
//             TgtMode::None,
//             v.is_assistance,
//             v.is_offensive,
//             Some(True),
//             Some(True),
//             v.discharge_attr_id,
//             v.duration_attr_id,
//             v.range_attr_id,
//             v.falloff_attr_id,
//             v.tracking_attr_id,
//             v.usage_chance_attr_id,
//             v.resist_attr_id,
//             Vec::new(),
//             Vec::new(),
//         ))
//         .collect()
// }




