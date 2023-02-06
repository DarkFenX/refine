use std::sync::{Arc, Weak};

use crate::{ct, Fit, ReeInt};

use super::{FitChild, IntItemBase, ItemBase};

pub struct Ship {
    type_id: ReeInt,
    item: Option<Arc<ct::Item>>,
    fit: Option<Weak<Fit>>,
}
impl Ship {
    pub fn new(type_id: ReeInt) -> Ship {
        Ship {
            type_id,
            item: None,
            fit: None,
        }
    }
}
impl ItemBase for Ship {
    fn get_type_id(&self) -> ReeInt {
        self.type_id
    }
}
impl IntItemBase for Ship {
    fn get_item(&self) -> Option<&ct::Item> {
        self.item.as_deref()
    }
    fn load_item(&mut self) {
        match self.get_fit() {
            None => self.item = None,
            Some(f) => match f.get_sol_sys() {
                None => self.item = None,
                Some(ss) => match ss.src.cache_handler.get_item(self.type_id) {
                    None => self.item = None,
                    Some(i) => self.item = Some(i),
                },
            },
        }
    }
}
impl FitChild for Ship {
    fn get_fit(&self) -> Option<Arc<Fit>> {
        match &self.fit {
            None => None,
            Some(f) => match f.upgrade() {
                None => None,
                Some(f) => Some(f),
            },
        }
    }
    fn set_fit(&mut self, fit: Option<Arc<Fit>>) {
        self.fit = fit.map(|v| Arc::downgrade(&v))
    }
}
