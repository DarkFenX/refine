// Buffs specify what they can affect via item lists. For efficiency of attribute calculation,
// information about item lists used by buffs stored on items belonging to those item lists.

use crate::{
    ad::{AData, AEffectBuffScope},
    util::RSet,
};

pub(in crate::ad::generator::flow::s8_conv_post) fn fill_buff_item_lists(a_data: &mut AData) {
    // Collect item lists which are used in buffs
    let mut proj_item_list_aids = RSet::new();
    let mut fleet_item_list_aids = RSet::new();
    for a_effect in a_data.effects.values() {
        if let Some(a_effect_buff) = &a_effect.buff {
            for a_buff_scope in a_effect_buff.iter_a_scopes() {
                match a_buff_scope {
                    AEffectBuffScope::Carrier => (),
                    AEffectBuffScope::Projected(item_list_aid) => {
                        proj_item_list_aids.insert(item_list_aid);
                    }
                    AEffectBuffScope::Fleet(item_list_aid) => {
                        fleet_item_list_aids.insert(item_list_aid);
                    }
                }
            }
        }
    }
    // Put data about buff-involved item lists onto items which belong to those lists
    for item_list_aid in proj_item_list_aids {
        if let Some(a_item_list) = a_data.item_lists.get(&item_list_aid) {
            for item_aid in a_item_list.item_ids.iter() {
                if let Some(a_item) = a_data.items.get_mut(item_aid) {
                    a_item.proj_buff_item_list_ids.push(item_list_aid);
                }
            }
        }
    }
    for item_list_aid in fleet_item_list_aids {
        if let Some(a_item_list) = a_data.item_lists.get(&item_list_aid) {
            for item_aid in a_item_list.item_ids.iter() {
                if let Some(a_item) = a_data.items.get_mut(item_aid) {
                    a_item.fleet_buff_item_list_ids.push(item_list_aid);
                }
            }
        }
    }
}
