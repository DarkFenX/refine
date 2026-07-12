#[derive(Copy, Clone)]
pub enum FleetIdBackref {
    Id(rc::FleetId),
    Backref(usize),
}

#[derive(Copy, Clone)]
pub enum FitIdBackref {
    Id(rc::FitId),
    Backref(usize),
}

#[derive(Copy, Clone)]
pub enum ItemIdBackref {
    Id(rc::ItemId),
    BackrefMain(usize),
    BackrefCharge(usize),
}
