use super::ctx::SolCtx;
use crate::Sol;

impl<'r> Sol<'r> {
    /// Methods which execute solar system changes in a threadpool are split into two groups:
    ///
    /// - infallible methods run commands which cannot fail, thus solar system never has to be
    ///   restored;
    /// - fallible methods run commands in a transaction, which is committed if they succeed, and
    ///   rolled back if they fail.
    ///
    /// The solar system lock moves into the threadpool job and comes back with its result, so a
    /// command whose future is dropped keeps the solar system locked until its job completes.
    ///
    /// A panic in any of them leaves the solar system poisoned; it is discarded when it is
    /// requested next.
    pub(crate) async fn exec_standard<F, T, E>(&mut self, func: F) -> Result<T, E>
    where
        F: FnOnce(&mut rc::Sol) -> Result<T, E> + Send + 'static,
        T: Send + 'static,
        E: Send + 'static,
    {
        let mut inner = self.take_inner();
        let (inner, result) = self
            .refine
            .tpool
            .exec_standard(move || {
                let result = inner.exec(|core_sol| {
                    let mut tx = core_sol.begin_tx();
                    let result = func(&mut tx);
                    match result {
                        Ok(..) => tx.commit(),
                        Err(..) => tx.rollback(),
                    }
                    result
                });
                (inner, result)
            })
            .await;
        self.put_inner(inner);
        result
    }
    pub(crate) async fn exec_standard_infallible<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce(&mut rc::SolOwned) -> R + Send + 'static,
        R: Send + 'static,
    {
        let mut inner = self.take_inner();
        let (inner, result) = self
            .refine
            .tpool
            .exec_standard(move || {
                let result = inner.exec(func);
                (inner, result)
            })
            .await;
        self.put_inner(inner);
        result
    }
    #[cfg(feature = "dev")]
    pub(crate) async fn exec_heavy_infallible<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce(&mut rc::SolOwned) -> R + Send + 'static,
        R: Send + 'static,
    {
        let mut inner = self.take_inner();
        let (inner, result) = self
            .refine
            .tpool
            .exec_heavy(move || {
                let result = inner.exec(func);
                (inner, result)
            })
            .await;
        self.put_inner(inner);
        result
    }
    /// Executes in current thread.
    pub(crate) fn exec_inplace<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce(&mut rc::SolOwned) -> R,
    {
        self.inner_mut().exec(func)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'r> Sol<'r> {
    pub(crate) fn get_ctx(&self) -> SolCtx {
        SolCtx {
            sol_id: self.get_id(),
            src_alias: self.get_src_alias(),
        }
    }
}
