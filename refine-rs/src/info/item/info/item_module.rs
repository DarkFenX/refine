use rc::ItemCommon;

use super::shared::{get_attrs, get_effect_mode_overrides, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, ChargeInfo, Count, EffectId, EffectMode, FitId, Index, ItemAttrValues, ItemEffectInfo, ItemId,
    ItemInfoMode, ItemMutationInfo, ItemSpoolInfo, ItemTypeId, ModRack, Modification, ModuleState, OptionalReload,
    RangedProjInfo, Spool, TriStateField, shared::OvrdMapLight,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct ModuleInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub charge: Option<ChargeInfo>,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<ModuleInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct ModuleInfoExt {
    #[cfg(feature = "serde")]
    kind: ItemKind,
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub state: ModuleState,
    pub rack: ModRack,
    pub pos: Index,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mutation: Option<ItemMutationInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "TriStateField::is_absent"))]
    pub charge_count: TriStateField<Count>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "TriStateField::is_absent"))]
    pub charged_cycles: TriStateField<Count>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub spool_cycles: Option<ItemSpoolInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub spool_override: Option<Spool>,
    pub optional_reload: OptionalReload,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub optional_reload_override: Option<OptionalReload>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub projs: Vec<RangedProjInfo>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effect_mode_overrides: Vec<(EffectId, EffectMode)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub attrs: Vec<(AttrId, ItemAttrValues)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effects: Vec<(EffectId, ItemEffectInfo)>,
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
impl ModuleInfo {
    pub(in crate::info) fn from_core(
        core_module: &mut rc::ModuleMut,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        let module_id = core_module.get_item_id();
        let module_info_mode = item_info_modes.get(&module_id);
        let charge_info = core_module
            .get_charge_mut()
            .map(|mut core_charge| ChargeInfo::from_core(&mut core_charge, item_info_modes));
        let has_charge = charge_info.is_some();
        Self {
            id: module_id,
            charge: charge_info,
            extended: match module_info_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => {
                    let charge_count = match has_charge {
                        true => match core_module.get_charge_count() {
                            Some(charge_count) => TriStateField::Value(charge_count),
                            None => TriStateField::None,
                        },
                        false => TriStateField::Absent,
                    };
                    let charged_cycle_count = match has_charge {
                        true => match core_module.get_charged_cycle_count() {
                            Some(charged_cycle_count) => TriStateField::Value(charged_cycle_count),
                            None => TriStateField::None,
                        },
                        false => TriStateField::Absent,
                    };
                    Some(ModuleInfoExt {
                        #[cfg(feature = "serde")]
                        kind: ItemKind::Module,
                        type_id: core_module.get_type_id(),
                        fit_id: core_module.get_fit().get_fit_id(),
                        state: core_module.get_state(),
                        rack: core_module.get_rack(),
                        pos: core_module.get_pos(),
                        mutation: core_module.get_mutation().map(ItemMutationInfo::try_from_core),
                        charge_count,
                        charged_cycles: charged_cycle_count,
                        spool_cycles: core_module.get_spool_cycle_count(),
                        spool_override: core_module.get_spool_override(),
                        optional_reload: core_module.get_optional_reload(),
                        optional_reload_override: core_module.get_optional_reload_override(),
                        projs: core_module.iter_projs().map(RangedProjInfo::from_core).collect(),
                        effect_mode_overrides: get_effect_mode_overrides(core_module, module_info_mode),
                        attrs: get_attrs(core_module, module_info_mode),
                        effects: get_effects(core_module, module_info_mode),
                        mods: get_mods(core_module, module_info_mode),
                    })
                }
            },
        }
    }
}
