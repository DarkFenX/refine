use std::collections::HashMap;

use crate::{ct, ReeFloat};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(super) struct CacheData {
    pub(super) items: Vec<ct::Item>,
    pub(super) attrs: Vec<ct::Attr>,
    pub(super) mutas: Vec<ct::Muta>,
    pub(super) effects: Vec<ct::Effect>,
    pub(super) buffs: Vec<ct::Buff>,
    pub(super) fingerprint: String,
    pub(super) cg_warns: Vec<String>,
    pub(super) cg_cleanup: HashMap<String, ReeFloat>,
}
impl CacheData {
    pub(super) fn new(
        items: Vec<ct::Item>,
        attrs: Vec<ct::Attr>,
        mutas: Vec<ct::Muta>,
        effects: Vec<ct::Effect>,
        buffs: Vec<ct::Buff>,
        fingerprint: String,
        cg_warns: Vec<String>,
        cg_cleanup: HashMap<String, ReeFloat>,
    ) -> Self {
        Self {
            items,
            attrs,
            mutas,
            effects,
            buffs,
            fingerprint,
            cg_warns,
            cg_cleanup,
        }
    }
}
