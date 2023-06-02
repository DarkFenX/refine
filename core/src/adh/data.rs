use crate::adt;

/// Struct to store cacheable types and cache generation data.
pub struct AData {
    pub items: Vec<adt::AItem>,
    pub attrs: Vec<adt::AAttr>,
    pub mutas: Vec<adt::AMuta>,
    pub effects: Vec<adt::AEffect>,
    pub buffs: Vec<adt::ABuff>,
}
impl AData {
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
