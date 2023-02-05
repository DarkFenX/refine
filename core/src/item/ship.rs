use std::sync::Arc;

use crate::{ct, ReeInt};

use super::ItemBase;

pub struct Ship {
    type_id: ReeInt,
    item: Option<Arc<ct::Item>>,
}
impl Ship {
    fn new(type_id: ReeInt) -> Ship {
        Ship { type_id, item: None }
    }
}
impl ItemBase for Ship {
    fn get_type_id(&self) -> ReeInt {
        self.type_id
    }
    fn get_item(&self) -> Option<&ct::Item> {
        self.item.as_deref()
    }
}
