use crate::ct;

/// Struct to store cacheable types
pub struct CHData {
    pub items: Vec<ct::Item>,
    pub attrs: Vec<ct::Attr>,
    pub mutas: Vec<ct::Muta>,
    pub effects: Vec<ct::Effect>,
    pub buffs: Vec<ct::Buff>,
}
impl CHData {
    pub(crate) fn new() -> CHData {
        CHData {
            items: Vec::new(),
            attrs: Vec::new(),
            mutas: Vec::new(),
            effects: Vec::new(),
            buffs: Vec::new(),
        }
    }
}
