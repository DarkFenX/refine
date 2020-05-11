use reefast::eve_type::{Attribute, Effect};
use reefast::consts::EveEffectCategory;

fn main() {
    let _attr = Attribute::new(0, Some(5), Some(50.0), false, false);
    let _eff = Effect::new(0,EveEffectCategory::Active, false, false, Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), Some(0));
}
