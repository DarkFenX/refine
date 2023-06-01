use crate::adt;

/// Struct to store cacheable types and cache generation data.
pub struct Data {
    pub items: Vec<adt::Item>,
    pub attrs: Vec<adt::Attr>,
    pub mutas: Vec<adt::Muta>,
    pub effects: Vec<adt::Effect>,
    pub buffs: Vec<adt::Buff>,
}
impl Data {
    pub(crate) fn new() -> Self {
        Self {
            items: Vec::new(),
            attrs: Vec::new(),
            mutas: Vec::new(),
            effects: Vec::new(),
            buffs: Vec::new(),
        }
    }
}
