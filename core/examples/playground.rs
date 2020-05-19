use std::collections::HashMap;
use std::path::PathBuf;

use chrono;

use reefast::consts::{EveEffectCategory, EveModDomain, EveModOperator};
use reefast::dh::Handler;
use reefast::dh_impls::phobos;
use reefast::eve_type::{Attribute, Effect, Item, ItemModifier};

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().unwrap();
    let _attr = Attribute::new(0, Some(5), Some(50.0), false, false);
    let _eff = Effect::new(
        0,
        EveEffectCategory::Active,
        false,
        false,
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        Some(0),
    );
    let _mod = ItemModifier::new(EveModDomain::Ship, 0, EveModOperator::PostPercent, 0);
    let _item = Item::new(1, 2, 3, HashMap::new(), HashMap::new(), None);

    let dh = phobos::Handler::new(PathBuf::from("/home/dfx/Desktop/phobos_tq_en-us"));
    match dh.get_evetypes() {
        Ok(r) => println!("evetypes: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("evetypes failed: {}", e),
    }
    match dh.get_evegroups() {
        Ok(r) => println!("evegroups: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("evegroups failed: {}", e),
    }
}
