#[derive(Copy, Clone)]
pub struct SolDmgTypes<T> {
    pub em: T,
    pub thermal: T,
    pub kinetic: T,
    pub explosive: T,
}
impl<T> SolDmgTypes<T> {
    pub(crate) fn new(em: T, thermal: T, kinetic: T, explosive: T) -> Self {
        Self {
            em,
            thermal,
            kinetic,
            explosive,
        }
    }
}
