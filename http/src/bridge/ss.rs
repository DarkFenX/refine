use crate::util::{Error, ErrorKind, Result};

pub(crate) struct SolarSystem {
    sol_sys: Option<reefast::SolarSystem>,
    accessed: chrono::DateTime<chrono::Utc>,
}
impl SolarSystem {
    pub(crate) fn new(sol_sys: reefast::SolarSystem) -> Self {
        Self {
            sol_sys: Some(sol_sys),
            accessed: chrono::Utc::now(),
        }
    }
    pub(crate) fn last_accessed(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.accessed
    }
    // Fit methods
    pub(crate) async fn add_fit(&mut self) -> Result<reefast::ReeId> {
        let mut ss = match self.sol_sys.take() {
            Some(ss) => ss,
            None => return Err(Error::new(ErrorKind::NoCoreSolSys)),
        };
        let (res, ss) = tokio_rayon::spawn_fifo(move || {
            let res = ss.add_fit();
            (res, ss)
        })
        .await;
        self.sol_sys = Some(ss);
        self.accessed = chrono::Utc::now();
        res.map_err(|e| e.into())
    }
}
