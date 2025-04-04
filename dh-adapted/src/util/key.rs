use std::{
    hash::{BuildHasher, Hash},
    sync::Arc,
};

pub trait Key {
    type Item;
    fn get_key(&self) -> Self::Item;
}
impl Key for rc::ad::AItem {
    type Item = rc::ad::AItemId;
    fn get_key(&self) -> Self::Item {
        self.id
    }
}
impl Key for rc::ad::AAttr {
    type Item = rc::ad::AAttrId;
    fn get_key(&self) -> Self::Item {
        self.id
    }
}
impl Key for rc::ad::AEffect {
    type Item = rc::ad::AEffectId;
    fn get_key(&self) -> Self::Item {
        self.id
    }
}
impl Key for rc::ad::AMuta {
    type Item = rc::ad::AItemId;
    fn get_key(&self) -> Self::Item {
        self.id
    }
}
impl Key for rc::ad::ABuff {
    type Item = rc::ad::ABuffId;
    fn get_key(&self) -> Self::Item {
        self.id
    }
}

pub(crate) fn move_map_to_arcmap<K, V, H>(map: rc::util::Map<V, K, H>, arcmap: &mut rc::util::Map<V, Arc<K>, H>)
where
    K: Key<Item = V>,
    V: Eq + PartialEq + Hash,
    H: BuildHasher + Default,
{
    arcmap.clear();
    arcmap.shrink_to_fit();
    arcmap.reserve(map.len());
    map.into_values().for_each(|v| {
        arcmap.insert(v.get_key(), Arc::new(v));
    });
}
