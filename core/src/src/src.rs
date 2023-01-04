
#[derive(Debug)]
pub struct Src {
    /// Attribute ID.
    pub alias: String,
}
impl Src {
    /// Make a new dogma attribute out of passed data.
    pub fn new(
        alias: String,
    ) -> Src {
        Src {
            alias,
        }
    }
}
