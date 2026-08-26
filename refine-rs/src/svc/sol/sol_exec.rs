use super::{ctx::SolCtx, inner::SolarSystemInner};
use crate::{SolarSystem, shared::SolBackup};

impl<'r> SolarSystem<'r> {
    /// Methods which execute solar system changes in a threadpool are split into two groups:
    ///
    /// - infallible methods run commands which cannot fail, thus solar system never has to be
    ///   restored;
    ///
    /// - fallible methods receive [`SolBackup`], which tells if solar system has to be backed up
    ///   before execution, in order to be restored in case of failure. Backup is not always needed,
    ///   since failing commands often leave solar system in consistent and expected state.
    pub(crate) async fn exec_standard<F, T, E>(&mut self, backup: SolBackup, func: F) -> Result<T, E>
    where
        F: FnOnce(&mut rc::SolarSystem) -> Result<T, E> + Send + 'static,
        T: Send + 'static,
        E: Send + 'static,
    {
        let mut core_sol = self.take_core().unwrap();
        let (core_sol, result) = self
            .refine
            .tpool
            .exec_standard(move || {
                let core_sol_backup = match backup {
                    SolBackup::Needed => Some(core_sol.clone()),
                    SolBackup::NotNeeded => None,
                };
                match (func(&mut core_sol), core_sol_backup) {
                    (Ok(ret), _) => (core_sol, Ok(ret)),
                    (Err(error), Some(core_sol_backup)) => (core_sol_backup, Err(error)),
                    (Err(error), None) => (core_sol, Err(error)),
                }
            })
            .await;
        self.put_core_back(core_sol);
        result
    }
    pub(crate) async fn exec_standard_infallible<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce(&mut rc::SolarSystem) -> R + Send + 'static,
        R: Send + 'static,
    {
        let mut core_sol = self.take_core().unwrap();
        let (core_sol, result) = self
            .refine
            .tpool
            .exec_standard(move || {
                let result = func(&mut core_sol);
                (core_sol, result)
            })
            .await;
        self.put_core_back(core_sol);
        result
    }
    pub(crate) async fn exec_heavy_infallible<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce(&mut rc::SolarSystem) -> R + Send + 'static,
        R: Send + 'static,
    {
        let mut core_sol = self.take_core().unwrap();
        let (core_sol, result) = self
            .refine
            .tpool
            .exec_heavy(move || {
                let result = func(&mut core_sol);
                (core_sol, result)
            })
            .await;
        self.put_core_back(core_sol);
        result
    }
    /// Executes in current thread.
    pub(crate) fn exec_inplace<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce(&mut rc::SolarSystem) -> R,
    {
        func(self.inner.core_sol.as_mut().unwrap())
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
    fn take_core(&mut self) -> Option<Box<rc::SolarSystem>> {
        self.inner.take_core()
    }
    fn put_core_back(&mut self, core_sol: Box<rc::SolarSystem>) {
        self.inner.put_core_back(core_sol);
    }
}

impl SolarSystemInner {
    fn take_core(&mut self) -> Option<Box<rc::SolarSystem>> {
        self.core_sol.take()
    }
    fn put_core_back(&mut self, core_sol: Box<rc::SolarSystem>) {
        self.core_sol = Some(core_sol);
    }
}
