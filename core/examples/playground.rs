use std::collections::HashMap;
use std::path::PathBuf;

use chrono;

use reefast::consts::{EveEffectCategory, EveModDomain, EveModOperator};
use reefast::dh::{self, Handler};
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

fn print_data<T>(name: &'static str, data: dh::Result<dh::Container<T>>) {
    match data {
        Ok(r) => {
            println!("{}: {} returned, {} failed", name, r.data.len(), r.errors.len());
            for e in r.errors.iter() {
                println!("  error: {}", e)
            }
        }
        Err(e) => println!("{} failed: {}", name, e),
    }
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
    print_data("invtypes", dh.get_invtypes());
    print_data("invgroups", dh.get_invgroups());
    print_data("dgmattrs", dh.get_dgmattrs());
    print_data("dgmtypeattrs", dh.get_dgmtypeattrs());
    print_data("dgmeffects", dh.get_dgmeffects());
    print_data("dgmtypeeffects", dh.get_dgmtypeeffects());
    print_data("dgmmutatypes", dh.get_dgmmutatypes());
    print_data("dgmmutaattrs", dh.get_dgmmutaattrs());
    print_data("dgmbuffs", dh.get_dgmbuffs());
    print_data("ftrabils", dh.get_ftrabils());
    print_data("ftrtypeabils", dh.get_ftrtypeabils());
    print_data("skillreqs", dh.get_skillreqs());
    match dh.get_version() {
        Ok(r) => println!("data version: {}", r),
        Err(e) => println!("version failed: {}", e),
    }
}
