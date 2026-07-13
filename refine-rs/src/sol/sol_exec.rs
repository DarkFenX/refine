use crate::sol::{SolarSystem, SolarSystemInner};

impl<'r> SolarSystem<'r> {
    /// Methods which execute solar system changes are split into two groups:
    /// - fallible methods have to back solar system up, and restore its state in case of failure.
    ///   Commands executed by those methods could have rollback code in case of errors, but it is
    ///   too hard to write, and is likely to become a source of bugs. Cloning is easier, and is
    ///   fast enough.
    /// - infallible/safe methods guarantee that solar system will stay in consistent and expected
    ///   state even if underlying operations can produce errors (e.g. there is rollback code in
    ///   core library methods).
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
