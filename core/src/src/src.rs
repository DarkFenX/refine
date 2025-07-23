use crate::{
    ac, ad, ed,
    src::{SrcInitError, key::map_to_arcmap, prepare::prepare_adapted_data},
    util::RMap,
};

/// Data source.
///
/// Data source is a top-level entity which manages EVE and adapted data handlers to do necessary
/// preparations and expose adapted data to solar system and its services.
#[derive(Clone)]
pub struct Src {
    storage_items: RMap<ad::AItemId, ad::ArcItemRt>,
    storage_attrs: RMap<ad::AAttrId, ad::ArcAttr>,
    storage_effects: RMap<ad::AEffectId, ad::ArcEffectRt>,
    storage_buffs: RMap<ad::ABuffId, ad::ArcBuff>,
    storage_mutas: RMap<ad::AItemId, ad::ArcMuta>,
    online_effect: Option<ad::ArcEffectRt>,
}
impl Src {
    #[tracing::instrument(name = "src-new", level = "trace", skip_all)]
    pub fn new(
        ed_handler: Box<dyn ed::EveDataHandler>,
        ad_cacher: Option<Box<dyn ad::AdaptedDataCacher>>,
    ) -> Result<Self, SrcInitError> {
        let a_data = prepare_adapted_data(ed_handler, ad_cacher)?;
        let storage_effects = map_to_arcmap(a_data.effects.into_values().map(ad::AEffectRt::new));
        let online_effect = storage_effects.get(&ac::effects::ONLINE).cloned();
        Ok(Self {
            storage_items: map_to_arcmap(a_data.items.into_values().map(ad::AItemRt::new)),
            storage_attrs: map_to_arcmap(a_data.attrs.into_values()),
            storage_effects,
            storage_buffs: map_to_arcmap(a_data.buffs.into_values()),
            storage_mutas: map_to_arcmap(a_data.mutas.into_values()),
            online_effect,
        })
    }
    pub(crate) fn get_a_item(&self, id: &ad::AItemId) -> Option<&ad::ArcItemRt> {
        self.storage_items.get(id)
    }
    pub(crate) fn get_a_attr(&self, id: &ad::AAttrId) -> Option<&ad::ArcAttr> {
        self.storage_attrs.get(id)
    }
    pub(crate) fn get_a_effect(&self, id: &ad::AEffectId) -> Option<&ad::ArcEffectRt> {
        self.storage_effects.get(id)
    }
    pub(crate) fn get_a_effect_online(&self) -> Option<&ad::ArcEffectRt> {
        self.online_effect.as_ref()
    }
    pub(crate) fn get_a_buff(&self, id: &ad::ABuffId) -> Option<&ad::ArcBuff> {
        self.storage_buffs.get(id)
    }
    pub(crate) fn get_a_mutator(&self, id: &ad::AItemId) -> Option<&ad::ArcMuta> {
        self.storage_mutas.get(id)
    }
}
