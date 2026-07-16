use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, AttrVals, EffectId, EffectInfo, FitId, ItemId, ItemInfoMode, ItemTypeId, Modification, ServiceState,
};

pub struct ServiceInfo {
    pub id: ItemId,
    pub extended: Option<ServiceInfoExt>,
}

pub struct ServiceInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: ServiceState,
    pub attrs: Vec<(AttrId, AttrVals)>,
    pub effects: Vec<(EffectId, EffectInfo)>,
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
