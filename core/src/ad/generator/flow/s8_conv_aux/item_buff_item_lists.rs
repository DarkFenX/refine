// Buffs specify what they can affect via item lists. For efficiency of attribute calculation,
// information about item lists used by buffs stored on items belonging to those item lists.

use crate::{
    ad::{ADataGenerator, AEffectBuffScope, AItemCatId},
    util::RSet,
};

const MAX_ALLOWED_LENGTH: usize = 5;

impl ADataGenerator {
    pub(super) fn fill_buff_item_lists(&mut self) {
        // Collect item lists which are used in buffs
        let mut proj_item_list_aids = RSet::new();
        let mut fleet_item_list_aids = RSet::new();
        for a_effect in self.a_data.effects.data.values() {
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
        let mut proj_max_len = 0;
        let mut fleet_max_len = 0;
        // Put data about buff-involved item lists onto items which belong to those lists
        for item_list_aid in proj_item_list_aids {
            if let Some(a_item_list) = self.a_data.item_lists.data.get(&item_list_aid) {
                for item_aid in a_item_list.item_ids.iter() {
                    if let Some(a_item) = self.a_data.items.data.get_mut(item_aid) {
                        a_item.proj_buff_item_list_ids.insert(item_list_aid);
                        if a_item.cat_id != AItemCatId::SHIP {
                            proj_max_len = proj_max_len.max(a_item.proj_buff_item_list_ids.len());
                        }
                    }
                }
            }
        }
        for item_list_aid in fleet_item_list_aids {
            if let Some(a_item_list) = self.a_data.item_lists.data.get(&item_list_aid) {
                for item_aid in a_item_list.item_ids.iter() {
                    if let Some(a_item) = self.a_data.items.data.get_mut(item_aid) {
                        a_item.fleet_buff_item_list_ids.insert(item_list_aid);
                        if a_item.cat_id != AItemCatId::SHIP {
                            fleet_max_len = fleet_max_len.max(a_item.fleet_buff_item_list_ids.len());
                        }
                    }
                }
            }
        }
        // Calculator module works with assumption that count of item lists for non-ships won't
        // exceed certain count. When that limit is broken, it might make sense to:
        // - switch from using vector to hash set (mostly it concerns rd module items);
        // - possibly change how calculator processes type list checks when working on buffs.
        // Ships are excluded mostly because performance when adding/removing ships does not matter
        // much. 5 is an arbitrary threshold, need to reassess different approaches once it is
        // reached.
        if proj_max_len >= MAX_ALLOWED_LENGTH {
            tracing::warn!(
                "max count of item list IDs involved in projected buffs is {}",
                proj_max_len
            );
        }
        if fleet_max_len >= MAX_ALLOWED_LENGTH {
            tracing::warn!(
                "max count of item list IDs involved in fleet buffs is {}",
                fleet_max_len
            );
        }
    }
}
