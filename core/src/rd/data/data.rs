use std::{
    hash::{BuildHasher, Hash},
    sync::Arc,
};

use crate::{
    ad,
    rd::{RAttr, RBuff, REffect, RItem, RMuta},
    util::{GetId, Map, RMap},
};

#[derive(Clone)]
pub(crate) struct RData {
    pub(crate) items: RMap<ad::AItemId, Arc<RItem>>,
    pub(crate) attrs: RMap<ad::AAttrId, Arc<RAttr>>,
    pub(crate) effects: RMap<ad::AEffectId, Arc<REffect>>,
    pub(crate) buffs: RMap<ad::ABuffId, Arc<RBuff>>,
    pub(crate) mutas: RMap<ad::AItemId, Arc<RMuta>>,
}
impl From<ad::AData> for RData {
    fn from(a_data: ad::AData) -> Self {
        Self {
            items: move_to_arcmap(a_data.items.into_values().map(RItem::new)),
            attrs: move_to_arcmap(a_data.attrs.into_values().map(RAttr::new)),
            effects: move_to_arcmap(a_data.effects.into_values().map(REffect::new)),
            buffs: move_to_arcmap(a_data.buffs.into_values().map(RBuff::new)),
            mutas: move_to_arcmap(a_data.mutas.into_values().map(RMuta::new)),
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
