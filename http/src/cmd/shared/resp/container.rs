use serde::Serialize;

use super::resp::HCmdResp;
use crate::util::HExecError;

#[derive(Serialize)]
#[serde(transparent)]
pub(crate) struct HCmdResps {
    data: Vec<HCmdResp>,
}
impl HCmdResps {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
    pub(crate) fn append(&mut self, resp: HCmdResp) {
        self.data.push(resp);
    }
    pub(in crate::cmd) fn get_fleet_id(&self, index: usize) -> Result<rc::FleetId, HExecError> {
        let resp = self.get_resp(index)?;
        match resp {
            HCmdResp::FleetId(resp) => Ok(resp.id),
            _ => Err(HExecError::BackrefCmdNoFleetId(index)),
        }
    }
    pub(in crate::cmd) fn get_fit_id(&self, index: usize) -> Result<rc::FitId, HExecError> {
        let resp = self.get_resp(index)?;
        match resp {
            HCmdResp::FitId(resp) => Ok(resp.id),
            _ => Err(HExecError::BackrefCmdNoFitId(index)),
        }
    }
    pub(in crate::cmd) fn get_item_id(&self, index: usize) -> Result<rc::ItemId, HExecError> {
        let resp = self.get_resp(index)?;
        match resp {
            HCmdResp::ItemIds(resp) => Ok(resp.id),
            _ => Err(HExecError::BackrefCmdNoItemId(index)),
        }
    }
    pub(in crate::cmd) fn get_charge_item_id(&self, index: usize) -> Result<rc::ItemId, HExecError> {
        let resp = self.get_resp(index)?;
        match resp {
            HCmdResp::ItemIds(resp) if let Some(charge_item_id) = resp.charge_id => Ok(charge_item_id),
            _ => Err(HExecError::BackrefCmdNoChargeItemId(index)),
        }
    }
    fn get_resp(&self, index: usize) -> Result<&HCmdResp, HExecError> {
        self.data.get(index).ok_or(HExecError::BackrefCmdNotFound(index))
    }
}
