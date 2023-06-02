use std::{collections::HashMap, sync::Arc};

pub(crate) trait Key {
    fn get_key(&self) -> rc::ReeInt;
}
impl Key for rc::adt::AItem {
    fn get_key(&self) -> rc::ReeInt {
        self.id
    }
}
impl Key for rc::adt::AAttr {
    fn get_key(&self) -> rc::ReeInt {
        self.id
    }
}
impl Key for rc::adt::AEffect {
    fn get_key(&self) -> rc::ReeInt {
        self.id
    }
}
impl Key for rc::adt::AMuta {
    fn get_key(&self) -> rc::ReeInt {
        self.id
    }
}
impl Key for rc::adt::ABuff {
    fn get_key(&self) -> rc::ReeInt {
        self.id
    }
}

pub(crate) fn move_vec_to_map<T: Key>(vec: Vec<T>, map: &mut HashMap<rc::ReeInt, Arc<T>>) {
    map.clear();
    map.shrink_to_fit();
    map.reserve(vec.len());
    vec.into_iter().for_each(|v| {
        map.insert(v.get_key(), Arc::new(v));
    });
}
