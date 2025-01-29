use crate::{
    defs::{EItemGrpId, SolItemId},
    sol::{svc::vast::SolVastFitData, uad::SolUad},
};

#[derive(Clone)]
pub struct SolChargeGroupValFail {
    pub parent_item_id: SolItemId,
    pub charge_item_id: SolItemId,
    pub charge_group_id: EItemGrpId,
    pub allowed_group_ids: Vec<EItemGrpId>,
}
impl SolChargeGroupValFail {
    fn new(
        parent_item_id: SolItemId,
        charge_item_id: SolItemId,
        charge_group_id: EItemGrpId,
        allowed_group_ids: Vec<EItemGrpId>,
    ) -> Self {
        Self {
            parent_item_id,
            charge_item_id,
            charge_group_id,
            allowed_group_ids,
        }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_charge_group_fast(&mut self, uad: &SolUad) -> bool {
        for (module_item_id, module_data) in self.charge_group.iter_mut() {
            match module_data {
                Some(result) => {
                    if result.is_some() {
                        return false;
                    }
                }
                None => match calculate_fail_data(uad, module_item_id) {
                    Some(fail) => {
                        let _ = module_data.insert(Some(fail));
                        return false;
                    }
                    None => {
                        let _ = module_data.insert(None);
                    }
                },
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_charge_group_verbose(
        &mut self,
        uad: &SolUad,
    ) -> Vec<SolChargeGroupValFail> {
        let mut fails = Vec::new();
        for (module_item_id, module_data) in self.charge_group.iter_mut() {
            match module_data {
                Some(result) => {
                    if let Some(fail) = result {
                        fails.push(fail.clone());
                    }
                }
                None => match calculate_fail_data(uad, module_item_id) {
                    Some(fail) => {
                        let _ = module_data.insert(Some(fail.clone()));
                        fails.push(fail);
                    }
                    None => {
                        let _ = module_data.insert(None);
                    }
                },
            }
        }
        fails
    }
}

fn calculate_fail_data(uad: &SolUad, module_item_id: &SolItemId) -> Option<SolChargeGroupValFail> {
    let module = uad.items.get_item(module_item_id).unwrap().get_module().unwrap();
    let charge_item_id = match module.get_charge_id() {
        Some(charge_item_id) => charge_item_id,
        None => return None,
    };
    let charge_group_id = match uad.items.get_item(&charge_item_id).unwrap().get_group_id() {
        Some(charge_group_id) => charge_group_id,
        None => return None,
    };
    let allowed_group_ids = module
        .get_a_extras()
        .unwrap()
        .charge_limit
        .as_ref()
        .unwrap()
        .group_ids
        .clone();
    match allowed_group_ids.contains(&charge_group_id) {
        true => None,
        false => {
            let fail = SolChargeGroupValFail::new(*module_item_id, charge_item_id, charge_group_id, allowed_group_ids);
            Some(fail)
        }
    }
}
