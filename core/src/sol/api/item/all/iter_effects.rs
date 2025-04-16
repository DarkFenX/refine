use crate::{
    err::basic::ItemLoadedError,
    sol::{
        EffectId, EffectInfo, ItemKey, SolarSystem,
        api::{
            Autocharge, AutochargeMut, Booster, BoosterMut, Character, CharacterMut, Charge, ChargeMut, Drone,
            DroneMut, Fighter, FighterMut, FwEffect, FwEffectMut, Implant, ImplantMut, Item, ItemMut, Module,
            ModuleMut, ProjEffect, ProjEffectMut, Rig, RigMut, Service, ServiceMut, Ship, ShipMut, Skill, SkillMut,
            Stance, StanceMut, Subsystem, SubsystemMut, SwEffect, SwEffectMut,
        },
    },
};

impl<'a> Item<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        let item_key = self.get_key();
        iter_effects(self.get_sol(), item_key)
    }
}
impl<'a> ItemMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        let item_key = self.get_key();
        iter_effects(self.get_sol(), item_key)
    }
}
impl<'a> Autocharge<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> AutochargeMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Booster<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> BoosterMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Character<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> CharacterMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Charge<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> ChargeMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Drone<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> DroneMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Fighter<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> FighterMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> FwEffect<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> FwEffectMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Implant<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> ImplantMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Module<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> ModuleMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> ProjEffect<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> ProjEffectMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Rig<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> RigMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Service<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> ServiceMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Ship<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> ShipMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Skill<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> SkillMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Stance<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> StanceMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> Subsystem<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> SubsystemMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> SwEffect<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}
impl<'a> SwEffectMut<'a> {
    pub fn iter_effects(
        &'a self,
    ) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
        iter_effects(self.sol, self.key)
    }
}

fn iter_effects(
    sol: &SolarSystem,
    item_key: ItemKey,
) -> Result<impl ExactSizeIterator<Item = (EffectId, EffectInfo)>, IterItemEffectsError> {
    let item = sol.uad.items.get(item_key);
    let a_effect_ids = match item.get_a_effect_datas() {
        Some(a_effect_datas) => a_effect_datas.keys(),
        None => {
            return Err(ItemLoadedError {
                item_id: sol.uad.items.id_by_key(item_key),
            }
            .into());
        }
    };
    let effect_infos = a_effect_ids.map(move |a_effect_id| {
        let running = sol.svc.is_effect_running(&item_key, a_effect_id);
        let mode = *item.get_effect_modes().get(a_effect_id);
        (a_effect_id.into(), EffectInfo { running, mode })
    });
    Ok(effect_infos)
}

#[derive(thiserror::Error, Debug)]
pub enum IterItemEffectsError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}
