use std::{
    hash::{BuildHasher, Hash},
    sync::Arc,
};

use slab::Slab;

use crate::{
    ad::{AAbilId, AAttrId, ABuffId, AData, AItemId},
    rd::{RAbil, RAttr, RBuff, REffect, RItem, RMuta, RcAbil, RcAttr, RcBuff, RcEffect, RcItem, RcMuta},
    util::{GetId, Map, RMap},
};

#[derive(Clone)]
pub(crate) struct RData {
    pub(crate) items: RMap<AItemId, RcItem>,
    pub(crate) attrs: RMap<AAttrId, RcAttr>,
    pub(crate) effects: Slab<RcEffect>,
    pub(crate) buffs: RMap<ABuffId, RcBuff>,
    pub(crate) mutas: RMap<AItemId, RcMuta>,
    pub(crate) abils: RMap<AAbilId, RcAbil>,
}
impl From<AData> for RData {
    fn from(a_data: AData) -> Self {
        let mut items = move_to_arcmap(a_data.items.into_values().map(RItem::new));
        let attrs = move_to_arcmap(a_data.attrs.into_values().map(RAttr::new));
        let buffs = move_to_arcmap(a_data.buffs.into_values().map(RBuff::new));
        let mutas = move_to_arcmap(a_data.mutas.into_values().map(RMuta::new));
        let mut abils = move_to_arcmap(a_data.abils.into_values().map(RAbil::new));
        // Put effects into slab
        let mut effect_id_key_map = RMap::with_capacity(a_data.effects.len());
        let mut effects = Slab::with_capacity(a_data.effects.len());
        for (a_effect_id, a_effect) in a_data.effects.into_iter() {
            let entry = effects.vacant_entry();
            let effect_key = entry.key();
            let r_effect = REffect::new(effect_key, a_effect);
            entry.insert(Arc::new(r_effect));
            effect_id_key_map.insert(a_effect_id, effect_key);
        }
        // Refresh data which relies on effects' slab keys
        for r_item in items.values_mut() {
            Arc::get_mut(r_item).unwrap().fill_key_dependents(&effect_id_key_map);
        }
        for (_, r_effect) in effects.iter_mut() {
            Arc::get_mut(r_effect).unwrap().fill_key_dependents(&effect_id_key_map);
        }
        for r_abil in abils.values_mut() {
            Arc::get_mut(r_abil).unwrap().fill_key_dependents(&effect_id_key_map);
        }
        Self {
            items,
            attrs,
            effects,
            buffs,
            mutas,
            abils,
        }
    }
}

fn move_to_arcmap<K, V, H>(entities: impl ExactSizeIterator<Item = V>) -> Map<K, Arc<V>, H>
where
    K: Eq + PartialEq + Hash,
    V: GetId<K>,
    H: BuildHasher + Default,
{
    entities.map(|v| (v.get_id(), Arc::new(v))).collect()
}
