use std::{collections::HashMap, sync::Arc};

type KeyVal = i32;

pub trait Key {
    fn get_key(&self) -> KeyVal;
}
impl Key for rc::ad::AItem {
    fn get_key(&self) -> KeyVal {
        self.id
    }
}
impl Key for rc::ad::AAttr {
    fn get_key(&self) -> KeyVal {
        self.id
    }
}
impl Key for rc::ad::AEffect {
    fn get_key(&self) -> KeyVal {
        self.id
    }
}
impl Key for rc::ad::AMuta {
    fn get_key(&self) -> KeyVal {
        self.id
    }
}
impl Key for rc::ad::ABuff {
    fn get_key(&self) -> KeyVal {
        self.id
    }
}

pub(crate) fn move_vec_to_map<T: Key>(vec: Vec<T>, map: &mut HashMap<KeyVal, Arc<T>>) {
    map.clear();
    map.shrink_to_fit();
    map.reserve(vec.len());
    vec.into_iter().for_each(|v| {
        map.insert(v.get_key(), Arc::new(v));
    });
}
