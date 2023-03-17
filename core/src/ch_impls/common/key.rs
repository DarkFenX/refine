use std::{collections::HashMap, sync::Arc};

use crate::{
    ct::{Attr, Buff, Effect, Item, Muta},
    ReeInt,
};

pub(in crate::ch_impls) trait Key {
    fn get_key(&self) -> ReeInt;
}
impl Key for Item {
    fn get_key(&self) -> ReeInt {
        self.id
    }
}
impl Key for Attr {
    fn get_key(&self) -> ReeInt {
        self.id
    }
}
impl Key for Effect {
    fn get_key(&self) -> ReeInt {
        self.id
    }
}
impl Key for Muta {
    fn get_key(&self) -> ReeInt {
        self.id
    }
}
impl Key for Buff {
    fn get_key(&self) -> ReeInt {
        self.id
    }
}

pub(in crate::ch_impls) fn move_vec_to_map<T>(vec: Vec<T>, map: &mut HashMap<ReeInt, Arc<T>>)
where
    T: Key,
{
    map.clear();
    map.shrink_to_fit();
    map.reserve(vec.len());
    vec.into_iter().for_each(|v| {
        map.insert(v.get_key(), Arc::new(v));
    });
}
