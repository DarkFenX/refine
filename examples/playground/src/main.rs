#![allow(warnings, unused)]

use std::{path::PathBuf, sync::Arc, thread::sleep, time::Duration};

use itertools::Itertools;

use rc::{ed::EveDataHandler, ModRack, OrdAddMode, SolarSystem, Src, State, VERSION};

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
    let dh = Box::new(rdhe::PhbFileEdh::new("/home/dfx/Desktop/phobos_tq_en-us".into()));
    // Get some data for skills
    let grp_ids = dh
        .get_item_groups()
        .unwrap()
        .data
        .iter()
        .filter(|v| v.category_id == 16)
        .map(|v| v.id)
        .collect_vec();
    let skill_ids = dh
        .get_items()
        .unwrap()
        .data
        .iter()
        .filter(|v| grp_ids.contains(&v.group_id))
        .map(|v| v.id)
        .collect_vec();
    let mut ch = Box::new(rdha::RamJsonAdh::new(
        PathBuf::from("/home/dfx/Workspace/eve/reefast/examples/playground/cache/"),
        "tq".to_string(),
    ));
    let src = Src::new(dh, ch).unwrap();
    let mut sol_sys = SolarSystem::new(src);
    let fit = sol_sys.add_fit().unwrap();
    let ship = sol_sys.set_fit_ship(fit, 11184, true).unwrap();
    // for skill_id in skill_ids.iter() {
    //     sol_sys.add_skill(fit, skill_id.to_owned(), 5, true);
    // }
    // let implant = sol_sys.add_implant(fit, 19687, true);
    println!("{}", sol_sys.get_item_attr(&ship.id, &4).unwrap().dogma);
    let plate = sol_sys
        .add_module(fit, 31906, State::Online, ModRack::Low, OrdAddMode::Equip, None)
        .unwrap();
    println!(
        "{} {}",
        sol_sys.get_item_attr(&plate.id, &796).unwrap().dogma,
        sol_sys.get_item_attr(&ship.id, &4).unwrap().dogma
    );
    let skill = sol_sys.add_skill(fit, 33078, 5, true).unwrap();
    println!(
        "{} {}",
        sol_sys.get_item_attr(&plate.id, &796).unwrap().dogma,
        sol_sys.get_item_attr(&ship.id, &4).unwrap().dogma
    );
    sol_sys.remove_item(&skill.id);
    println!(
        "{} {}",
        sol_sys.get_item_attr(&plate.id, &796).unwrap().dogma,
        sol_sys.get_item_attr(&ship.id, &4).unwrap().dogma
    );
}
