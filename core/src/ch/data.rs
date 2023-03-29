use std::collections::HashMap;

use crate::{ct, ReeFloat};

/// Struct to store cacheable types and cache generation data.
pub struct Data {
    pub items: Vec<ct::Item>,
    pub attrs: Vec<ct::Attr>,
    pub mutas: Vec<ct::Muta>,
    pub effects: Vec<ct::Effect>,
    pub buffs: Vec<ct::Buff>,
    pub cg_warns: Vec<String>,
    pub cg_cleanup: HashMap<String, ReeFloat>,
}
impl Data {
    pub(crate) fn new() -> Self {
        Self {
            items: Vec::new(),
            attrs: Vec::new(),
            mutas: Vec::new(),
            effects: Vec::new(),
            buffs: Vec::new(),
            cg_warns: Vec::new(),
            cg_cleanup: HashMap::new(),
        }
    }
}
