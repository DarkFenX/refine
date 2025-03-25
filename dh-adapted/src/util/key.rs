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

pub(crate) fn move_vec_to_map<T, U>(vec: Vec<T>, map: &mut rc::util::StMap<U, Arc<T>>)
where
    T: Key<Item = U>,
    U: Eq + PartialEq + Hash,
{
    map.clear();
    map.shrink_to_fit();
    map.reserve(vec.len());
    vec.into_iter().for_each(|v| {
        map.insert(v.get_key(), Arc::new(v));
    });
}
