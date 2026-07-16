use rc::ItemCommon;

use super::shared::{ItemMutationInfo, RangedProjInfo, get_attrs, get_effects, get_mods};
use crate::{
    info::{ChargeInfo, ItemInfoMode},
    util::TriStateField,
};

pub struct ModuleInfo {
    pub id: rc::ItemId,
    pub extended: Option<ModuleInfoExt>,
}

pub struct ModuleInfoExt {
    pub kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub state: rc::ModuleState,
    pub rack: rc::ModRack,
    pub pos: rc::Index,
    pub mutation: Option<ItemMutationInfo>,
    pub charge: Option<ChargeInfo>,
    pub charge_count: TriStateField<rc::Count>,
    pub charged_cycles: TriStateField<rc::Count>,
    pub spool_cycles: Option<rc::ItemSpoolInfo>,
    pub optional_reload: rc::ItemOptionalReloadInfo,
    pub projs: Vec<RangedProjInfo>,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ModuleInfo {
    pub(in crate::info) fn from_core(core_module: &mut rc::ModuleMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_module.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => {
                    let charge_info = core_module
                        .get_charge_mut()
                        .map(|mut core_charge| ChargeInfo::from_core(&mut core_charge, item_mode));
                    let charge_count = match charge_info.is_some() {
                        true => match core_module.get_charge_count() {
                            Some(charge_count) => TriStateField::Value(charge_count),
                            None => TriStateField::None,
                        },
                        false => TriStateField::Absent,
                    };
                    let charged_cycle_count = match charge_info.is_some() {
                        true => match core_module.get_charged_cycle_count() {
                            Some(charged_cycle_count) => TriStateField::Value(charged_cycle_count),
                            None => TriStateField::None,
                        },
                        false => TriStateField::Absent,
                    };
                    Some(ModuleInfoExt {
                        kind: rc::ItemKind::Module,
                        type_id: core_module.get_type_id(),
                        fit_id: core_module.get_fit().get_fit_id(),
                        state: core_module.get_state(),
                        rack: core_module.get_rack(),
                        pos: core_module.get_pos(),
                        mutation: core_module.get_mutation().and_then(ItemMutationInfo::try_from_core),
                        charge: charge_info,
                        charge_count,
                        charged_cycles: charged_cycle_count,
                        spool_cycles: core_module.get_spool_cycle_count(),
                        optional_reload: core_module.get_optional_reload(),
                        projs: core_module.iter_projs().map(RangedProjInfo::from_core).collect(),
                        attrs: get_attrs(core_module, item_mode),
                        effects: get_effects(core_module, item_mode),
                        mods: get_mods(core_module, item_mode),
                    })
                }
            },
        }
    }
}
