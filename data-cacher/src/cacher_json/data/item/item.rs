use super::{super::shared::CState, effect_data::CItemEffectData};

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json::data) struct CItem {
    id: i32,
    grp_id: i32,
    cat_id: i32,
    attrs: Vec<CItemAttr>,
    effect_datas: Vec<CItemEffect>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>", no_default)]
    defeff_id: Option<rc::ad::AEffectId>,
    abil_ids: Vec<i32>,
    srqs: Vec<(i32, u8)>,
    max_state: CState,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    fleet_buff_item_list_ids: Vec<rc::ad::AItemListId>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    proj_buff_item_list_ids: Vec<rc::ad::AItemListId>,
    val_fitted_group_id: Option<i32>,
    val_online_group_id: Option<i32>,
    val_active_group_id: Option<i32>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    cap_use_attr_ids: Vec<rc::ad::AAttrId>,
    is_ice_harvester: bool,
    disallowed_in_wspace: bool,
}
impl CItem {
    pub(in crate::cacher_json::data) fn from_adapted(a_item: &rc::ad::AItem) -> Self {
        Self {
            id: a_item.id.into_i32(),
            grp_id: a_item.grp_id.into_i32(),
            cat_id: a_item.cat_id.into_i32(),
            attrs: a_item
                .attrs
                .iter()
                .map(|(k, v)| CItemAttr {
                    id: *k,
                    val: v.into_f64(),
                })
                .collect(),
            effect_datas: a_item
                .effect_datas
                .iter()
                .map(|(effect_aid, a_effect_data)| CItemEffect {
                    id: *effect_aid,
                    data: CItemEffectData::from_adapted(a_effect_data),
                })
                .collect(),
            defeff_id: a_item.defeff_id,
            abil_ids: a_item.abil_ids.iter().map(|v| v.into_i32()).collect(),
            srqs: a_item.srqs.iter().map(|(k, v)| (k.into_i32(), v.into_u8())).collect(),
            max_state: CState::from_adapted(&a_item.max_state),
            proj_buff_item_list_ids: a_item.proj_buff_item_list_ids.clone(),
            fleet_buff_item_list_ids: a_item.fleet_buff_item_list_ids.clone(),
            val_fitted_group_id: a_item.val_fitted_group_id.map(|grp_aid| grp_aid.into_i32()),
            val_online_group_id: a_item.val_online_group_id.map(|grp_aid| grp_aid.into_i32()),
            val_active_group_id: a_item.val_active_group_id.map(|grp_aid| grp_aid.into_i32()),
            cap_use_attr_ids: a_item.cap_use_attr_ids.clone(),
            is_ice_harvester: a_item.is_ice_harvester,
            disallowed_in_wspace: a_item.disallowed_in_wspace,
        }
    }
    pub(in crate::cacher_json::data) fn into_adapted(self) -> rc::ad::AItem {
        rc::ad::AItem {
            id: rc::ad::AItemId::from_i32(self.id),
            grp_id: rc::ad::AItemGrpId::from_i32(self.grp_id),
            cat_id: rc::ad::AItemCatId::from_i32(self.cat_id),
            attrs: self
                .attrs
                .into_iter()
                .map(|v| (v.id, rc::ad::AValue::from_f64(v.val)))
                .collect(),
            effect_datas: self
                .effect_datas
                .into_iter()
                .map(|v| (v.id, v.data.into_adapted()))
                .collect(),
            defeff_id: self.defeff_id,
            abil_ids: self
                .abil_ids
                .into_iter()
                .map(|v| rc::ad::AAbilId::from_i32(v))
                .collect(),
            srqs: self
                .srqs
                .into_iter()
                .map(|(k, v)| (rc::ad::AItemId::from_i32(k), rc::ad::ASkillLevel::from_u8_clamped(v)))
                .collect(),
            max_state: self.max_state.into_adapted(),
            proj_buff_item_list_ids: self.proj_buff_item_list_ids.clone(),
            fleet_buff_item_list_ids: self.fleet_buff_item_list_ids.clone(),
            val_fitted_group_id: self.val_fitted_group_id.map(|v| rc::ad::AItemGrpId::from_i32(v)),
            val_online_group_id: self.val_online_group_id.map(|v| rc::ad::AItemGrpId::from_i32(v)),
            val_active_group_id: self.val_active_group_id.map(|v| rc::ad::AItemGrpId::from_i32(v)),
            cap_use_attr_ids: self.cap_use_attr_ids.clone(),
            is_ice_harvester: self.is_ice_harvester,
            disallowed_in_wspace: self.disallowed_in_wspace,
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct CItemAttr {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ad::AAttrId,
    val: f64,
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct CItemEffect {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ad::AEffectId,
    data: CItemEffectData,
}
