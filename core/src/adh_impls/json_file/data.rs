use crate::adt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct CacheData {
    pub(super) items: Vec<adt::Item>,
    pub(super) attrs: Vec<adt::Attr>,
    pub(super) mutas: Vec<adt::Muta>,
    pub(super) effects: Vec<adt::Effect>,
    pub(super) buffs: Vec<adt::Buff>,
    pub(super) fingerprint: String,
}
impl CacheData {
    pub(super) fn new(
        items: Vec<adt::Item>,
        attrs: Vec<adt::Attr>,
        mutas: Vec<adt::Muta>,
        effects: Vec<adt::Effect>,
        buffs: Vec<adt::Buff>,
        fingerprint: String,
    ) -> Self {
        Self {
            items,
            attrs,
            mutas,
            effects,
            buffs,
            fingerprint,
        }
    }
}
