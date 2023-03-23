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
}
