use crate::ct;

/// Struct for cacheable types, used as part of cache handler interface.
pub struct Container {
    pub items: Vec<ct::Item>,
    pub attrs: Vec<ct::Attr>,
    pub mutas: Vec<ct::Muta>,
    pub effects: Vec<ct::Effect>,
    pub buffs: Vec<ct::Buff>,
}
impl Container {
    pub fn new() -> Container {
        Container {
            items: Vec::new(),
            attrs: Vec::new(),
            mutas: Vec::new(),
            effects: Vec::new(),
            buffs: Vec::new(),
        }
    }
}
