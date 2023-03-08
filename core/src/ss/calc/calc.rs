use std::{collections::HashMap, sync::Arc};

use crate::{consts::TgtMode, ct, ss::item::Item, ReeFloat, ReeId, ReeInt, Src};

use super::affection_reg::AffectionRegister;

pub(in crate::ss) struct CalcSvc {
    attrs: HashMap<ReeId, HashMap<ReeInt, ReeFloat>>,
    affection: AffectionRegister,
}
impl CalcSvc {
    pub(in crate::ss) fn new() -> CalcSvc {
        CalcSvc {
            attrs: HashMap::new(),
            affection: AffectionRegister::new(),
        }
    }
    // Query methods
    pub(in crate::ss) fn get_attr_val(&mut self, item: &Item, attr_id: &ReeInt, src: &Src) -> Option<ReeFloat> {
        let item_id = item.get_id();
        match self.attrs.get(&item_id) {
            Some(attrs) => match attrs.get(attr_id) {
                Some(v) => return Some(*v),
                _ => (),
            },
            _ => (),
        };
        let val = match self.calc_attr(item, attr_id, src) {
            Some(v) => v,
            _ => return None,
        };
        match self.attrs.get_mut(&item_id) {
            Some(attrs) => {
                attrs.insert(*attr_id, val);
                ();
            }
            _ => (),
        };
        Some(val)
    }
    // Maintenance methods
    pub(in crate::ss) fn item_loaded(&mut self, item: &Item) {
        self.attrs.insert(item.get_id(), HashMap::new());
        self.affection.reg_afee(item);
    }
    pub(in crate::ss) fn item_unloaded(&mut self, item: &Item) {
        self.affection.unreg_afee(item);
        self.attrs.remove(&item.get_id());
    }
    pub(in crate::ss) fn effects_started(&mut self, item: &Item, effects: &Vec<Arc<ct::Effect>>) {
        for effect in effects.iter().filter(|e| matches!(&e.tgt_mode, TgtMode::None)) {
            self.affection.reg_local_effect(item, effect);
        }
    }
    pub(in crate::ss) fn effects_stopped(&mut self, item: &Item, effects: &Vec<Arc<ct::Effect>>) {
        for effect in effects.iter().filter(|e| matches!(&e.tgt_mode, TgtMode::None)) {
            self.affection.unreg_local_effect(item, effect);
        }
    }
    // Private methods
    fn calc_attr(&mut self, item: &Item, attr_id: &ReeInt, src: &Src) -> Option<ReeFloat> {
        let attr = match src.cache_handler.get_attr(attr_id) {
            Some(attr) => attr,
            None => return None,
        };
        // Get base value; use on-iteme original attributes, or, if not specified, default attribute value.
        // If both can't be fetched, consider it a failure
        let val = match item.get_orig_attrs() {
            Some(orig_attrs) => match orig_attrs.get(attr_id) {
                Some(orig_val) => *orig_val,
                None => match attr.def_val {
                    Some(def_val) => def_val,
                    None => return None,
                },
            },
            None => return None,
        };
        // let stacked = Vec::new();
        // let stacked_penalized = Vec::new();
        // let aggregate_min = Vec::new();
        // let aggregate_max = Vec::new();
        Some(0.0)
    }
    //fn get_modifications
}
