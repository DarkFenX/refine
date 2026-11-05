use rc::ItemCommon;

#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    EffectId, EffectMode, FitId, ItemId, ItemInfoFull, ItemInfoMode, ItemTypeId, ServiceState, shared::OvrdMapLight,
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
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effect_mode_overrides: Vec<(EffectId, EffectMode)>,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub full: Option<ItemInfoFull>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ServiceInfo {
    pub(in crate::info) fn from_core(
        core_service: &mut rc::Service,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let service_id = core_service.get_item_id();
        let service_info_mode = item_info_modes.get(&service_id);
        Self {
            id: service_id,
            extended: match service_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(ServiceInfoExt {
                    #[cfg(feature = "serde")]
                    kind: ItemKind::Service,
                    type_id: core_service.get_type_id(),
                    fit_id: core_service.get_fit_id(),
                    state: core_service.get_state(),
                    effect_mode_overrides: core_service.iter_effect_mode_overrides().collect(),
                    full: ItemInfoFull::try_from_core(core_service, service_info_mode),
                }),
            },
        }
    }
}
