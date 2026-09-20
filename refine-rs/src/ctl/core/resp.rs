use rc::ItemCommon;

use crate::{EffectId, FitId, FleetId, ItemId};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone)]
pub struct AddedFleetIdResp {
    pub fleet_id: FleetId,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone)]
pub struct AddedFitIdResp {
    pub fit_id: FitId,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct AddedItemIdsResp {
    pub item_id: ItemId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub charge_item_id: Option<ItemId> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "AutochargeItemIds::is_not_generated"))]
    pub autocharge_item_ids: AutochargeItemIds = AutochargeItemIds::NotGenerated,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Default)]
pub struct ChangedItemIdsResp {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub charge_item_id: Option<ItemId> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "AutochargeItemIds::is_not_generated"))]
    pub autocharge_item_ids: AutochargeItemIds = AutochargeItemIds::NotGenerated,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AddedFleetIdResp {
    pub(in crate::ctl::core) fn from_core_fleet(core_fleet: rc::FleetMut) -> Self {
        Self {
            fleet_id: core_fleet.get_fleet_id(),
        }
    }
}

impl AddedFitIdResp {
    pub(in crate::ctl::core) fn from_core_fit(core_fit: rc::FitMut) -> Self {
        Self {
            fit_id: core_fit.get_fit_id(),
        }
    }
}

impl AddedItemIdsResp {
    pub(in crate::ctl::core) fn from_core_item(core_item: rc::ItemMut) -> Self {
        Self {
            item_id: core_item.get_item_id(),
            autocharge_item_ids: match &core_item {
                rc::ItemMut::Fighter(core_fighter) => AutochargeItemIds::from_core_fighter(core_fighter),
                _ => AutochargeItemIds::NotGenerated,
            },
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_booster(core_booster: rc::BoosterMut) -> Self {
        Self {
            item_id: core_booster.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_character(core_character: rc::CharacterMut) -> Self {
        Self {
            item_id: core_character.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_drone(core_drone: rc::DroneMut) -> Self {
        Self {
            item_id: core_drone.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_fighter(core_fighter: rc::FighterMut) -> Self {
        Self {
            item_id: core_fighter.get_item_id(),
            autocharge_item_ids: AutochargeItemIds::from_core_fighter(&core_fighter),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_fw_effect(core_fw_effect: rc::FwEffectMut) -> Self {
        Self {
            item_id: core_fw_effect.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_implant(core_implant: rc::ImplantMut) -> Self {
        Self {
            item_id: core_implant.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_module(core_module: rc::ModuleMut) -> Self {
        Self {
            item_id: core_module.get_item_id(),
            charge_item_id: core_module.get_charge().map(|core_charge| core_charge.get_item_id()),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_proj_effect(core_proj_effect: rc::ProjEffectMut) -> Self {
        Self {
            item_id: core_proj_effect.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_rig(core_rig: rc::RigMut) -> Self {
        Self {
            item_id: core_rig.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_service(core_service: rc::ServiceMut) -> Self {
        Self {
            item_id: core_service.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_ship(core_ship: rc::ShipMut) -> Self {
        Self {
            item_id: core_ship.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_skill(core_skill: rc::SkillMut) -> Self {
        Self {
            item_id: core_skill.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_stance(core_stance: rc::StanceMut) -> Self {
        Self {
            item_id: core_stance.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_subsystem(core_subsystem: rc::SubsystemMut) -> Self {
        Self {
            item_id: core_subsystem.get_item_id(),
            ..
        }
    }
    pub(in crate::ctl::core) fn from_core_sw_effect(core_sw_effect: rc::SwEffectMut) -> Self {
        Self {
            item_id: core_sw_effect.get_item_id(),
            ..
        }
    }
}

impl ChangedItemIdsResp {
    pub(in crate::ctl::core) fn add_core_charge(&mut self, core_charge: rc::ChargeMut) {
        self.charge_item_id = Some(core_charge.get_item_id());
    }
    pub(in crate::ctl::core) fn add_core_fighter_autocharges(&mut self, core_fighter: &rc::FighterMut) {
        self.autocharge_item_ids = AutochargeItemIds::from_core_fighter(core_fighter);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Autocharge handling
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Some item kinds can house autocharges, which are charges automatically created for needs of an
/// effect of that item, if that effect needs it.
#[derive(Clone, Default)]
pub enum AutochargeItemIds {
    /// Autocharge generation process was not triggered.
    ///
    /// It means that:
    /// - in case item was added: item does not support autocharges altogether;
    /// - in case item was changed: even if item supports autocharges, request to change didn't
    ///   trigger the autocharge generation process (e.g. item type ID change triggers it, ability
    ///   status change does not).
    #[default]
    NotGenerated,
    /// Autocharges were (re)generated, and this is the result.
    Generated(Vec<(EffectId, ItemId)>),
}
impl AutochargeItemIds {
    fn from_core_fighter(core_fighter: &rc::FighterMut) -> Self {
        Self::Generated(
            core_fighter
                .iter_autocharges()
                .map(|core_autocharge| (core_autocharge.get_cont_effect_id(), core_autocharge.get_item_id()))
                .collect(),
        )
    }
    pub fn is_not_generated(&self) -> bool {
        matches!(self, Self::NotGenerated)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{Serialize, SerializeMap, Serializer};

    use super::*;

    impl Serialize for AutochargeItemIds {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                // This still serializes unchanged variant, still have to declare
                // "skip_serializing_if" in containing struct
                Self::NotGenerated => serializer.serialize_unit(),
                Self::Generated(item_ids) => {
                    let mut map = serializer.serialize_map(Some(item_ids.len()))?;
                    for (effect_id, item_id) in item_ids.iter() {
                        map.serialize_entry(effect_id, item_id)?;
                    }
                    map.end()
                }
            }
        }
    }
}
