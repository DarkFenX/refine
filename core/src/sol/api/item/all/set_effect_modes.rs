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
    pub fn set_effect_modes(mut self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        let item_key = self.get_key();
        set_effect_modes(self.get_sol_mut(), item_key, modes);
        self
    }
}
impl<'a> AutochargeMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> BoosterMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> CharacterMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> ChargeMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> DroneMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> FighterMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> FwEffectMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> ImplantMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> ModuleMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> ProjEffectMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> RigMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> ServiceMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> ShipMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> SkillMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> StanceMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> SubsystemMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}
impl<'a> SwEffectMut<'a> {
    pub fn set_effect_modes(self, modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        set_effect_modes(self.sol, self.key, modes);
        self
    }
}

fn set_effect_modes(sol: &mut SolarSystem, item_key: ItemKey, modes: impl Iterator<Item = (EffectId, EffectMode)>) {
    let effect_modes = sol.uad.items.get_mut(item_key).get_effect_modes_mut();
    for (effect_id, effect_mode) in modes {
        effect_modes.set(effect_id.into(), effect_mode)
    }
    let uad_item = sol.uad.items.get(item_key);
    sol.svc
        .process_effects(&sol.uad, item_key, uad_item, uad_item.get_a_state());
}
