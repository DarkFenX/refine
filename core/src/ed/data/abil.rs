use crate::{ed::EAbilId, util::Named};

pub struct EFighterAbil {
    pub id: EAbilId,
    pub disallow_hisec: bool,
    pub disallow_lowsec: bool,
}
impl Named for EFighterAbil {
    fn get_name() -> &'static str {
        "EFighterAbil"
    }
}
