use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemReceiveProjError},
    misc::Spool,
    sol::api::FitMut,
    svc::vast::{StatDmg, StatDmgApplied, StatDmgItemKinds},
    ud::UItemKey,
};

impl<'a> FitMut<'a> {
    pub fn get_stat_dps(&mut self, item_kinds: StatDmgItemKinds, reload: bool, spool: Option<Spool>) -> StatDmg {
        self.sol
            .svc
            .get_stat_fit_dps_raw(&self.sol.u_data, self.key, item_kinds, reload, spool)
    }
    pub fn get_stat_dps_applied(
        &mut self,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, FitStatDmgAppliedError> {
        let projectee_key = self.get_stat_dmg_projectee_key(projectee_item_id)?;
        Ok(self
            .sol
            .svc
            .get_stat_fit_dps_applied(&self.sol.u_data, self.key, item_kinds, reload, spool, projectee_key))
    }
    pub fn get_stat_volley(&mut self, item_kinds: StatDmgItemKinds, spool: Option<Spool>) -> StatDmg {
        self.sol
            .svc
            .get_stat_fit_volley_raw(&self.sol.u_data, self.key, item_kinds, spool)
    }
    pub fn get_stat_volley_applied(
        &mut self,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, FitStatDmgAppliedError> {
        let projectee_key = self.get_stat_dmg_projectee_key(projectee_item_id)?;
        Ok(self
            .sol
            .svc
            .get_stat_fit_volley_applied(&self.sol.u_data, self.key, item_kinds, spool, projectee_key))
    }
    fn get_stat_dmg_projectee_key(&self, projectee_item_id: &ItemId) -> Result<UItemKey, FitStatDmgAppliedError> {
        let projectee_key = self.sol.u_data.items.key_by_id_err(projectee_item_id)?;
        let projectee_u_item = self.sol.u_data.items.get(projectee_key);
        if !projectee_u_item.can_receive_projs() {
            return Err(ItemReceiveProjError {
                item_id: projectee_u_item.get_item_id(),
                item_kind: projectee_u_item.get_name(),
            }
            .into());
        }
        Ok(projectee_key)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitStatDmgAppliedError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
