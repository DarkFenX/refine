use crate::{
    misc::NpcProp,
    sol::{SolarSystem, UndoStep},
};

impl SolarSystem {
    pub fn set_default_npc_prop(&mut self, npc_prop: NpcProp) {
        let old_npc_prop = std::mem::replace(&mut self.u_data.default_npc_prop, npc_prop);
        if old_npc_prop != npc_prop && self.undo.is_recording() {
            self.undo.record(UndoStep::DefaultNpcProp(old_npc_prop));
        }
    }
}
