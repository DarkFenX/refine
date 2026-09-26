use super::ctx::SolCtx;
use crate::{SolarSystem, shared::SolBackup};

impl<'r> SolarSystem<'r> {
    /// Methods which execute solar system changes in a threadpool are split into two groups:
    ///
    /// - infallible methods run commands which cannot fail, thus solar system never has to be
    ///   restored;
    /// - fallible methods receive [`SolBackup`], which tells if solar system has to be backed up
    ///   before execution, in order to be restored in case of failure. Backup is not always needed,
    ///   since failing commands often leave solar system in consistent and expected state.
    pub(crate) async fn exec_standard<F, T, E>(&mut self, backup: SolBackup, func: F) -> Result<T, E>
    where
        F: FnOnce(&mut rc::SolarSystem) -> Result<T, E> + Send + 'static,
        T: Send + 'static,
        E: Send + 'static,
    {
        let core_sol_guarded = self.inner.core_sol.clone();
        self.refine
            .tpool
            .exec_standard(move || {
                let mut core_sol = core_sol_guarded.lock();
                let core_sol_backup = match backup {
                    SolBackup::Needed => Some(core_sol.clone()),
                    SolBackup::NotNeeded => None,
                };
                match (func(&mut core_sol), core_sol_backup) {
                    (Ok(ret), _) => Ok(ret),
                    (Err(error), Some(core_sol_backup)) => {
                        *core_sol = core_sol_backup;
                        Err(error)
                    }
                    (Err(error), None) => Err(error),
                }
            })
            .await
    }
    pub(crate) async fn exec_standard_infallible<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce(&mut rc::SolarSystem) -> R + Send + 'static,
        R: Send + 'static,
    {
        let core_sol_guarded = self.inner.core_sol.clone();
        self.refine
            .tpool
            .exec_standard(move || func(&mut core_sol_guarded.lock()))
            .await
    }
    pub(crate) async fn exec_heavy_infallible<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce(&mut rc::SolarSystem) -> R + Send + 'static,
        R: Send + 'static,
    {
        let core_sol_guarded = self.inner.core_sol.clone();
        self.refine
            .tpool
            .exec_heavy(move || func(&mut core_sol_guarded.lock()))
            .await
    }
    /// Executes in current thread.
    pub(crate) fn exec_inplace<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce(&mut rc::SolarSystem) -> R,
    {
        func(&mut self.inner.core_sol.lock())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'r> SolarSystem<'r> {
    pub(crate) fn get_ctx(&self) -> SolCtx {
        SolCtx {
            sol_id: self.get_id(),
            src_alias: self.get_src_alias(),
        }
    }
}
