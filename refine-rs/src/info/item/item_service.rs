use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct ServiceInfo {
    pub id: rc::ItemId,
    pub extended: Option<ServiceInfoExt>,
}

pub struct ServiceInfoExt {
    kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub state: rc::ServiceState,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
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
                    kind: rc::ItemKind::Service,
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
