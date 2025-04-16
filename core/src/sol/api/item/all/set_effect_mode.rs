use crate::{
    EffectId, EffectMode,
    sol::{
        ItemKey, SolarSystem,
        api::{
            AutochargeMut, BoosterMut, CharacterMut, ChargeMut, DroneMut, FighterMut, FwEffectMut, ImplantMut, ItemMut,
            ModuleMut, ProjEffectMut, RigMut, ServiceMut, ShipMut, SkillMut, StanceMut, SubsystemMut, SwEffectMut,
        },
    },
};

impl<'a> ItemMut<'a> {
    pub fn set_effect_mode(mut self, effect_id: &EffectId, mode: EffectMode) -> Self {
        let item_key = self.get_key();
        set_effect_mode(self.get_sol_mut(), item_key, effect_id, mode);
        self
    }
}
impl<'a> AutochargeMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> BoosterMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> CharacterMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> ChargeMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> DroneMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> FighterMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> FwEffectMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> ImplantMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> ModuleMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> ProjEffectMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> RigMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> ServiceMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> ShipMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> SkillMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> StanceMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> SubsystemMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}
impl<'a> SwEffectMut<'a> {
    pub fn set_effect_mode(self, effect_id: &EffectId, mode: EffectMode) -> Self {
        set_effect_mode(self.sol, self.key, effect_id, mode);
        self
    }
}

fn set_effect_mode(sol: &mut SolarSystem, item_key: ItemKey, effect_id: &EffectId, mode: EffectMode) {
    sol.uad
        .items
        .get_mut(item_key)
        .get_effect_modes_mut()
        .set(effect_id.into(), mode);
    let uad_item = sol.uad.items.get(item_key);
    sol.svc
        .process_effects(&sol.uad, item_key, uad_item, uad_item.get_a_state());
}
