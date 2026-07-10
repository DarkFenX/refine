use crate::{ed::EAbilId, util::LibNamed};

pub struct EAbil {
    pub id: EAbilId,
    pub disallow_hisec: bool,
    pub disallow_lowsec: bool,
}
impl LibNamed for EAbil {
    fn lib_get_name() -> &'static str {
        "EAbil"
    }
}
