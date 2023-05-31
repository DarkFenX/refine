pub use calc::AttrVal;
pub use info::{
    BoosterInfo, CharacterInfo, ChargeInfo, DroneInfo, FighterInfo, ImplantInfo, ItemInfo, ModuleInfo, RigInfo,
    ShipInfo, SkillInfo, StanceInfo, SubsystemInfo, SwEffectInfo,
};
pub use ss::SolarSystem;
use ss::SsInnerData;

mod calc;
mod helpers;
mod info;
pub(crate) mod item;
mod notify;
mod ss;
