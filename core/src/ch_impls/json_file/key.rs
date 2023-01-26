use crate::{
    ct::{Attr, Buff, Effect, Item, Muta},
    ReeInt,
};

pub(super) trait Key {
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
