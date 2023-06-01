#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct CacheData {
    pub(super) items: Vec<rc::adt::Item>,
    pub(super) attrs: Vec<rc::adt::Attr>,
    pub(super) mutas: Vec<rc::adt::Muta>,
    pub(super) effects: Vec<rc::adt::Effect>,
    pub(super) buffs: Vec<rc::adt::Buff>,
    pub(super) fingerprint: String,
}
impl CacheData {
    pub(super) fn new(
        items: Vec<rc::adt::Item>,
        attrs: Vec<rc::adt::Attr>,
        mutas: Vec<rc::adt::Muta>,
        effects: Vec<rc::adt::Effect>,
        buffs: Vec<rc::adt::Buff>,
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
