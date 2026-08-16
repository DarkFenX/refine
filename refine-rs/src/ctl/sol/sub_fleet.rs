use crate::{
    FitIdBr, FleetIdBr, SolCtlCmd,
    ctl::core::{ICmdFleetChangeFCtxBIds, ICmdFleetRemoveFCtxBIds},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeFleetCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdFleetChangeFCtxBIds,
}
impl SolChangeFleetCmd {
    pub fn new(fleet_id: FleetIdBr) -> Self {
        Self {
            inner: ICmdFleetChangeFCtxBIds { fleet_id, .. },
        }
    }
    pub fn with_add_fit_ids(mut self, add_fit_ids: impl Iterator<Item = FitIdBr>) -> Self {
        self.inner.ictx_cmd.add_fit_ids.clear();
        self.inner.ictx_cmd.add_fit_ids.extend(add_fit_ids);
        self
    }
    pub fn with_rm_fit_ids(mut self, rm_fit_ids: impl Iterator<Item = FitIdBr>) -> Self {
        self.inner.ictx_cmd.rm_fit_ids.clear();
        self.inner.ictx_cmd.rm_fit_ids.extend(rm_fit_ids);
        self
    }
}
impl From<SolChangeFleetCmd> for SolCtlCmd {
    fn from(sub_cmd: SolChangeFleetCmd) -> Self {
        Self::ChangeFleet(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Remove
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolRemoveFleetCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdFleetRemoveFCtxBIds,
}
impl SolRemoveFleetCmd {
    pub fn new(fleet_id: FleetIdBr) -> Self {
        Self {
            inner: ICmdFleetRemoveFCtxBIds { fleet_id, .. },
        }
    }
}
impl From<SolRemoveFleetCmd> for SolCtlCmd {
    fn from(sub_cmd: SolRemoveFleetCmd) -> Self {
        Self::RemoveFleet(sub_cmd)
    }
}
