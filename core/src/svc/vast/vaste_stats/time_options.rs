use crate::{def::AttrVal, misc::Spool};

#[derive(Copy, Clone)]
pub enum StatTimeOptions {
    Burst(StatTimeOptionsBurst),
    Sim(StatTimeOptionsSim),
}

#[derive(Copy, Clone)]
pub struct StatTimeOptionsBurst {
    pub spool: Option<Spool> = None,
}

#[derive(Copy, Clone)]
pub struct StatTimeOptionsSim {
    pub time: Option<AttrVal> = None,
    pub reload_optionals: Option<bool> = None,
    pub refuel_fighters: Option<bool> = None,
}
