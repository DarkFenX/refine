use crate::{
    def::{AttrVal, FitKey},
    misc::{DmgKinds, Spool},
    svc::{Svc, SvcCtx, err::StatItemCheckError, vast::Vast},
    uad::Uad,
};

impl Svc {
    pub(crate) fn get_stat_item_dps(
        &mut self,
        uad: &Uad,
        item_key: FitKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<DmgKinds<AttrVal>, StatItemCheckError> {
        Vast::get_stat_item_dps_checked(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            item_key,
            spool,
            ignore_state,
        )
    }
}
