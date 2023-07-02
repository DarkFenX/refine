#![feature(test)]
#![feature(core_intrinsics)]

extern crate test;

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

fn setup() {
    setup_logger().unwrap();
}

fn make_attr_mod1() -> rc::ad::AEffectAttrMod {
    rc::ad::AEffectAttrMod {
        afor_attr_id: 333,
        aggr_mode: rc::ec::ModAggrMode::Stack,
        op: rc::ec::ModOp::Add,
        afee_filter: rc::ec::ModAfeeFilter::LocGrp(rc::ec::ModDomain::Item, 33),
        afee_attr_id: 4747,
    }
}
fn make_attr_mod2() -> rc::ad::AEffectAttrMod {
    rc::ad::AEffectAttrMod {
        afor_attr_id: 333,
        aggr_mode: rc::ec::ModAggrMode::Min(22),
        op: rc::ec::ModOp::Sub,
        afee_filter: rc::ec::ModAfeeFilter::Direct(rc::ec::ModDomain::Ship),
        afee_attr_id: 222234324,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{intrinsics::black_box, rc::Rc, sync::Arc};
    use test::Bencher;

    static BENCH_SIZE: usize = 10000000;

    #[bench]
    fn bench_copy(b: &mut Bencher) {
        let obj1 = make_attr_mod1();
        let obj2 = make_attr_mod2();
        b.iter(|| {
            for i in 0..BENCH_SIZE {
                let a = black_box(obj1);
                let c = black_box(obj2);
            }
        })
    }

    #[bench]
    fn bench_arc(b: &mut Bencher) {
        let obj1 = Arc::new(make_attr_mod1());
        let obj2 = Arc::new(make_attr_mod2());
        b.iter(|| {
            for i in 0..BENCH_SIZE {
                obj1.clone();
                obj2.clone();
            }
        })
    }
}

fn main() {
    setup_logger();
}
