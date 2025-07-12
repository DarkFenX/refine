use crate::{
    def::{AttrVal, FitKey},
    misc::Spool,
    sol::REffs,
    svc::{Svc, SvcCtx, vast::Vast},
    uad::Uad,
};

impl Svc {
    pub(crate) fn get_stat_fit_orr_shield(&mut self, uad: &Uad, fit_key: FitKey, spool: Option<Spool>) -> AttrVal {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_orr_shield(SvcCtx::new(uad, &self.eprojs), &mut self.calc, spool)
    }
    pub(crate) fn get_stat_fit_orr_armor(&mut self, uad: &Uad, fit_key: FitKey, spool: Option<Spool>) -> AttrVal {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_orr_armor(SvcCtx::new(uad, &self.eprojs), &mut self.calc, spool)
    }
    pub(crate) fn get_stat_fit_orr_hull(&mut self, uad: &Uad, fit_key: FitKey, spool: Option<Spool>) -> AttrVal {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_orr_hull(SvcCtx::new(uad, &self.eprojs), &mut self.calc, spool)
    }
    pub(crate) fn get_stat_fit_orr_cap(&mut self, uad: &Uad, fit_key: FitKey, spool: Option<Spool>) -> AttrVal {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_orr_cap(SvcCtx::new(uad, &self.eprojs), &mut self.calc, spool)
    }
    pub(crate) fn get_stat_item_orr_shield(
        &mut self,
        uad: &Uad,
        reffs: &REffs,
        item_key: FitKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Option<AttrVal> {
        Vast::get_stat_item_orr_shield(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            reffs,
            item_key,
            spool,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_item_orr_armor(
        &mut self,
        uad: &Uad,
        reffs: &REffs,
        item_key: FitKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Option<AttrVal> {
        Vast::get_stat_item_orr_armor(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            reffs,
            item_key,
            spool,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_item_orr_hull(
        &mut self,
        uad: &Uad,
        reffs: &REffs,
        item_key: FitKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Option<AttrVal> {
        Vast::get_stat_item_orr_hull(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            reffs,
            item_key,
            spool,
            ignore_state,
        )
    }
    pub(crate) fn get_stat_item_orr_cap(
        &mut self,
        uad: &Uad,
        reffs: &REffs,
        item_key: FitKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Option<AttrVal> {
        Vast::get_stat_item_orr_cap(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            reffs,
            item_key,
            spool,
            ignore_state,
        )
    }
}
