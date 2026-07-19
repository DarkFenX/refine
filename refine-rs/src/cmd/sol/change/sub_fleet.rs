use crate::{
    ChangeSolEnumCmd, FitIdBackref, FleetIdBackref,
    cmd::inner::{ICmdFleetAddFCtxBIds, ICmdFleetChangeFCtxBIds, ICmdFleetRemoveFCtxBIds},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Add
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct SolAddFleetCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdFleetAddFCtxBIds = ICmdFleetAddFCtxBIds { .. },
}
impl SolAddFleetCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_fit_ids(mut self, fit_ids: impl Iterator<Item = FitIdBackref>) -> Self {
        self.inner.fit_ids.clear();
        self.inner.fit_ids.extend(fit_ids);
        self
    }
}
impl From<SolAddFleetCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolAddFleetCmd) -> Self {
        Self::AddFleet(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeFleetCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdFleetChangeFCtxBIds,
}
impl SolChangeFleetCmd {
    pub fn new(fleet_id: FleetIdBackref) -> Self {
        Self {
            inner: ICmdFleetChangeFCtxBIds { fleet_id, .. },
        }
    }
    pub fn with_add_fit_ids(mut self, add_fit_ids: impl Iterator<Item = FitIdBackref>) -> Self {
        self.inner.ictx_cmd.add_fit_ids.clear();
        self.inner.ictx_cmd.add_fit_ids.extend(add_fit_ids);
        self
    }
    pub fn with_rm_fit_ids(mut self, rm_fit_ids: impl Iterator<Item = FitIdBackref>) -> Self {
        self.inner.ictx_cmd.rm_fit_ids.clear();
        self.inner.ictx_cmd.rm_fit_ids.extend(rm_fit_ids);
        self
    }
}
impl From<SolChangeFleetCmd> for ChangeSolEnumCmd {
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
    pub fn new(fleet_id: FleetIdBackref) -> Self {
        Self {
            inner: ICmdFleetRemoveFCtxBIds { fleet_id, .. },
        }
    }
}
impl From<SolRemoveFleetCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolRemoveFleetCmd) -> Self {
        Self::RemoveFleet(sub_cmd)
    }
}
