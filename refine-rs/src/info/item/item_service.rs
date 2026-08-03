use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, FitId, ItemAttrInfo, ItemEffectInfo, ItemId, ItemInfoMode, ItemTypeId, Modification, ServiceState,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct ServiceInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<ServiceInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct ServiceInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: ServiceState,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::KeyValueMap<_>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub attrs: Vec<ItemAttrInfo>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::KeyValueMap<_>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effects: Vec<ItemEffectInfo>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub mods: Vec<(AttrId, Vec<Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ServiceInfo {
    pub(in crate::info) fn from_core(core_service: &mut rc::ServiceMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_service.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ServiceInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Service,
                    type_id: core_service.get_type_id(),
                    fit_id: core_service.get_fit().get_fit_id(),
                    state: core_service.get_state(),
                    attrs: get_attrs(core_service, item_mode),
                    effects: get_effects(core_service, item_mode),
                    mods: get_mods(core_service, item_mode),
                }),
            },
        }
    }
}
