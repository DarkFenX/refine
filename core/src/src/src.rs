use crate::{
    ac,
    ad::{AAbilId, AAttrId, ABuffId, AEffectId, AItemId, AItemListId, AdaptedDataCacher},
    ed::EveDataHandler,
    rd::{RData, REffectKey, RcAbil, RcAttr, RcBuff, RcEffect, RcItem, RcItemList, RcMuta},
    src::{SrcInitError, prepare::prepare_adapted_data},
    util::{GetId, RMap},
};

/// Data source.
///
/// Data source is a top-level entity which manages EVE and adapted data handlers to do necessary
/// preparations and expose adapted data to solar system and its services.
#[derive(Clone)]
pub struct Src {
    r_data: RData,
    effect_id_key_map: RMap<AEffectId, REffectKey>,
    online_effect_key: Option<REffectKey>,
    online_effect: Option<RcEffect>,
    rah_effect_key: Option<REffectKey>,
    rah_duration_attr_id: Option<AAttrId>,
    hi_slot_effect_key: Option<REffectKey>,
    mid_slot_effect_key: Option<REffectKey>,
    low_slot_effect_key: Option<REffectKey>,
    rig_slot_effect_key: Option<REffectKey>,
    svc_slot_effect_key: Option<REffectKey>,
}
impl Src {
    #[tracing::instrument(name = "src-new", level = "trace", skip_all)]
    pub fn new(
        ed_handler: Box<dyn EveDataHandler>,
        ad_cacher: Option<Box<dyn AdaptedDataCacher>>,
    ) -> Result<Self, SrcInitError> {
        let a_data = prepare_adapted_data(ed_handler, ad_cacher)?;
        let r_data = RData::from(a_data);
        let effect_id_key_map = r_data
            .effects
            .iter()
            .map(|(k, v)| (v.get_id(), k))
            .collect::<RMap<_, _>>();
        let online_effect_key = effect_id_key_map.get(&ac::effects::ONLINE).copied();
        let online_effect = online_effect_key.map(|v| r_data.effects.get(v).unwrap().clone());
        let rah_effect_key = effect_id_key_map.get(&ac::effects::ADAPTIVE_ARMOR_HARDENER).copied();
        let rah_duration_attr_id = rah_effect_key.and_then(|v| r_data.effects.get(v).unwrap().get_duration_attr_id());
        let hi_slot_effect_key = effect_id_key_map.get(&ac::effects::HI_POWER).copied();
        let mid_slot_effect_key = effect_id_key_map.get(&ac::effects::MED_POWER).copied();
        let low_slot_effect_key = effect_id_key_map.get(&ac::effects::LO_POWER).copied();
        let rig_slot_effect_key = effect_id_key_map.get(&ac::effects::RIG_SLOT).copied();
        let svc_slot_effect_key = effect_id_key_map.get(&ac::effects::SERVICE_SLOT).copied();
        Ok(Self {
            r_data,
            effect_id_key_map,
            online_effect_key,
            online_effect,
            rah_effect_key,
            rah_duration_attr_id,
            hi_slot_effect_key,
            mid_slot_effect_key,
            low_slot_effect_key,
            rig_slot_effect_key,
            svc_slot_effect_key,
        })
    }
    pub(crate) fn get_item(&self, id: &AItemId) -> Option<&RcItem> {
        self.r_data.items.get(id)
    }
    pub(crate) fn get_item_list(&self, id: &AItemListId) -> Option<&RcItemList> {
        self.r_data.item_lists.get(id)
    }
    pub(crate) fn get_attr(&self, id: &AAttrId) -> Option<&RcAttr> {
        self.r_data.attrs.get(id)
    }
    pub(crate) fn get_effect(&self, key: REffectKey) -> &RcEffect {
        self.r_data.effects.get(key).unwrap()
    }
    pub(crate) fn get_buff(&self, id: &ABuffId) -> Option<&RcBuff> {
        self.r_data.buffs.get(id)
    }
    pub(crate) fn get_mutator(&self, id: &AItemId) -> Option<&RcMuta> {
        self.r_data.mutas.get(id)
    }
    pub(crate) fn get_ability(&self, id: &AAbilId) -> Option<&RcAbil> {
        self.r_data.abils.get(id)
    }
    // Misc getters
    pub(crate) fn get_effect_key_by_id(&self, id: &AEffectId) -> Option<REffectKey> {
        self.effect_id_key_map.get(id).copied()
    }
    pub(crate) fn get_online_effect_key(&self) -> Option<REffectKey> {
        self.online_effect_key
    }
    pub(crate) fn get_online_effect(&self) -> Option<&RcEffect> {
        self.online_effect.as_ref()
    }
    pub(crate) fn get_rah_effect_key(&self) -> Option<REffectKey> {
        self.rah_effect_key
    }
    pub(crate) fn get_rah_duration_attr_id(&self) -> Option<AAttrId> {
        self.rah_duration_attr_id
    }
    pub(crate) fn get_hi_slot_effect_key(&self) -> Option<REffectKey> {
        self.hi_slot_effect_key
    }
    pub(crate) fn get_mid_slot_effect_key(&self) -> Option<REffectKey> {
        self.mid_slot_effect_key
    }
    pub(crate) fn get_low_slot_effect_key(&self) -> Option<REffectKey> {
        self.low_slot_effect_key
    }
    pub(crate) fn get_rig_slot_effect_key(&self) -> Option<REffectKey> {
        self.rig_slot_effect_key
    }
    pub(crate) fn get_svc_slot_effect_key(&self) -> Option<REffectKey> {
        self.svc_slot_effect_key
    }
}
