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
                chrono::Local::now().format("[%H:%M:%S%.3f]"),
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
    println!("using {:?}", dh);
    match dh.get_invtypes() {
        Ok(r) => println!("invtypes: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("invtypes failed: {}", e),
    }
    match dh.get_invgroups() {
        Ok(r) => println!("invgroups: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("invgroups failed: {}", e),
    }
    match dh.get_dgmattrs() {
        Ok(r) => println!("dgmattrs: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("dgmattrs failed: {}", e),
    }
    match dh.get_dgmtypeattrs() {
        Ok(r) => println!("dgmtypeattrs: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("dgmtypeattrs failed: {}", e),
    }
    match dh.get_dgmeffects() {
        Ok(r) => println!("dgmeffects: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("dgmeffects failed: {}", e),
    }
    match dh.get_dgmtypeeffects() {
        Ok(r) => println!("dgmtypeeffects: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("dgmtypeeffects failed: {}", e),
    }
    match dh.get_dgmbuffs() {
        Ok(r) => println!("dgmbuffs: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("dgmbuffs failed: {}", e),
    }
    match dh.get_ftrabils() {
        Ok(r) => println!("ftrabils: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("ftrabils failed: {}", e),
    }
    match dh.get_ftrtypeabils() {
        Ok(r) => println!("ftrtypeabils: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("ftrtypeabils failed: {}", e),
    }
    match dh.get_skillreqs() {
        Ok(r) => println!("skillreqs: {} returned, {} failed", r.data.len(), r.failed),
        Err(e) => println!("skillreqs failed: {}", e),
    }
    match dh.get_version() {
        Ok(r) => println!("data version: {}", r),
        Err(e) => println!("version failed: {}", e),
    }
}
