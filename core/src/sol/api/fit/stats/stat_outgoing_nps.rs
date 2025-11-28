use crate::{
    def::{AttrVal, ItemId},
    sol::api::{FitMut, FitStatAppliedError},
    svc::vast::StatNeutItemKinds,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_outgoing_nps(&mut self, item_kinds: StatNeutItemKinds) -> AttrVal {
        self.sol
            .svc
            .get_stat_fit_outgoing_nps(&self.sol.u_data, self.key, item_kinds, None)
    }
    pub fn get_stat_outgoing_nps_applied(
        &mut self,
        item_kinds: StatNeutItemKinds,
        projectee_item_id: &ItemId,
    ) -> Result<AttrVal, FitStatAppliedError> {
        let projectee_key = self.get_stat_applied_projectee_key(projectee_item_id)?;
        Ok(self
            .sol
            .svc
            .get_stat_fit_outgoing_nps(&self.sol.u_data, self.key, item_kinds, Some(projectee_key)))
    }
}
