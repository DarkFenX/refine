use rc::{ItemCommon, Lender};

use super::shared::{get_attrs, get_effects, get_mods};
use crate::info::ItemInfoMode;

pub struct BoosterInfo {
    pub id: rc::ItemId,
    pub extended: Option<BoosterInfoExt>,
}

pub struct BoosterInfoExt {
    pub kind: rc::ItemKind,
    pub type_id: rc::ItemTypeId,
    pub fit_id: rc::FitId,
    pub slot: Option<rc::SlotIndex>,
    pub state: bool,
    pub side_effects: Vec<(rc::EffectId, SideEffectInfo)>,
    pub attrs: Vec<(rc::AttrId, rc::AttrVals)>,
    pub effects: Vec<(rc::EffectId, rc::EffectInfo)>,
    pub mods: Vec<(rc::AttrId, Vec<rc::Modification>)>,
}

pub struct SideEffectInfo {
    pub chance: rc::UnitInterval,
    pub state: bool,
    pub modification: Option<SideEffectMod>,
}

pub struct SideEffectMod {
    pub op: SideEffectOp,
    pub str: rc::Value,
}

pub enum SideEffectOp {
    Add,
    Perc,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BoosterInfo {
    pub(super) fn from_core(core_booster: &mut rc::BoosterMut, item_mode: ItemInfoMode) -> Self {
        Self {
            id: core_booster.get_item_id(),
            extended: match item_mode {
                ItemInfoMode::Id => None,
                ItemInfoMode::Partial | ItemInfoMode::Full => Some(BoosterInfoExt {
                    kind: rc::ItemKind::Booster,
                    type_id: core_booster.get_type_id(),
                    fit_id: core_booster.get_fit().get_fit_id(),
                    slot: core_booster.get_slot(),
                    state: core_booster.get_state(),
                    side_effects: core_booster
                        .iter_side_effects_mut()
                        .map_into_iter(|core_side_effect| {
                            (
                                core_side_effect.get_effect_id(),
                                SideEffectInfo::from_core(core_side_effect),
                            )
                        })
                        .collect(),
                    attrs: get_attrs(core_booster, item_mode),
                    effects: get_effects(core_booster, item_mode),
                    mods: get_mods(core_booster, item_mode),
                }),
            },
        }
    }
}

impl SideEffectInfo {
    fn from_core(mut core_side_effect: rc::SideEffectMut) -> Self {
        Self {
            chance: core_side_effect.get_chance(),
            state: core_side_effect.get_state(),
            modification: core_side_effect.get_strength().and_then(SideEffectMod::try_from_core),
        }
    }
}

impl SideEffectMod {
    fn try_from_core(core_sid_str: rc::SideEffectStr) -> Option<Self> {
        let raw_strength = core_sid_str.get_strength();
        match core_sid_str.get_op() {
            rc::Op::Add | rc::Op::ExtraAdd => Some(Self {
                op: SideEffectOp::Add,
                str: raw_strength,
            }),
            rc::Op::Sub => Some(Self {
                op: SideEffectOp::Add,
                str: rc::Value::from_f64(-raw_strength.into_f64()),
            }),
            rc::Op::PreMul | rc::Op::PostMul | rc::Op::ExtraMul => Some(Self {
                op: SideEffectOp::Perc,
                str: rc::Value::from_f64(raw_strength.into_f64().mul_add(100.0, -100.0)),
            }),
            rc::Op::PreDiv | rc::Op::PostDiv => match raw_strength.into_f64() {
                0.0 => None,
                v => Some(Self {
                    op: SideEffectOp::Perc,
                    str: rc::Value::from_f64(100.0 / v - 100.0),
                }),
            },
            rc::Op::PostPerc => Some(Self {
                op: SideEffectOp::Perc,
                str: raw_strength,
            }),
            rc::Op::BaseAssign | rc::Op::PreAssign | rc::Op::PostAssign | rc::Op::MinLimit | rc::Op::MaxLimit => None,
        }
    }
}
