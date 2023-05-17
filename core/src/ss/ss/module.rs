use crate::{
    consts::{ModRack, OrdAddMode, State},
    defs::{ReeId, ReeIdx, ReeInt},
    ss::{
        info::{ChargeInfo, ModuleInfo},
        item::{Item, Module},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_module_info(&self, item_id: &ReeId) -> Result<ModuleInfo> {
        Ok(self.make_mod_info(self.get_module(item_id)?))
    }
    pub fn get_high_module_infos(&self, fit_id: &ReeId) -> Vec<ModuleInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::ModuleHigh(m) if m.fit_id == *fit_id => Some(self.make_mod_info(m)),
                _ => None,
            })
            .collect()
    }
    pub fn get_mid_module_infos(&self, fit_id: &ReeId) -> Vec<ModuleInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::ModuleMid(m) if m.fit_id == *fit_id => Some(self.make_mod_info(m)),
                _ => None,
            })
            .collect()
    }
    pub fn get_low_module_infos(&self, fit_id: &ReeId) -> Vec<ModuleInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::ModuleLow(m) if m.fit_id == *fit_id => Some(self.make_mod_info(m)),
                _ => None,
            })
            .collect()
    }
    pub fn add_high_module(
        &mut self,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        add_mode: OrdAddMode,
        charge_type_id: Option<ReeInt>,
    ) -> Result<ModuleInfo> {
        // Allocate resources first
        let m_item_id = self.alloc_item_id()?;
        let c_item_id = match charge_type_id {
            Some(_) => Some(self.alloc_item_id()?),
            None => None,
        };
        // Calculate position for the module and make necessary changes to positions of other modules
        let infos = self.get_high_module_infos(&fit_id);
        let pos = match add_mode {
            OrdAddMode::Append => infos.iter().map(|v| v.pos).max().map(|v| 1 + v).unwrap_or(0),
            OrdAddMode::Equip => {
                let positions = infos.iter().map(|v| v.pos).collect();
                find_equip_pos(positions)
            }
            OrdAddMode::Insert(pos) => {
                for info in infos.iter() {
                    match self.items.get_mut(&info.item_id) {
                        Some(Item::ModuleHigh(m)) if m.pos >= pos => m.pos += 1,
                        _ => (),
                    }
                }
                pos
            }
            OrdAddMode::Place(pos, repl) => {
                let mut old_item_id = None;
                for info in infos.iter() {
                    match self.items.get(&info.item_id) {
                        Some(Item::ModuleHigh(m)) if m.pos == pos => {
                            old_item_id = Some(info.item_id);
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
                    (Some(oid), false) => {
                        return Err(Error::new(
                            ErrorKind::SlotTaken,
                            format!("high slot position {} is taken by item ID {}", pos, oid),
                        ))
                    }
                    _ => (),
                }
                pos
            }
        };
        // Create and register all necessary items
        let c_info = self.add_charge_with_id_opt(c_item_id, fit_id, charge_type_id, m_item_id);
        let module = Module::new(
            &self.src,
            m_item_id,
            fit_id,
            type_id,
            state,
            ModRack::High,
            pos,
            c_item_id,
        );
        let m_info = ModuleInfo::from_mod_and_charge(&module, c_info);
        let m_item = Item::ModuleHigh(module);
        self.add_item(m_item);
        Ok(m_info)
    }
    pub fn add_mid_module(
        &mut self,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        add_mode: OrdAddMode,
        charge_type_id: Option<ReeInt>,
    ) -> Result<ModuleInfo> {
        // Allocate resources first
        let m_item_id = self.alloc_item_id()?;
        let c_item_id = match charge_type_id {
            Some(_) => Some(self.alloc_item_id()?),
            None => None,
        };
        // Calculate position for the module and make necessary changes to positions of other modules
        let infos = self.get_mid_module_infos(&fit_id);
        let pos = match add_mode {
            OrdAddMode::Append => infos.iter().map(|v| v.pos).max().map(|v| 1 + v).unwrap_or(0),
            OrdAddMode::Equip => {
                let positions = infos.iter().map(|v| v.pos).collect();
                find_equip_pos(positions)
            }
            OrdAddMode::Insert(pos) => {
                for info in infos.iter() {
                    match self.items.get_mut(&info.item_id) {
                        Some(Item::ModuleMid(m)) if m.pos >= pos => m.pos += 1,
                        _ => (),
                    }
                }
                pos
            }
            OrdAddMode::Place(pos, repl) => {
                let mut old_item_id = None;
                for info in infos.iter() {
                    match self.items.get(&info.item_id) {
                        Some(Item::ModuleMid(m)) if m.pos == pos => {
                            old_item_id = Some(info.item_id);
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
                    (Some(oid), false) => {
                        return Err(Error::new(
                            ErrorKind::SlotTaken,
                            format!("mid slot position {} is taken by item ID {}", pos, oid),
                        ))
                    }
                    _ => (),
                }
                pos
            }
        };
        // Create and register all necessary items
        let c_info = self.add_charge_with_id_opt(c_item_id, fit_id, charge_type_id, m_item_id);
        let module = Module::new(
            &self.src,
            m_item_id,
            fit_id,
            type_id,
            state,
            ModRack::Mid,
            pos,
            c_item_id,
        );
        let m_info = ModuleInfo::from_mod_and_charge(&module, c_info);
        let m_item = Item::ModuleMid(module);
        self.add_item(m_item);
        Ok(m_info)
    }
    pub fn add_low_module(
        &mut self,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        add_mode: OrdAddMode,
        charge_type_id: Option<ReeInt>,
    ) -> Result<ModuleInfo> {
        // Allocate resources first
        let m_item_id = self.alloc_item_id()?;
        let c_item_id = match charge_type_id {
            Some(_) => Some(self.alloc_item_id()?),
            None => None,
        };
        // Calculate position for the module and make necessary changes to positions of other modules
        let infos = self.get_low_module_infos(&fit_id);
        let pos = match add_mode {
            OrdAddMode::Append => infos.iter().map(|v| v.pos).max().map(|v| 1 + v).unwrap_or(0),
            OrdAddMode::Equip => {
                let positions = infos.iter().map(|v| v.pos).collect();
                find_equip_pos(positions)
            }
            OrdAddMode::Insert(pos) => {
                for info in infos.iter() {
                    match self.items.get_mut(&info.item_id) {
                        Some(Item::ModuleLow(m)) if m.pos >= pos => m.pos += 1,
                        _ => (),
                    }
                }
                pos
            }
            OrdAddMode::Place(pos, repl) => {
                let mut old_item_id = None;
                for info in infos.iter() {
                    match self.items.get(&info.item_id) {
                        Some(Item::ModuleLow(m)) if m.pos == pos => {
                            old_item_id = Some(info.item_id);
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
                    (Some(oid), false) => {
                        return Err(Error::new(
                            ErrorKind::SlotTaken,
                            format!("low slot position {} is taken by item ID {}", pos, oid),
                        ))
                    }
                    _ => (),
                }
                pos
            }
        };
        // Create and register all necessary items
        let c_info = self.add_charge_with_id_opt(c_item_id, fit_id, charge_type_id, m_item_id);
        let module = Module::new(
            &self.src,
            m_item_id,
            fit_id,
            type_id,
            state,
            ModRack::Low,
            pos,
            c_item_id,
        );
        let m_info = ModuleInfo::from_mod_and_charge(&module, c_info);
        let m_item = Item::ModuleLow(module);
        self.add_item(m_item);
        Ok(m_info)
    }
    pub fn set_module_state(&mut self, item_id: &ReeId, state: State) -> Result<()> {
        self.get_module_mut(item_id)?.state = state;
        Ok(())
    }
    pub fn set_module_charge(&mut self, item_id: &ReeId, charge_type_id: ReeInt) -> Result<ChargeInfo> {
        let c_item_id = self.alloc_item_id()?;
        self.remove_module_charge(item_id)?;
        let module = self.get_module(item_id)?;
        let c_info = self.add_charge_with_id(c_item_id, module.fit_id, charge_type_id, module.item_id);
        let module = self.get_module_mut(item_id)?;
        module.charge = Some(c_item_id);
        Ok(c_info)
    }
    pub fn remove_module_charge(&mut self, item_id: &ReeId) -> Result<bool> {
        let module = self.get_module_mut(item_id)?;
        match module.charge {
            Some(cid) => {
                module.charge = None;
                Ok(self.items.remove(&cid).is_some())
            }
            None => Ok(false),
        }
    }
    // Non-public
    fn get_module(&self, item_id: &ReeId) -> Result<&Module> {
        match self.get_item(item_id)? {
            Item::ModuleHigh(m) => Ok(m),
            Item::ModuleMid(m) => Ok(m),
            Item::ModuleLow(m) => Ok(m),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Module::get_name(), item_id),
            )),
        }
    }
    fn get_module_mut(&mut self, item_id: &ReeId) -> Result<&mut Module> {
        match self.get_item_mut(item_id)? {
            Item::ModuleHigh(m) => Ok(m),
            Item::ModuleMid(m) => Ok(m),
            Item::ModuleLow(m) => Ok(m),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Module::get_name(), item_id),
            )),
        }
    }
    pub(in crate::ss) fn make_mod_info(&self, module: &Module) -> ModuleInfo {
        let charge_info = match module.charge {
            Some(cid) => match self.get_charge_info(&cid) {
                Ok(ci) => Some(ci),
                _ => None,
            },
            None => None,
        };
        ModuleInfo::from_mod_and_charge(module, charge_info)
    }
}

// Find first slot not taken by any module
fn find_equip_pos(mut positions: Vec<ReeIdx>) -> ReeIdx {
    for i in 0..positions.len() {
        while (positions[i] < positions.len()) && (positions[i] != i) {
            let j = positions[i];
            if positions[j] == positions[i] {
                break;
            }
            positions.swap(i, j);
        }
    }
    for i in 0..positions.len() {
        if i != positions[i] {
            return i;
        }
    }
    positions.len()
}
