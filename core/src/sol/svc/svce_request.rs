use crate::{
    AttrVal, ad,
    sol::{
        DmgKinds, FitKey, ItemKey,
        err::KeyedItemLoadedError,
        reffs::REffs,
        svc::{
            Svc,
            calc::{CalcAttrVal, ModificationInfo},
            vast::{
                StatLayerHp, StatRes, StatSlot, StatTank, ValOptionsInt, ValOptionsSolInt, ValResultFit, ValResultSol,
                Vast,
            },
        },
        uad::{Uad, fit::UadFit},
    },
};

impl Svc {
    // Attributes and modifiers
    pub(in crate::sol) fn get_item_attr_val_full(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, KeyedItemLoadedError> {
        self.calc.get_item_attr_val_full(uad, &self.eprojs, item_key, a_attr_id)
    }
    pub(in crate::sol) fn iter_item_attr_vals(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (ad::AAttrId, CalcAttrVal)>, KeyedItemLoadedError> {
        self.calc.iter_item_attr_vals(uad, &self.eprojs, item_key)
    }
    pub(in crate::sol) fn iter_item_mods(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (ad::AAttrId, Vec<ModificationInfo>)>, KeyedItemLoadedError> {
        self.calc.iter_item_mods(uad, &self.eprojs, item_key)
    }
    // Validations
    pub(in crate::sol) fn validate_sol_fast(&mut self, uad: &Uad, reffs: &REffs, options: &ValOptionsSolInt) -> bool {
        self.vast
            .validate_sol_fast(uad, &self.eprojs, &mut self.calc, reffs, options)
    }
    pub(in crate::sol) fn validate_sol_verbose(
        &mut self,
        uad: &Uad,
        reffs: &REffs,
        options: &ValOptionsSolInt,
    ) -> ValResultSol {
        self.vast
            .validate_sol_verbose(uad, &self.eprojs, &mut self.calc, reffs, options)
    }
    pub(in crate::sol) fn validate_fit_fast(
        &mut self,
        uad: &Uad,
        reffs: &REffs,
        fit_key: FitKey,
        options: &ValOptionsInt,
    ) -> bool {
        self.vast
            .validate_fit_fast(uad, &self.eprojs, &mut self.calc, reffs, fit_key, options)
    }
    pub(in crate::sol) fn validate_fit_verbose(
        &mut self,
        uad: &Uad,
        reffs: &REffs,
        fit_key: FitKey,
        options: &ValOptionsInt,
    ) -> ValResultFit {
        self.vast
            .validate_fit_verbose(uad, &self.eprojs, &mut self.calc, reffs, fit_key, options)
    }
    // Stats - slots
    pub(in crate::sol) fn get_stat_fit_high_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_high_slots(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_mid_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_mid_slots(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_low_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_low_slots(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_turret_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_turret_slots(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_launcher_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launcher_slots(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_rig_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_rig_slots(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_service_slots(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_service_slots(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_subsystem_slots(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_subsystem_slots(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_launched_drones(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launched_drones(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_launched_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launched_fighters(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_launched_light_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launched_light_fighters(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_launched_heavy_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launched_heavy_fighters(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_launched_support_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launched_support_fighters(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_launched_st_light_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launched_st_light_fighters(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_launched_st_heavy_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launched_st_heavy_fighters(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_launched_st_support_fighters(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatSlot {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_launched_st_support_fighters(uad, &self.eprojs, &mut self.calc, fit)
    }
    // Stats - resources
    pub(in crate::sol) fn get_stat_fit_cpu(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_cpu(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_powergrid(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_powergrid(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_calibration(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_calibration(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_drone_bay_volume(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_drone_bay_volume(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_drone_bandwidth(&mut self, uad: &Uad, fit_key: FitKey, fit: &UadFit) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_drone_bandwidth(uad, &self.eprojs, &mut self.calc, fit)
    }
    pub(in crate::sol) fn get_stat_fit_fighter_bay_volume(
        &mut self,
        uad: &Uad,
        fit_key: FitKey,
        fit: &UadFit,
    ) -> StatRes {
        self.vast
            .get_fit_data(&fit_key)
            .get_stat_fighter_bay_volume(uad, &self.eprojs, &mut self.calc, fit)
    }
    // Stats - tank
    pub(in crate::sol) fn get_stat_item_hp(&mut self, uad: &Uad, item_key: ItemKey) -> Option<StatTank<StatLayerHp>> {
        self.vast.get_stat_item_hp(uad, &self.eprojs, &mut self.calc, item_key)
    }
    pub(in crate::sol) fn get_stat_item_resists(
        &mut self,
        uad: &Uad,
        item_key: ItemKey,
    ) -> Option<StatTank<DmgKinds<AttrVal>>> {
        Vast::get_stat_item_resists(uad, &self.eprojs, &mut self.calc, item_key)
    }
    // Stats - mobility
    pub(in crate::sol) fn get_stat_item_speed(&mut self, uad: &Uad, item_key: ItemKey) -> Option<AttrVal> {
        Vast::get_stat_item_speed(uad, &self.eprojs, &mut self.calc, item_key)
    }
    pub(in crate::sol) fn get_stat_item_agility(&mut self, uad: &Uad, item_key: ItemKey) -> Option<AttrVal> {
        Vast::get_stat_item_agility(uad, &self.eprojs, &mut self.calc, item_key)
    }
    pub(in crate::sol) fn get_stat_item_align_time(&mut self, uad: &Uad, item_key: ItemKey) -> Option<AttrVal> {
        Vast::get_stat_item_align_time(uad, &self.eprojs, &mut self.calc, item_key)
    }
}
