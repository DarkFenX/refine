pub struct StatTank<T> {
    pub shield: T,
    pub armor: T,
    pub hull: T,
}

pub struct StatTankRegen<T, U> {
    pub shield: U,
    pub armor: T,
    pub hull: T,
}
