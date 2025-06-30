use crate::{
    dbg::DebugResult,
    uad::{Uad, UadProjEffect},
};

impl UadProjEffect {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        self.get_projs().consistency_check(uad)?;
        Ok(())
    }
}
