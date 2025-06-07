use crate::sol::{api::FitMut, svc::vast::StatSlot};

impl<'a> FitMut<'a> {
    // Modules
    pub fn get_stat_high_slots(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_high_slots(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_mid_slots(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_mid_slots(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_low_slots(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_low_slots(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_turret_slots(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_turret_slots(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_launcher_slots(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_launcher_slots(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    // Rigs
    pub fn get_stat_rig_slots(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_rig_slots(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    // Service
    pub fn get_stat_service_slots(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_service_slots(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    // Subsystems
    pub fn get_stat_subsystem_slots(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_subsystem_slots(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    // Drones
    pub fn get_stat_launched_drones(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_launched_drones(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    // Fighters
    pub fn get_stat_launched_fighters(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_launched_fighters(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_launched_light_fighters(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_launched_light_fighters(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_launched_heavy_fighters(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_launched_heavy_fighters(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_launched_support_fighters(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_launched_support_fighters(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_launched_st_light_fighters(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_launched_st_light_fighters(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_launched_st_heavy_fighters(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_launched_st_heavy_fighters(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
    pub fn get_stat_launched_st_support_fighters(&mut self) -> StatSlot {
        let fit = self.sol.uad.fits.get(self.key);
        self.sol
            .svc
            .vast
            .get_fit_data(&self.key)
            .get_stat_launched_st_support_fighters(&self.sol.uad, &mut self.sol.svc.calc, fit)
    }
}
