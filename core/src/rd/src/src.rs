use std::sync::Arc;

use super::{error::SrcInitError, prepare::prepare_adapted_data};
use crate::{
    ad::{AAbilId, AAttrId, ABuffId, AEffectId, AItemId, AItemListId, AdaptedDataCacher},
    ed::EveDataHandler,
    rd::{
        RAbil, RAttr, RAttrConsts, RAttrId, RBuff, RBuffId, RData, REffectConsts, REffectId, RItemList, RItemListId,
        RcEffect, RcItem, RcMuta,
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
    rah_duration_attr_rid: Option<RAttrId>,
}
impl Src {
    #[tracing::instrument(name = "src-new", level = "trace", skip_all)]
    pub fn new(
        ed_handler: &dyn EveDataHandler,
        ad_cacher: Option<&mut Box<dyn AdaptedDataCacher>>,
    ) -> Result<Self, SrcInitError> {
        let a_data = prepare_adapted_data(ed_handler, ad_cacher)?;
        let r_data = RData::from_a_data(a_data);
        let rah_duration_attr_rid = r_data
            .effect_consts
            .adaptive_armor_hardener
            .and_then(|effect_rid| r_data.effects.get(effect_rid.into_usize()).unwrap().duration_attr_rid);
        let online_effect = r_data
            .effect_consts
            .online
            .map(|effect_rid| r_data.effects.get(effect_rid.into_usize()).unwrap().clone());
        Ok(Self {
            r_data: Arc::new(r_data),
            online_effect,
            rah_duration_attr_rid,
        })
    }
    // Item methods
    pub(crate) fn get_item_by_aid(&self, item_aid: &AItemId) -> Option<&RcItem> {
        self.r_data.items.get(item_aid)
    }
    // Item list methods
    pub(crate) fn get_item_list_by_rid(&self, item_list_rid: RItemListId) -> &RItemList {
        self.r_data.item_lists.get(item_list_rid.into_usize()).unwrap()
    }
    pub(crate) fn get_item_list_rid_by_aid(&self, item_list_aid: &AItemListId) -> Option<RItemListId> {
        self.r_data.item_list_aid_rid_map.get(item_list_aid).copied()
    }
    // Attr methods
    pub(crate) fn get_attr_by_rid(&self, attr_rid: RAttrId) -> &RAttr {
        self.r_data.attrs.get(attr_rid.into_usize()).unwrap()
    }
    pub(crate) fn get_attr_rid_by_aid(&self, attr_aid: &AAttrId) -> Option<RAttrId> {
        self.r_data.attr_aid_rid_map.get(attr_aid).copied()
    }
    pub(crate) fn get_attr_aid_rid_map(&self) -> &RMap<AAttrId, RAttrId> {
        &self.r_data.attr_aid_rid_map
    }
    pub(crate) fn get_attr_consts(&self) -> &RAttrConsts {
        &self.r_data.attr_consts
    }
    // Attr methods
    pub(crate) fn get_effect_by_rid(&self, effect_rid: REffectId) -> &RcEffect {
        self.r_data.effects.get(effect_rid.into_usize()).unwrap()
    }
    pub(crate) fn get_effect_rid_by_aid(&self, effect_aid: &AEffectId) -> Option<REffectId> {
        self.r_data.effect_aid_rid_map.get(effect_aid).copied()
    }
    pub(crate) fn get_effect_consts(&self) -> &REffectConsts {
        &self.r_data.effect_consts
    }
    // Buff methods
    pub(crate) fn get_buff_by_rid(&self, buff_rid: RBuffId) -> &RBuff {
        self.r_data.buffs.get(buff_rid.into_usize()).unwrap()
    }
    pub(crate) fn get_buff_by_aid(&self, buff_aid: &ABuffId) -> Option<&RBuff> {
        let buff_rid = *self.r_data.buff_aid_rid_map.get(buff_aid)?;
        Some(self.get_buff_by_rid(buff_rid))
    }
    // Mutator methods
    pub(crate) fn get_mutator_by_aid(&self, item_aid: &AItemId) -> Option<&RcMuta> {
        self.r_data.mutas.get(item_aid)
    }
    // Abilitu methods
    pub(crate) fn get_ability_by_aid(&self, ability_aid: &AAbilId) -> Option<&RAbil> {
        self.r_data.abils.get(ability_aid)
    }
    // Misc getters
    pub(crate) fn get_online_effect(&self) -> Option<&RcEffect> {
        self.online_effect.as_ref()
    }
    pub(crate) fn get_rah_duration_attr_rid(&self) -> Option<RAttrId> {
        self.rah_duration_attr_rid
    }
}
