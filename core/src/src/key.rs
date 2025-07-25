use std::{
    hash::{BuildHasher, Hash},
    sync::Arc,
};

use crate::{
    ad, rd,
    util::{GetId, Map},
};

impl GetId<ad::AItemId> for rd::RItem {
    fn get_id(&self) -> ad::AItemId {
        self.get_id()
    }
}
impl GetId<ad::AAttrId> for rd::RAttr {
    fn get_id(&self) -> ad::AAttrId {
        self.get_id()
    }
}
impl GetId<ad::AEffectId> for rd::REffect {
    fn get_id(&self) -> ad::AEffectId {
        self.get_id()
    }
}
impl GetId<ad::AItemId> for rd::RMuta {
    fn get_id(&self) -> ad::AItemId {
        self.get_id()
    }
}
impl GetId<ad::ABuffId> for rd::RBuff {
    fn get_id(&self) -> ad::ABuffId {
        self.get_id()
    }
}

pub(in crate::src) fn map_to_arcmap<K, V, H>(map: impl ExactSizeIterator<Item = V>) -> Map<K, Arc<V>, H>
where
    K: Eq + PartialEq + Hash,
    V: GetId<K>,
    H: BuildHasher + Default,
{
    map.map(|v| (v.get_id(), Arc::new(v))).collect()
}
