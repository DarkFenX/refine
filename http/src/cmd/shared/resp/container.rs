use serde::Serialize;

use crate::{
    cmd::shared::{HCmdResp, HFitIdBackref, HFleetIdBackref, HItemIdBackref},
    err::HExecError,
};

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
    pub(in crate::cmd) fn render_fleet_id(&self, fleet_id: HFleetIdBackref) -> Result<rc::FleetId, HExecError> {
        match fleet_id {
            HFleetIdBackref::Id(item_id) => Ok(item_id),
            HFleetIdBackref::Backref(index) => self.get_fleet_id(index),
        }
    }
    pub(in crate::cmd) fn render_fit_id(&self, fit_id: HFitIdBackref) -> Result<rc::FitId, HExecError> {
        match fit_id {
            HFitIdBackref::Id(item_id) => Ok(item_id),
            HFitIdBackref::Backref(index) => self.get_fit_id(index),
        }
    }
    pub(in crate::cmd) fn render_fit_ids(
        &self,
        backref_fit_ids: Vec<HFitIdBackref>,
    ) -> Result<Vec<rc::FitId>, HExecError> {
        let mut fit_ids = Vec::with_capacity(backref_fit_ids.len());
        for backref_fit_id in backref_fit_ids {
            fit_ids.push(self.render_fit_id(backref_fit_id)?);
        }
        Ok(fit_ids)
    }
    pub(in crate::cmd) fn render_item_id(&self, item_id: HItemIdBackref) -> Result<rc::ItemId, HExecError> {
        match item_id {
            HItemIdBackref::Id(item_id) => Ok(item_id),
            HItemIdBackref::BackrefMain(index) => self.get_item_id(index),
            HItemIdBackref::BackrefCharge(index) => self.get_charge_item_id(index),
        }
    }
    pub(in crate::cmd) fn render_item_ids(
        &self,
        backref_item_ids: Vec<HItemIdBackref>,
    ) -> Result<Vec<rc::ItemId>, HExecError> {
        let mut item_ids = Vec::with_capacity(backref_item_ids.len());
        for backref_item_id in backref_item_ids {
            item_ids.push(self.render_item_id(backref_item_id)?);
        }
        Ok(item_ids)
    }
    // Private methods
    fn get_fleet_id(&self, index: usize) -> Result<rc::FleetId, HExecError> {
        let resp = self.get_resp(index)?;
        match resp {
            HCmdResp::CreatedFleetId(resp) => Ok(resp.id),
            _ => Err(HExecError::BackrefCmdNoFleetId(index)),
        }
    }
    fn get_fit_id(&self, index: usize) -> Result<rc::FitId, HExecError> {
        let resp = self.get_resp(index)?;
        match resp {
            HCmdResp::CreatedFitId(resp) => Ok(resp.id),
            _ => Err(HExecError::BackrefCmdNoFitId(index)),
        }
    }
    fn get_item_id(&self, index: usize) -> Result<rc::ItemId, HExecError> {
        let resp = self.get_resp(index)?;
        match resp {
            HCmdResp::CreatedItemIds(resp) => Ok(resp.id),
            _ => Err(HExecError::BackrefCmdNoItemId(index)),
        }
    }
    fn get_charge_item_id(&self, index: usize) -> Result<rc::ItemId, HExecError> {
        let resp = self.get_resp(index)?;
        match resp {
            HCmdResp::CreatedItemIds(resp) if let Some(charge_item_id) = resp.charge_id => Ok(charge_item_id),
            HCmdResp::ChangedItemIds(resp) if let Some(charge_item_id) = resp.charge_id => Ok(charge_item_id),
            _ => Err(HExecError::BackrefCmdNoChargeItemId(index)),
        }
    }
    fn get_resp(&self, index: usize) -> Result<&HCmdResp, HExecError> {
        self.data.get(index).ok_or(HExecError::BackrefCmdNotFound(index))
    }
}
