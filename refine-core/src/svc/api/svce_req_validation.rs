use crate::{
    ad::AItemId,
    rd::RItemBase,
    svc::{
        Svc, SvcCtx,
        vast::{ValOptionsInt, ValOptionsSolInt, ValResultFitFast},
    },
    ud::{UData, UFitId, UItemId},
    val::{DetectedItemKind, ValResultFit, ValResultSol},
};

impl Svc {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Sol
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub(crate) fn validate_sol_fast(&self, options: &ValOptionsInt) -> bool {
        self.vast.validate_sol_fast(options)
    }
    pub(crate) fn validate_sol_verbose(&self, u_data: &UData, options: &ValOptionsSolInt) -> ValResultSol {
        self.vast
            .validate_sol_verbose(&SvcCtx::new(u_data, &self.eff_projs), options)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Fit fast
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub(crate) fn validate_fit_fast(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        options: &ValOptionsInt,
    ) -> ValResultFitFast {
        self.vast
            .validate_fit_fast(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit_uid, options)
    }
    pub(crate) fn validate_fit_pdbw_fast(&mut self, u_data: &UData, fit_uid: UFitId, options: &ValOptionsInt) -> bool {
        self.vast
            .validate_fit_pdbw_fast(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit_uid, options)
    }
    pub(crate) fn validate_charge_static_fast(
        &self,
        u_data: &UData,
        cont_uid: UItemId,
        charge_type_aid: AItemId,
        options: &ValOptionsInt,
    ) -> bool {
        self.vast
            .validate_charge_static_fast(u_data, cont_uid, charge_type_aid, options)
    }
    pub(crate) fn validate_item_static_fast(
        &self,
        u_data: &UData,
        fit_uid: UFitId,
        item_rib: &RItemBase,
        item_kind: DetectedItemKind,
        options: &ValOptionsInt,
    ) -> bool {
        self.vast
            .validate_item_static_fast(u_data, fit_uid, item_rib, item_kind, options)
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Fit verbose
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub(crate) fn validate_fit_verbose(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        options: &ValOptionsInt,
    ) -> ValResultFit {
        self.vast
            .validate_fit_verbose(&SvcCtx::new(u_data, &self.eff_projs), &mut self.calc, fit_uid, options)
    }
    pub(crate) fn validate_fit_pdbw_verbose(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        options: &ValOptionsInt,
        result: &mut ValResultFit,
    ) {
        self.vast.validate_fit_pdbw_verbose(
            &SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            fit_uid,
            options,
            result,
        )
    }
}
