use crate::ert;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct CacheData {
    pub(super) items: Vec<ert::Item>,
    pub(super) attrs: Vec<ert::Attr>,
    pub(super) mutas: Vec<ert::Muta>,
    pub(super) effects: Vec<ert::Effect>,
    pub(super) buffs: Vec<ert::Buff>,
    pub(super) fingerprint: String,
}
impl CacheData {
    pub(super) fn new(
        items: Vec<ert::Item>,
        attrs: Vec<ert::Attr>,
        mutas: Vec<ert::Muta>,
        effects: Vec<ert::Effect>,
        buffs: Vec<ert::Buff>,
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
