use super::inner::SolarSystemInner;
use crate::SolarSystem;

impl<'r> SolarSystem<'r> {
    /// Methods which execute solar system changes in a threadpool are split into three groups:
    /// - safe/infallible methods guarantee that solar system will stay in consistent and expected
    ///   state even if underlying operations can produce errors. For example:
    ///   - attempt to remove something that does not exist produces an error, but does not apply
    ///     any changes;
    ///   - attempt to mutate a module makes core library to unregister existing module from
    ///     services before realizing it cannot be mutated (because it already is mutated). In this
    ///     case, this specific operation is still considered safe, because core library has code
    ///     which restores state (register the module in services again).
    ///   Note that for a command to be safe, solar system state does not have to be exactly equal
    ///   to what it was before command execution. Failed operations can increment ID counters, but
    ///   it since it does not affect anything but IDs of newly created entities, it is considered
    ///   safe.
    /// - fallible methods have to back solar system up, and restore its state in case of failure.
    ///   Commands executed by those methods could have rollback code in case of errors, but it is
    ///   too hard to write, and is likely to become a source of bugs. Cloning is easier, and is
    ///   fast enough.
    /// - rollback methods always back system up and restore it, regardless of results.
    pub(crate) async fn exec_standard_safe<F, R>(&mut self, func: F) -> R
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
    pub(crate) async fn exec_standard_fallible<F, T, E>(&mut self, func: F) -> Result<T, E>
    where
        F: FnOnce(&mut rc::SolarSystem) -> Result<T, E> + Send + 'static,
        T: Send + 'static,
        E: Send + 'static,
    {
        let mut core_sol = self.take_core().unwrap();
        let core_sol_backup = core_sol.clone();
        match self
            .refine
            .tpool
            .exec_standard(move || {
                let ret = func(&mut core_sol)?;
                Ok((core_sol, ret))
            })
            .await
        {
            Ok((core_sol, ret)) => {
                self.put_core_back(core_sol);
                Ok(ret)
            }
            Err(error) => {
                self.put_core_back(core_sol_backup);
                Err(error)
            }
        }
    }
    pub(crate) async fn exec_standard_rollback<F, R>(&mut self, func: F) -> R
    where
        F: FnOnce(&mut rc::SolarSystem) -> R + Send + 'static,
        R: Send + 'static,
    {
        // Not actually rolling back, just cloning sol and sending it in, which is effectively the
        // same
        let mut core_sol = self.inner.core_sol.as_ref().unwrap().as_ref().clone();
        self.refine.tpool.exec_standard(move || func(&mut core_sol)).await
    }
    pub(crate) async fn exec_heavy_safe<F, R>(&mut self, func: F) -> R
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
