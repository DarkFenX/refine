use std::sync::Arc;

use crate::{
    ac, ad, ed, rd,
    src::{SrcInitError, key::map_to_arcmap, prepare::prepare_adapted_data},
    util::RMap,
};

/// Data source.
///
/// Data source is a top-level entity which manages EVE and adapted data handlers to do necessary
/// preparations and expose adapted data to solar system and its services.
#[derive(Clone)]
pub struct Src {
    storage_items: RMap<ad::AItemId, Arc<rd::RItem>>,
    storage_attrs: RMap<ad::AAttrId, Arc<rd::RAttr>>,
    storage_effects: RMap<ad::AEffectId, Arc<rd::REffect>>,
    storage_buffs: RMap<ad::ABuffId, Arc<rd::RBuff>>,
    storage_mutas: RMap<ad::AItemId, Arc<rd::RMuta>>,
    online_effect: Option<Arc<rd::REffect>>,
}
impl Src {
    #[tracing::instrument(name = "src-new", level = "trace", skip_all)]
    pub fn new(
        ed_handler: Box<dyn ed::EveDataHandler>,
        ad_cacher: Option<Box<dyn ad::AdaptedDataCacher>>,
    ) -> Result<Self, SrcInitError> {
        let a_data = prepare_adapted_data(ed_handler, ad_cacher)?;
        let storage_effects = map_to_arcmap(a_data.effects.into_values().map(rd::REffect::new));
        let online_effect = storage_effects.get(&ac::effects::ONLINE).cloned();
        Ok(Self {
            storage_items: map_to_arcmap(a_data.items.into_values().map(rd::RItem::new)),
            storage_attrs: map_to_arcmap(a_data.attrs.into_values().map(rd::RAttr::new)),
            storage_effects,
            storage_buffs: map_to_arcmap(a_data.buffs.into_values().map(rd::RBuff::new)),
            storage_mutas: map_to_arcmap(a_data.mutas.into_values().map(rd::RMuta::new)),
            online_effect,
        })
    }
    pub(crate) fn get_r_item(&self, id: &ad::AItemId) -> Option<&Arc<rd::RItem>> {
        self.storage_items.get(id)
    }
    pub(crate) fn get_r_attr(&self, id: &ad::AAttrId) -> Option<&Arc<rd::RAttr>> {
        self.storage_attrs.get(id)
    }
    pub(crate) fn get_r_effect(&self, id: &ad::AEffectId) -> Option<&Arc<rd::REffect>> {
        self.storage_effects.get(id)
    }
    pub(crate) fn get_r_effect_online(&self) -> Option<&Arc<rd::REffect>> {
        self.online_effect.as_ref()
    }
    pub(crate) fn get_r_buff(&self, id: &ad::ABuffId) -> Option<&Arc<rd::RBuff>> {
        self.storage_buffs.get(id)
    }
    pub(crate) fn get_r_mutator(&self, id: &ad::AItemId) -> Option<&Arc<rd::RMuta>> {
        self.storage_mutas.get(id)
    }
}
