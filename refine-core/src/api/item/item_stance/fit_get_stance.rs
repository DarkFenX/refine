use crate::{Fit, Stance};

impl<'s> Fit<'s> {
    pub fn get_stance(&mut self) -> Option<Stance<'_>> {
        self.sol
            .u_data
            .fits
            .get(self.uid)
            .stance
            .map(|stance_uid| Stance::new(self.sol, stance_uid))
    }
}
