use crate::{Character, Fit};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn get_character(&mut self) -> Option<Character<'_>> {
        self.core
            .u_data
            .fits
            .get(self.uid)
            .character
            .map(|character_uid| Character::new(self.core, character_uid))
    }
}
