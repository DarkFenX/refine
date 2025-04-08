use crate::{
    err::basic::ItemKindMatchError,
    sol::{ItemKey, SolarSystem, info::ShipInfo},
};

impl SolarSystem {
    pub(in crate::sol) fn get_ship_internal(&self, item_key: ItemKey) -> Result<ShipInfo, ItemKindMatchError> {
        let ship = self.uad.items.get(item_key).get_ship()?;
        Ok(ShipInfo::from_ship(&self.uad, ship))
    }
}
