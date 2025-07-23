use std::{
    hash::{BuildHasher, Hash},
    sync::Arc,
};

use crate::{ad, util::Map};

pub(in crate::src) trait Key {
    type Item;
    fn get_key(&self) -> Self::Item;
}
impl Key for ad::AItemRt {
    type Item = ad::AItemId;
    fn get_key(&self) -> Self::Item {
        self.ai.id
    }
}
impl Key for ad::AAttr {
    type Item = ad::AAttrId;
    fn get_key(&self) -> Self::Item {
        self.id
    }
}
impl Key for ad::AEffectRt {
    type Item = ad::AEffectId;
    fn get_key(&self) -> Self::Item {
        self.ae.id
    }
}
impl Key for ad::AMuta {
    type Item = ad::AItemId;
    fn get_key(&self) -> Self::Item {
        self.id
    }
}
impl Key for ad::ABuff {
    type Item = ad::ABuffId;
    fn get_key(&self) -> Self::Item {
        self.id
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
