pub struct Adjustable<T> {
    pub current: T,
    pub max: T,
    pub overridden: bool,
}
