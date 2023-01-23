use crate::ct;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(super) struct Container {
    pub(super) items: Vec<ct::Item>,
    pub(super) attrs: Vec<ct::Attr>,
    pub(super) mutas: Vec<ct::Muta>,
    pub(super) effects: Vec<ct::Effect>,
    pub(super) buffs: Vec<ct::Buff>,
    pub(super) fingerprint: String,
}
impl Container {
    pub fn new(
        items: Vec<ct::Item>,
        attrs: Vec<ct::Attr>,
        mutas: Vec<ct::Muta>,
        effects: Vec<ct::Effect>,
        buffs: Vec<ct::Buff>,
        fingerprint: String,
    ) -> Container {
        Container {
            items,
            attrs,
            mutas,
            effects,
            buffs,
            fingerprint,
        }
    }
}
