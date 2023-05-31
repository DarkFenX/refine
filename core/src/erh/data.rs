use crate::ert;

/// Struct to store cacheable types and cache generation data.
pub struct Data {
    pub items: Vec<ert::Item>,
    pub attrs: Vec<ert::Attr>,
    pub mutas: Vec<ert::Muta>,
    pub effects: Vec<ert::Effect>,
    pub buffs: Vec<ert::Buff>,
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
