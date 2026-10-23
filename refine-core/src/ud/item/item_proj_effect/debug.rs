use crate::{
    dbg::DebugResult,
    ud::{UData, UProjEffect},
};

impl UProjEffect {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self { base, projs } = self;
        base.consistency_check(u_data)?;
        projs.consistency_check(u_data)?;
        // All projections are supposed to be without range on projected effect
        for (_projectee_uid, proj_data) in projs.iter() {
            if proj_data.is_some() {
                return Err(Default::default());
            }
        }
        Ok(())
    }
}
