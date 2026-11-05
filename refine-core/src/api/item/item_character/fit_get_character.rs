use crate::{Character, Fit};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn get_character(&mut self) -> Option<Character<'_>> {
        self.sol
            .u_data
            .fits
            .get(self.uid)
            .character
            .map(|character_uid| Character::new(self.sol, character_uid))
    }
}
