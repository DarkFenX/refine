use crate::{NpcProp, SolarSystem};

impl SolarSystem {
    pub fn get_default_npc_prop(&self) -> NpcProp {
        self.u_data.default_npc_prop
    }
}
