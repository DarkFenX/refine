use crate::ProjMut;

impl<'s> ProjMut<'s> {
    pub fn remove(self) {
        self.sol
            .internal_remove_projection(self.projector_uid, self.projectee_uid)
            .unwrap()
    }
}
