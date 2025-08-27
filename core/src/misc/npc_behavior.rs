use crate::ud::UNpcProp;

#[derive(Copy, Clone)]
pub enum NpcProp {
    Cruise,
    Chase,
}
impl From<UNpcProp> for NpcProp {
    fn from(u_npc_prop: UNpcProp) -> Self {
        match u_npc_prop {
            UNpcProp::Cruise => Self::Cruise,
            UNpcProp::Chase => Self::Chase,
        }
    }
}
impl From<NpcProp> for UNpcProp {
    fn from(npc_prop: NpcProp) -> Self {
        match npc_prop {
            NpcProp::Cruise => Self::Cruise,
            NpcProp::Chase => Self::Chase,
        }
    }
}
