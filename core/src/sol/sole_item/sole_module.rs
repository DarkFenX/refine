use crate::{
    defs::{AttrVal, EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolItem, SolItemState, SolModule},
        item_info::{SolChargeInfo, SolModuleInfo},
        SolModRack, SolOrdAddMode, SolView, SolarSystem,
    },
    util::{Error, ErrorKind, Result},
};

use super::misc::find_equip_pos;

impl SolarSystem {
    // Public
    pub fn get_module_info(&self, item_id: &SolItemId) -> Result<SolModuleInfo> {
        Ok(self.make_mod_info(self.items.get_module(item_id)?))
    }
    pub fn get_module_infos(&self, fit_id: &SolFitId, rack: SolModRack) -> Result<Vec<SolModuleInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let module_ids = match rack {
            SolModRack::High => &fit.mods_high,
            SolModRack::Mid => &fit.mods_mid,
            SolModRack::Low => &fit.mods_low,
        };
        let module_infos = module_ids
            .iter()
            .map(|v| self.make_mod_info(self.items.get_module(v).unwrap()))
            .collect();
        Ok(module_infos)
    }
    pub fn add_module(
        &mut self,
        fit_id: SolFitId,
        rack: SolModRack,
        pos_mode: SolOrdAddMode,
        a_item_id: EItemId,
        state: SolItemState,
        charge_a_item_id: Option<EItemId>,
    ) -> Result<SolModuleInfo> {
        // Allocate resources early, to make sure if we fail we don't need to roll anything back
        let m_item_id = self.items.alloc_item_id()?;
        let c_item_id = match charge_a_item_id {
            Some(_) => Some(self.items.alloc_item_id()?),
            None => None,
        };
        // Calculate position for the module and make necessary changes to positions of other modules
        let infos = self.get_module_infos(&fit_id, rack)?;
        let pos = match pos_mode {
            SolOrdAddMode::Append => infos.iter().map(|v| v.pos).max().map(|v| 1 + v).unwrap_or(0),
            SolOrdAddMode::Equip => {
                let positions = infos.iter().map(|v| v.pos).collect();
                find_equip_pos(positions)
            }
            SolOrdAddMode::Insert(pos) => {
                for info in infos.iter() {
                    match self.items.get_item_mut(&info.id) {
                        Ok(SolItem::Module(m)) if m.rack == rack && m.pos >= pos => m.pos += 1,
                        _ => (),
                    }
                }
                pos
            }
            SolOrdAddMode::Place(pos, repl) => {
                let mut old_item_id = None;
                for info in infos.iter() {
                    match self.items.get_item(&info.id) {
                        Ok(SolItem::Module(m)) if m.rack == rack && m.pos == pos => {
                            old_item_id = Some(info.id);
                            break;
                        }
                        _ => (),
                    }
                }
                match (old_item_id, repl) {
                    (Some(oid), true) => {
                        self.remove_item(&oid);
                        ()
                    }
                    (Some(oid), false) => return Err(Error::new(ErrorKind::ModuleSlotTaken(rack, pos, oid))),
                    _ => (),
                }
                pos
            }
        };
        // Create and register all necessary items
        let c_info = self.add_charge_with_id_opt(c_item_id, fit_id, charge_a_item_id, m_item_id);
        let module = SolModule::new(&self.src, m_item_id, fit_id, a_item_id, state, rack, pos, c_item_id);
        let m_info = SolModuleInfo::from_mod_and_charge(&module, c_info);
        let m_item = SolItem::Module(module);
        self.add_item(m_item);
        Ok(m_info)
    }
    pub fn set_module_state(&mut self, item_id: &SolItemId, state: SolItemState) -> Result<()> {
        let module = self.items.get_module_mut(item_id)?;
        let old_state = module.state;
        module.state = state;
        if state != old_state {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs.switch_item_state(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                item,
                old_state,
                state,
            );
        };
        Ok(())
    }
    pub fn set_module_charge(&mut self, item_id: &SolItemId, charge_a_item_id: EItemId) -> Result<SolChargeInfo> {
        let c_item_id = self.items.alloc_item_id()?;
        self.remove_module_charge(item_id)?;
        let module = self.items.get_module_mut(item_id)?;
        module.charge_item_id = Some(c_item_id);
        let module = self.items.get_module(item_id)?;
        let c_info = self.add_charge_with_id(c_item_id, module.fit_id, charge_a_item_id, module.id);
        Ok(c_info)
    }
    pub fn remove_module_charge(&mut self, item_id: &SolItemId) -> Result<bool> {
        let module = self.items.get_module(item_id)?;
        let removed = match module.charge_item_id {
            Some(cid) => {
                self.remove_item(&cid)?;
                true
            }
            None => false,
        };
        Ok(removed)
    }
    pub fn add_module_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: SolItemId,
        range: Option<AttrVal>,
    ) -> Result<()> {
        // Execute change command if projection is already defined
        let module = self.items.get_module(item_id)?;
        if module.projs.contains(&projectee_item_id) {
            return self.change_module_proj(item_id, &projectee_item_id, range);
        }
        // Check if item can receive projections
        let projectee_item = self.items.get_item(&projectee_item_id)?;
        if !projectee_item.can_receive_projs() {
            return Err(Error::new(ErrorKind::ItemNotProjectable(projectee_item_id)));
        }
        // Add info to the skeleton
        self.proj_tracker.reg_projectee(*item_id, projectee_item_id);
        let module = self.items.get_module_mut(item_id)?;
        module.projs.add(projectee_item_id, range);
        // Process request in services
        let item = self.items.get_item(item_id).unwrap();
        let projectee_item = self.items.get_item(&projectee_item_id).unwrap();
        self.svcs.add_item_projection(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            &item,
            projectee_item,
            range,
        );
        Ok(())
    }
    pub fn change_module_proj(
        &mut self,
        item_id: &SolItemId,
        projectee_item_id: &SolItemId,
        range: Option<AttrVal>,
    ) -> Result<()> {
        // Check if projection is defined before changing it
        let module = self.items.get_module(item_id)?;
        let old_range = match module.projs.get(projectee_item_id) {
            Some(old_range) => *old_range,
            None => return Err(Error::new(ErrorKind::ProjecteeNotFound(*item_id, *projectee_item_id))),
        };
        // Do nothing if ranges are equal
        if range == old_range {
            return Ok(());
        }
        // Adjust skeleton
        let module = self.items.get_module_mut(item_id).unwrap();
        module.projs.add(*projectee_item_id, range);
        // Process request in services
        let item = self.items.get_item(item_id).unwrap();
        let projectee_item = self.items.get_item(projectee_item_id).unwrap();
        self.svcs.change_item_proj_range(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            &item,
            projectee_item,
            range,
        );
        Ok(())
    }
    pub fn remove_module_proj(&mut self, item_id: &SolItemId, projectee_item_id: &SolItemId) -> Result<()> {
        // Check if projection is defined
        let module = self.items.get_module(item_id)?;
        if !module.projs.contains(projectee_item_id) {
            return Err(Error::new(ErrorKind::ProjecteeNotFound(*item_id, *projectee_item_id)));
        };
        // Process request in services
        let item = self.items.get_item(item_id).unwrap();
        let projectee_item = self.items.get_item(projectee_item_id).unwrap();
        self.svcs.remove_item_projection(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            &item,
            projectee_item,
        );
        // Update the skeleton
        self.proj_tracker.unreg_projectee(item_id, projectee_item_id);
        let module = self.items.get_module_mut(item_id).unwrap();
        module.projs.remove(projectee_item_id);
        Ok(())
    }
    // Non-public
    pub(in crate::sol) fn make_mod_info(&self, module: &SolModule) -> SolModuleInfo {
        let charge_info = match module.charge_item_id {
            Some(cid) => match self.get_charge_info(&cid) {
                Ok(ci) => Some(ci),
                _ => None,
            },
            None => None,
        };
        SolModuleInfo::from_mod_and_charge(module, charge_info)
    }
}
