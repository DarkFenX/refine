use crate::ct;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(super) struct Container<'a> {
    pub(super) items: Vec<&'a ct::Item>,
    pub(super) attrs: Vec<&'a ct::Attr>,
    pub(super) mutas: Vec<&'a ct::Muta>,
    pub(super) effects: Vec<&'a ct::Effect>,
    pub(super) buffs: Vec<&'a ct::Buff>,
    pub(super) fingerprint: &'a str,
}
impl Container<'_> {
    pub fn new<'a>(
        items: Vec<&'a ct::Item>,
        attrs: Vec<&'a ct::Attr>,
        mutas: Vec<&'a ct::Muta>,
        effects: Vec<&'a ct::Effect>,
        buffs: Vec<&'a ct::Buff>,
        fingerprint: &'a String,
    ) -> Container<'a> {
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
