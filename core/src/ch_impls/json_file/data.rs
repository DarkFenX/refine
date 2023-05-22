use crate::ct;

#[derive(Debug, serde_tuple::Deserialize_tuple, serde_tuple::Serialize_tuple)]
pub(super) struct CacheData {
    pub(super) items: Vec<ct::Item>,
    pub(super) attrs: Vec<ct::Attr>,
    pub(super) mutas: Vec<ct::Muta>,
    pub(super) effects: Vec<ct::Effect>,
    pub(super) buffs: Vec<ct::Buff>,
    pub(super) fingerprint: String,
}
impl CacheData {
    pub(super) fn new(
        items: Vec<ct::Item>,
        attrs: Vec<ct::Attr>,
        mutas: Vec<ct::Muta>,
        effects: Vec<ct::Effect>,
        buffs: Vec<ct::Buff>,
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
