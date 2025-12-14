use std::sync::Arc;

use super::{error::SrcInitError, prepare::prepare_adapted_data};
use crate::{
    ad::{AAbilId, AAttrId, ABuffId, AEffectId, AItemId, AItemListId, AdaptedDataCacher},
    ed::EveDataHandler,
    rd::{
        RAbil, RAttr, RAttrConsts, RAttrKey, RBuff, RBuffKey, RData, REffectConsts, REffectKey, RItemList,
        RItemListKey, RcEffect, RcItem, RcMuta,
    },
    util::RMap,
};

/// Data source.
///
/// Data source is a top-level entity which manages EVE data handler and adapted data cacher to do
/// necessary preparations and expose processed data to solar system and its services.
// Under the hood it's an entity which builds runtime data container, and then provides access to
// its contents
#[derive(Clone)]
pub struct Src {
    r_data: Arc<RData>,
    online_effect: Option<RcEffect>,
    rah_duration_attr_key: Option<RAttrKey>,
}
impl Src {
    #[tracing::instrument(name = "src-new", level = "trace", skip_all)]
    pub fn new(
        ed_handler: &dyn EveDataHandler,
        ad_cacher: Option<&mut Box<dyn AdaptedDataCacher>>,
    ) -> Result<Self, SrcInitError> {
        let a_data = prepare_adapted_data(ed_handler, ad_cacher)?;
        let r_data = RData::from(a_data);
        let rah_duration_attr_key = r_data
            .effect_consts
            .adaptive_armor_hardener
            .and_then(|v| r_data.effects.get(v).unwrap().duration_attr_key);
        let online_effect = r_data
            .effect_consts
            .online
            .map(|v| r_data.effects.get(v).unwrap().clone());
        Ok(Self {
            r_data: Arc::new(r_data),
            online_effect,
            rah_duration_attr_key,
        })
    }
    // Item methods
    pub(crate) fn get_item(&self, id: &AItemId) -> Option<&RcItem> {
        self.r_data.items.get(id)
    }
    // Item list methods
    pub(crate) fn get_item_list(&self, key: RItemListKey) -> &RItemList {
        self.r_data.item_lists.get(key).unwrap()
    }
    pub(crate) fn get_item_list_key_by_id(&self, id: &AItemListId) -> Option<RItemListKey> {
        self.r_data.item_list_id_key_map.get(id).copied()
    }
    // Attr methods
    pub(crate) fn get_attr(&self, key: RAttrKey) -> &RAttr {
        self.r_data.attrs.get(key).unwrap()
    }
    pub(crate) fn get_attr_key_by_id(&self, id: &AAttrId) -> Option<RAttrKey> {
        self.r_data.attr_id_key_map.get(id).copied()
    }
    pub(crate) fn get_attr_id_key_map(&self) -> &RMap<AAttrId, RAttrKey> {
        &self.r_data.attr_id_key_map
    }
    pub(crate) fn get_attr_consts(&self) -> &RAttrConsts {
        &self.r_data.attr_consts
    }
    // Attr methods
    pub(crate) fn get_effect(&self, key: REffectKey) -> &RcEffect {
        self.r_data.effects.get(key).unwrap()
    }
    pub(crate) fn get_effect_key_by_id(&self, id: &AEffectId) -> Option<REffectKey> {
        self.r_data.effect_id_key_map.get(id).copied()
    }
    pub(crate) fn get_effect_consts(&self) -> &REffectConsts {
        &self.r_data.effect_consts
    }
    // Buff methods
    pub(crate) fn get_buff(&self, key: RBuffKey) -> &RBuff {
        self.r_data.buffs.get(key).unwrap()
    }
    pub(crate) fn get_buff_by_id(&self, id: &ABuffId) -> Option<&RBuff> {
        let buff_key = *self.r_data.buff_id_key_map.get(id)?;
        Some(self.get_buff(buff_key))
    }
    // Mutator methods
    pub(crate) fn get_mutator(&self, id: &AItemId) -> Option<&RcMuta> {
        self.r_data.mutas.get(id)
    }
    // Abilitu methods
    pub(crate) fn get_ability(&self, id: &AAbilId) -> Option<&RAbil> {
        self.r_data.abils.get(id)
    }
    // Misc getters
    pub(crate) fn get_online_effect(&self) -> Option<&RcEffect> {
        self.online_effect.as_ref()
    }
    pub(crate) fn get_rah_duration_attr_key(&self) -> Option<RAttrKey> {
        self.rah_duration_attr_key
    }
}
