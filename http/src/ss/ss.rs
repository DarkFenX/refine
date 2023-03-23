pub(crate) struct ManagedSolSys {
    sol_sys: reefast::SolarSystem,
    accessed: chrono::DateTime<chrono::Utc>,
    locked: bool,
}
impl ManagedSolSys {
    pub(crate) fn new(sol_sys: reefast::SolarSystem) -> Self {
        Self {
            sol_sys,
            accessed: chrono::Utc::now(),
            locked: false,
        }
    }
    pub(crate) fn last_accessed(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.accessed
    }
    pub(crate) fn is_busy(&self) -> bool {
        self.locked
    }
}
