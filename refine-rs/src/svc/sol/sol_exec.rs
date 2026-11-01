use super::ctx::SolCtx;
use crate::SolarSystem;

impl<'r> SolarSystem<'r> {
    /// Methods which execute solar system changes in a threadpool are split into two groups:
    ///
    /// - infallible methods run commands which cannot fail, thus solar system never has to be
    ///   restored;
    /// - fallible methods run commands in a transaction, which is committed if they succeed, and
    ///   rolled back if they fail.
    ///
    /// A panic in any of them poisons the core sol lock; the solar system is discarded when it is
    /// requested next.
    pub(crate) async fn exec_standard<F, T, E>(&mut self, func: F) -> Result<T, E>
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
                let mut tx = match core_sol.begin_tx() {
                    Ok(tx) => tx,
                    Err(rc::err::TxBeginError::InTransaction) => unreachable!("sol shouldn't be mid-transaction"),
                };
                let result = func(&mut tx);
                match result {
                    Ok(..) => tx.commit(),
                    Err(..) => tx.rollback(),
                }
                result
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
    #[cfg(feature = "dev")]
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
