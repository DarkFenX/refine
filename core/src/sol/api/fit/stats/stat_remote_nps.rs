use crate::{
    def::{AttrVal, ItemId},
    sol::api::{FitMut, FitStatAppliedError},
    svc::vast::StatRemoteNpsItemKinds,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_remote_nps(&mut self, item_kinds: StatRemoteNpsItemKinds) -> AttrVal {
        self.sol
            .svc
            .get_stat_fit_remote_nps(&self.sol.u_data, self.key, item_kinds, None)
    }
    pub fn get_stat_remote_nps_applied(
        &mut self,
        item_kinds: StatRemoteNpsItemKinds,
        projectee_item_id: &ItemId,
    ) -> Result<AttrVal, FitStatAppliedError> {
        let projectee_key = self.get_stat_applied_projectee_key(projectee_item_id)?;
        Ok(self
            .sol
            .svc
            .get_stat_fit_remote_nps(&self.sol.u_data, self.key, item_kinds, Some(projectee_key)))
    }
}
