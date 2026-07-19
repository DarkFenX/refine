use rc::ItemCommon;

use super::shared::{get_attrs, get_effects, get_mods};
#[cfg(feature = "serde")]
use crate::ItemKind;
use crate::{
    AttrId, AttrVals, ChargeInfo, Count, EffectId, EffectInfo, FitId, Index, ItemId, ItemInfoMode, ItemMutationInfo,
    ItemOptionalReloadInfo, ItemSpoolInfo, ItemTypeId, ModRack, Modification, ModuleState, RangedProjInfo,
    TriStateField,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ModuleInfo {
    pub id: ItemId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub charge: Option<ChargeInfo>,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<ModuleInfoExt>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
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
    pub optional_reload: ItemOptionalReloadInfo,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub projs: Vec<RangedProjInfo>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub attrs: Vec<(AttrId, AttrVals)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub effects: Vec<(EffectId, EffectInfo)>,
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
    pub(in crate::info) fn from_core(core_module: &mut rc::ModuleMut, item_mode: ItemInfoMode) -> Self {
        let charge_info = core_module
            .get_charge_mut()
            .map(|mut core_charge| ChargeInfo::from_core(&mut core_charge, item_mode));
        let has_charge = charge_info.is_some();
        Self {
            id: core_module.get_item_id(),
            charge: charge_info,
            extended: match item_mode {
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
                        mutation: core_module.get_mutation().and_then(ItemMutationInfo::try_from_core),
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
