use crate::{ed::EAbilId, util::Named};

pub struct EAbil {
    pub id: EAbilId,
    pub disallow_hisec: bool,
    pub disallow_lowsec: bool,
}
impl Named for EAbil {
    fn get_name() -> &'static str {
        "EAbil"
    }
}
