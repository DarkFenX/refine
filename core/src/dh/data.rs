use crate::defines::ReeInt;

#[derive(Debug)]
pub struct Container<T> {
    pub data: Vec<T>,
    pub failed: u32,
}
impl<T> Container<T> {
    pub fn new(data: Vec<T>, failed: u32) -> Container<T> {
        Container { data, failed }
    }
}

#[derive(Debug)]
pub struct EveType {
    pub id: ReeInt,
    pub group_id: ReeInt,
}
impl EveType {
    pub fn new(id: ReeInt, group_id: ReeInt) -> EveType {
        EveType { id, group_id }
    }
}

#[derive(Debug)]
pub struct EveGroup {
    pub id: ReeInt,
    pub category_id: ReeInt,
}
impl EveGroup {
    pub fn new(id: ReeInt, category_id: ReeInt) -> EveGroup {
        EveGroup { id, category_id }
    }
}

#[derive(Debug)]
pub struct FighterAbil {
    pub id: ReeInt,
    pub target_mode: String,
    pub disallow_hisec: bool,
    pub disallow_lowsec: bool,
}
impl FighterAbil {
    pub fn new<T: Into<String>>(
        id: ReeInt,
        target_mode: T,
        disallow_hisec: bool,
        disallow_lowsec: bool,
    ) -> FighterAbil {
        FighterAbil {
            id,
            target_mode: target_mode.into(),
            disallow_hisec,
            disallow_lowsec,
        }
    }
}
