use std::{hash::Hash, sync::Arc};

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

pub(crate) fn move_map_to_arcmap<T, U>(map: rc::util::HMap<U, T>, arcmap: &mut rc::util::HMap<U, Arc<T>>)
where
    T: Key<Item = U>,
    U: Eq + PartialEq + Hash,
{
    arcmap.clear();
    arcmap.shrink_to_fit();
    arcmap.reserve(map.len());
    map.into_values().for_each(|v| {
        arcmap.insert(v.get_key(), Arc::new(v));
    });
}
