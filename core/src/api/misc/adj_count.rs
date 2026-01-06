pub struct AdjustableCount<T> {
    pub current: T,
    pub max: T,
    pub overridden: bool,
}
