pub(crate) struct ManagedSolSys {
    sol_sys: reefast::SolarSystem,
    accessed: chrono::DateTime<chrono::Utc>,
}
impl ManagedSolSys {
    pub(crate) fn new(sol_sys: reefast::SolarSystem) -> Self {
        Self {
            sol_sys,
            accessed: chrono::Utc::now(),
        }
    }
    pub(crate) fn last_accessed(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.accessed
    }
    fn update_access_ts(&mut self) {
        self.accessed = chrono::Utc::now();
    }
    // Fit methods
    pub fn add_fit(&mut self) -> reefast::Result<reefast::ReeId> {
        let res = self.sol_sys.add_fit();
        self.update_access_ts();
        res
    }
}
