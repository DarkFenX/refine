use std::collections::HashMap;
use std::path::PathBuf;

use chrono;

use reefast::consts::{EveEffectCategory, EveModDomain, EveModOperator};
use reefast::dh::{self, DataHandler};
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

    let dh = phobos::PhobosHandler::new(PathBuf::from("/home/dfx/Desktop/phobos_tq_en-us"));
    println!("using {:?}", dh);
    print_data("items", dh.get_items());
    print_data("item groups", dh.get_item_groups());
    print_data("attributes", dh.get_attrs());
    print_data("item attributes", dh.get_item_attrs());
    print_data("effects", dh.get_effects());
    print_data("item effects", dh.get_item_effects());
    print_data("mutaplasmid item conversions", dh.get_muta_item_convs());
    print_data("mutaplasmid attr modifications", dh.get_muta_attr_mods());
    print_data("buffs", dh.get_buffs());
    print_data("fighter abilities", dh.get_fighter_abils());
    print_data("item fighter abilities", dh.get_item_fighter_abils());
    print_data("item skill requirements", dh.get_item_skill_reqs());
    match dh.get_version() {
        Ok(r) => println!("data version: {}", r),
        Err(e) => println!("version failed: {}", e),
    }
}
