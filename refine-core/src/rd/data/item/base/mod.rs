pub(crate) use base::RItemBase;
pub(crate) use cap_consumer::RItemCapConsumer;
pub(crate) use effect::RItemBaseEffectData;
pub(crate) use effect_list_id::RItemEffectListId;
pub(crate) use getters::{RItemChargeLimit, RItemShipLimit, RShipDroneLimit};

mod base;
mod cap_consumer;
mod effect;
mod effect_list_id;
mod getters;
