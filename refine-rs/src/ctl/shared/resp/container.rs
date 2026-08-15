use crate::{CtlCmdResp, FitId, FitIdBackref, FleetId, FleetIdBackref, ItemId, ItemIdBackref, err::BackrefRenderError};

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct CtlCmdResps {
    data: Vec<CtlCmdResp>,
}
impl CtlCmdResps {
    pub fn get(&self, index: usize) -> Option<&CtlCmdResp> {
        self.data.get(index)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CtlCmdResps {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
    pub(crate) fn append(&mut self, resp: CtlCmdResp) {
        self.data.push(resp);
    }
    pub(in crate::ctl) fn render_fleet_id(&self, fleet_id: FleetIdBackref) -> Result<FleetId, BackrefRenderError> {
        match fleet_id {
            FleetIdBackref::Id(item_id) => Ok(item_id),
            FleetIdBackref::Backref(index) => self.get_fleet_id(index),
        }
    }
    pub(in crate::ctl) fn render_fit_id(&self, fit_id: FitIdBackref) -> Result<FitId, BackrefRenderError> {
        match fit_id {
            FitIdBackref::Id(item_id) => Ok(item_id),
            FitIdBackref::Backref(index) => self.get_fit_id(index),
        }
    }
    pub(in crate::ctl) fn render_fit_ids(
        &self,
        backref_fit_ids: Vec<FitIdBackref>,
    ) -> Result<Vec<FitId>, BackrefRenderError> {
        let mut fit_ids = Vec::with_capacity(backref_fit_ids.len());
        for backref_fit_id in backref_fit_ids {
            fit_ids.push(self.render_fit_id(backref_fit_id)?);
        }
        Ok(fit_ids)
    }
    pub(in crate::ctl) fn render_item_id(&self, item_id: ItemIdBackref) -> Result<ItemId, BackrefRenderError> {
        match item_id {
            ItemIdBackref::Id(item_id) => Ok(item_id),
            ItemIdBackref::BackrefMain(index) => self.get_item_id(index),
            ItemIdBackref::BackrefCharge(index) => self.get_charge_item_id(index),
        }
    }
    pub(in crate::ctl) fn render_item_ids(
        &self,
        backref_item_ids: Vec<ItemIdBackref>,
    ) -> Result<Vec<ItemId>, BackrefRenderError> {
        let mut item_ids = Vec::with_capacity(backref_item_ids.len());
        for backref_item_id in backref_item_ids {
            item_ids.push(self.render_item_id(backref_item_id)?);
        }
        Ok(item_ids)
    }
    // Private methods
    fn get_fleet_id(&self, index: usize) -> Result<FleetId, BackrefRenderError> {
        let resp = self.get_resp(index)?;
        match resp {
            CtlCmdResp::AddedFleetId(resp) => Ok(resp.fleet_id),
            _ => Err(BackrefRenderError::NoFleetId(index)),
        }
    }
    fn get_fit_id(&self, index: usize) -> Result<FitId, BackrefRenderError> {
        let resp = self.get_resp(index)?;
        match resp {
            CtlCmdResp::AddedFitId(resp) => Ok(resp.fit_id),
            _ => Err(BackrefRenderError::NoFitId(index)),
        }
    }
    fn get_item_id(&self, index: usize) -> Result<ItemId, BackrefRenderError> {
        let resp = self.get_resp(index)?;
        match resp {
            CtlCmdResp::AddedItemIds(resp) => Ok(resp.item_id),
            _ => Err(BackrefRenderError::NoItemId(index)),
        }
    }
    fn get_charge_item_id(&self, index: usize) -> Result<ItemId, BackrefRenderError> {
        let resp = self.get_resp(index)?;
        match resp {
            CtlCmdResp::AddedItemIds(resp) if let Some(charge_item_id) = resp.charge_item_id => Ok(charge_item_id),
            CtlCmdResp::ChangedItemIds(resp) if let Some(charge_item_id) = resp.charge_item_id => Ok(charge_item_id),
            _ => Err(BackrefRenderError::NoChargeItemId(index)),
        }
    }
    fn get_resp(&self, index: usize) -> Result<&CtlCmdResp, BackrefRenderError> {
        self.data.get(index).ok_or(BackrefRenderError::NotFound(index))
    }
}
