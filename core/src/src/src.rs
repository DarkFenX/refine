use std::sync::Arc;

use crate::{
    ac, ad, ed, rd,
    src::{SrcInitError, prepare::prepare_adapted_data},
};

/// Data source.
///
/// Data source is a top-level entity which manages EVE and adapted data handlers to do necessary
/// preparations and expose adapted data to solar system and its services.
#[derive(Clone)]
pub struct Src {
    r_data: rd::RData,
    online_effect: Option<Arc<rd::REffect>>,
}
impl Src {
    #[tracing::instrument(name = "src-new", level = "trace", skip_all)]
    pub fn new(
        ed_handler: Box<dyn ed::EveDataHandler>,
        ad_cacher: Option<Box<dyn ad::AdaptedDataCacher>>,
    ) -> Result<Self, SrcInitError> {
        let a_data = prepare_adapted_data(ed_handler, ad_cacher)?;
        let r_data = rd::RData::from(a_data);
        let online_effect = r_data.effects.get(&ac::effects::ONLINE).cloned();
        Ok(Self { r_data, online_effect })
    }
    pub(crate) fn get_r_item(&self, id: &ad::AItemId) -> Option<&Arc<rd::RItem>> {
        self.r_data.items.get(id)
    }
    pub(crate) fn get_r_attr(&self, id: &ad::AAttrId) -> Option<&Arc<rd::RAttr>> {
        self.r_data.attrs.get(id)
    }
    pub(crate) fn get_r_effect(&self, id: &ad::AEffectId) -> Option<&Arc<rd::REffect>> {
        self.r_data.effects.get(id)
    }
    pub(crate) fn get_r_effect_online(&self) -> Option<&Arc<rd::REffect>> {
        self.online_effect.as_ref()
    }
    pub(crate) fn get_r_buff(&self, id: &ad::ABuffId) -> Option<&Arc<rd::RBuff>> {
        self.r_data.buffs.get(id)
    }
    pub(crate) fn get_r_mutator(&self, id: &ad::AItemId) -> Option<&Arc<rd::RMuta>> {
        self.r_data.mutas.get(id)
    }
}
