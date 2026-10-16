use crate::rd::RItemBase;

// Charge-container mismatches which depend only on their types. They are used to register charge
// validation failures, and to check if a charge would fail validations without putting it into a
// container
pub(in crate::svc::vast) fn is_charge_group_mismatch(cont_rib: &RItemBase, charge_rib: &RItemBase) -> bool {
    match &cont_rib.charge_limit {
        Some(charge_limit) => !charge_limit.group_ids.contains(&charge_rib.grp_id),
        None => false,
    }
}

pub(in crate::svc::vast) fn is_charge_cont_group_mismatch(cont_rib: &RItemBase, charge_rib: &RItemBase) -> bool {
    match &charge_rib.cont_limit {
        Some(charge_cont_limit) => !charge_cont_limit.group_ids.contains(&cont_rib.grp_id),
        None => false,
    }
}

// Charge size mismatch happens when parent module requires some charge size
pub(in crate::svc::vast) fn is_charge_size_mismatch(cont_rib: &RItemBase, charge_rib: &RItemBase) -> bool {
    cont_rib.charge_size.is_some() && cont_rib.charge_size != charge_rib.charge_size
}

pub(in crate::svc::vast) fn is_charge_volume_mismatch(cont_rib: &RItemBase, charge_rib: &RItemBase) -> bool {
    cont_rib.capacity < charge_rib.volume
}
