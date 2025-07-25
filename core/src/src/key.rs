use std::{
    hash::{BuildHasher, Hash},
    sync::Arc,
};

use crate::{ad, rd, util::Map};

pub(in crate::src) trait Key {
    type Item;
    fn get_key(&self) -> Self::Item;
}
impl Key for rd::RItem {
    type Item = ad::AItemId;
    fn get_key(&self) -> Self::Item {
        self.get_id()
    }
}
impl Key for rd::RAttr {
    type Item = ad::AAttrId;
    fn get_key(&self) -> Self::Item {
        self.get_id()
    }
}
impl Key for rd::REffect {
    type Item = ad::AEffectId;
    fn get_key(&self) -> Self::Item {
        self.get_id()
    }
}
impl Key for rd::RMuta {
    type Item = ad::AItemId;
    fn get_key(&self) -> Self::Item {
        self.get_id()
    }
}
impl Key for rd::RBuff {
    type Item = ad::ABuffId;
    fn get_key(&self) -> Self::Item {
        self.get_id()
    }
}

pub(in crate::src) fn map_to_arcmap<K, V, H>(map: impl ExactSizeIterator<Item = V>) -> Map<K, Arc<V>, H>
where
    K: Eq + PartialEq + Hash,
    V: Key<Item = K>,
    H: BuildHasher + Default,
{
    map.map(|v| (v.get_key(), Arc::new(v))).collect()
}
