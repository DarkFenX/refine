use crate::{
    ad::AItemId,
    misc::DetectedItemKind,
    rd::RItemBase,
    svc::{
        Vast,
        vast::{
            ValOptionsInt,
            val::{
                ValKind, ValOptionInt, get_fit_ship_rig_size, get_fit_ship_type_group, get_item_vs_ship_kind_mismatch,
                is_capital_module, is_charge_cont_group_mismatch, is_charge_group_mismatch, is_charge_size_mismatch,
                is_charge_volume_mismatch, is_drone_group_mismatch, is_rig_size_mismatch, is_ship_limit_met,
                is_ship_subcap, is_skill_level_missing, is_slot_index_taken,
            },
        },
    },
    ud::{UData, UFitId, UItemId},
};

impl Vast {
    // Validations which depend only on type of an item, and on fit data which does not change when
    // the item is added. Allows to check if an item fails them without adding it to the fit.
    // Validation passes if there is not enough data to tell
    pub(in crate::svc) fn validate_item_static_fast(
        &self,
        u_data: &UData,
        fit_uid: UFitId,
        item_rib: &RItemBase,
        item_kind: DetectedItemKind,
        options: &ValOptionsInt,
    ) -> bool {
        let fit = u_data.fits.get(fit_uid);
        let fit_data = self.get_fit_data(fit_uid);
        let is_module = matches!(
            item_kind,
            DetectedItemKind::ModuleHigh | DetectedItemKind::ModuleMid | DetectedItemKind::ModuleLow
        );
        // Order of validations should be the same as in the full version
        if let ValOptionInt::Enabled(..) = options.get(ValKind::SkillReqs)
            && (is_module
                || matches!(
                    item_kind,
                    DetectedItemKind::Booster
                        | DetectedItemKind::Charge
                        | DetectedItemKind::Drone
                        | DetectedItemKind::Fighter
                        | DetectedItemKind::Implant
                        | DetectedItemKind::Ship
                        | DetectedItemKind::Skill
                        | DetectedItemKind::Subsystem
                ))
            && item_rib.srqs.iter().any(|(skill_aid, &required_lvl)| {
                is_skill_level_missing(fit.skills.get(skill_aid).map(|v| v.level), required_lvl)
            })
        {
            return false;
        }
        if let ValOptionInt::Enabled(..) = options.get(ValKind::ItemVsShipKind)
            && (is_module || matches!(item_kind, DetectedItemKind::Rig | DetectedItemKind::Service))
            && get_item_vs_ship_kind_mismatch(u_data, fit, item_rib.cat_id).is_some()
        {
            return false;
        }
        if let ValOptionInt::Enabled(..) = options.get(ValKind::ShipLimit)
            && let Some(ship_limit) = &item_rib.ship_limit
            && (is_module
                || matches!(
                    item_kind,
                    DetectedItemKind::Rig
                        | DetectedItemKind::Service
                        | DetectedItemKind::Stance
                        | DetectedItemKind::Subsystem
                ))
        {
            let (ship_type_aid, ship_grp_aid) = get_fit_ship_type_group(u_data, fit);
            if !is_ship_limit_met(ship_limit, ship_type_aid, ship_grp_aid) {
                return false;
            }
        }
        if let ValOptionInt::Enabled(..) = options.get(ValKind::CapitalModule)
            && is_module
            && is_capital_module(item_rib)
            && is_ship_subcap(fit.ship.map(|ship_uid| u_data.items.get(ship_uid).dc_ship().unwrap()))
        {
            return false;
        }
        if let ValOptionInt::Enabled(..) = options.get(ValKind::RigSize)
            && item_kind == DetectedItemKind::Rig
            && is_rig_size_mismatch(item_rib.rig_size, get_fit_ship_rig_size(u_data, fit))
        {
            return false;
        }
        if let ValOptionInt::Enabled(..) = options.get(ValKind::ImplantSlotIndex)
            && item_kind == DetectedItemKind::Implant
            && let Some(slot) = item_rib.implant_slot
            && is_slot_index_taken(&fit_data.slotted_implants, slot)
        {
            return false;
        }
        if let ValOptionInt::Enabled(..) = options.get(ValKind::BoosterSlotIndex)
            && item_kind == DetectedItemKind::Booster
            && let Some(slot) = item_rib.booster_slot
            && is_slot_index_taken(&fit_data.slotted_boosters, slot)
        {
            return false;
        }
        if let ValOptionInt::Enabled(..) = options.get(ValKind::SubsystemSlotIndex)
            && item_kind == DetectedItemKind::Subsystem
            && let Some(slot) = item_rib.subsystem_slot
            && is_slot_index_taken(&fit_data.slotted_subsystems, slot)
        {
            return false;
        }
        if let ValOptionInt::Enabled(..) = options.get(ValKind::DroneGroup)
            && item_kind == DetectedItemKind::Drone
            && is_drone_group_mismatch(&fit_data.drone_group_limit, item_rib)
        {
            return false;
        }
        true
    }
    // Charge validations which depend only on types of a charge and its container. Allows to check
    // if a charge fails them without putting it into the container. Validation passes if there
    // is not enough data to tell.
    pub(in crate::svc) fn validate_charge_static_fast(
        &self,
        u_data: &UData,
        cont_uid: UItemId,
        charge_type_aid: AItemId,
        options: &ValOptionsInt,
    ) -> bool {
        let Some(cont_rib) = u_data.items.get(cont_uid).get_r_item_base() else {
            return true;
        };
        let Some(charge_r_item) = u_data.r_data.get_item_by_aid(&charge_type_aid) else {
            return true;
        };
        let charge_rib = &charge_r_item.base;
        if let ValOptionInt::Enabled(..) = options.get(ValKind::ChargeGroup)
            && is_charge_group_mismatch(cont_rib, charge_rib)
        {
            return false;
        }
        if let ValOptionInt::Enabled(..) = options.get(ValKind::ChargeParentGroup)
            && is_charge_cont_group_mismatch(cont_rib, charge_rib)
        {
            return false;
        }
        if let ValOptionInt::Enabled(..) = options.get(ValKind::ChargeSize)
            && is_charge_size_mismatch(cont_rib, charge_rib)
        {
            return false;
        }
        if let ValOptionInt::Enabled(..) = options.get(ValKind::ChargeVolume)
            && is_charge_volume_mismatch(cont_rib, charge_rib)
        {
            return false;
        }
        true
    }
}
