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
    fn touch(&mut self) {
        self.accessed = chrono::Utc::now();
    }
    pub(crate) fn last_accessed(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.accessed
    }
    // Fit methods
    pub(crate) async fn add_fit(&mut self) -> Result<String> {
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
        self.touch();
        match res {
            Ok(fid) => Ok(fid.to_string()),
            Err(e) => Err(e.into()),
        }
    }
    pub(crate) async fn remove_fit(&mut self, fit_id: &str) -> Result<()> {
        let fit_id = match fit_id.parse() {
            Ok(fid) => fid,
            Err(_) => {
                self.touch();
                return Err(Error::new(ErrorKind::IdCastFailed(fit_id.to_string())));
            }
        };
        let mut ss = match self.sol_sys.take() {
            Some(ss) => ss,
            None => return Err(Error::new(ErrorKind::NoCoreSolSys)),
        };
        let (res, ss) = tokio_rayon::spawn_fifo(move || {
            let res = ss.remove_fit(fit_id);
            (res, ss)
        })
        .await;
        self.sol_sys = Some(ss);
        self.touch();
        res.map_err(|e| e.into())
    }
}
