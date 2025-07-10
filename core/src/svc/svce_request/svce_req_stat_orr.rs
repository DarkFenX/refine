use crate::{
    def::{AttrVal, FitKey},
    svc::{Svc, SvcCtx},
    uad::Uad,
};

impl Svc {
    pub(crate) fn get_stat_fit_orr_shield(&mut self, uad: &Uad, fit_key: FitKey) -> AttrVal {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_orr_shield(SvcCtx::new(uad, &self.eprojs), &mut self.calc)
    }
    pub(crate) fn get_stat_fit_orr_armor(&mut self, uad: &Uad, fit_key: FitKey) -> AttrVal {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_orr_armor(SvcCtx::new(uad, &self.eprojs), &mut self.calc)
    }
    pub(crate) fn get_stat_fit_orr_struct(&mut self, uad: &Uad, fit_key: FitKey) -> AttrVal {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_orr_struct(SvcCtx::new(uad, &self.eprojs), &mut self.calc)
    }
    pub(crate) fn get_stat_fit_orr_cap(&mut self, uad: &Uad, fit_key: FitKey) -> AttrVal {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_orr_cap(SvcCtx::new(uad, &self.eprojs), &mut self.calc)
    }
}
