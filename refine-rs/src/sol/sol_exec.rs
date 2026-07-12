use crate::sol::{SolarSystem, SolarSystemInner};

impl<'r> SolarSystem<'r> {
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
