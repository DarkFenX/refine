use crate::{FitId, FleetId, ItemId};

#[derive(Copy, Clone)]
pub enum FleetIdBackref {
    Id(FleetId),
    Backref(usize),
}

#[derive(Copy, Clone)]
pub enum FitIdBackref {
    Id(FitId),
    Backref(usize),
}

#[derive(Copy, Clone)]
pub enum ItemIdBackref {
    Id(ItemId),
    BackrefMain(usize),
    BackrefCharge(usize),
}
