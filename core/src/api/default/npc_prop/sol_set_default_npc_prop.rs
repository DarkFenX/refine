use crate::{misc::NpcProp, sol::SolarSystem};

impl SolarSystem {
    pub fn set_default_npc_prop(&mut self, npc_prop: NpcProp) {
        self.u_data.default_npc_prop = npc_prop;
    }
}
