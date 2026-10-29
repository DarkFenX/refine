use crate::svc::calc::{
    calce_rah::RahSim,
    misc::{AttrValData, CalcReuse},
    registers::{BuffRegister, DependencyRegister, RawModStorage, RevisionRegister, StandardRegister},
};

#[derive(Clone)]
pub(crate) struct Calc {
    pub(in crate::svc::calc) reuse: CalcReuse,
    pub(in crate::svc::calc) attrs: AttrValData,
    pub(in crate::svc::calc) rmods: RawModStorage,
    pub(in crate::svc::calc) std: StandardRegister,
    pub(in crate::svc::calc) buffs: BuffRegister,
    pub(in crate::svc::calc) deps: DependencyRegister,
    pub(in crate::svc::calc) revs: RevisionRegister,
    pub(in crate::svc::calc) rah: RahSim,
}
impl Calc {
    pub(in crate::svc) fn new() -> Self {
        Self {
            reuse: CalcReuse::default(),
            attrs: AttrValData::new(),
            rmods: RawModStorage::new(),
            std: StandardRegister::new(),
            buffs: BuffRegister::new(),
            deps: DependencyRegister::new(),
            revs: RevisionRegister::new(),
            rah: RahSim::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl Calc {
        pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self {
                attrs,
                rmods,
                std,
                buffs,
                deps,
                revs,
                rah,
                // Scratch containers
                reuse: _,
            } = self;
            attrs.consistency_check(u_data)?;
            rmods.consistency_check(u_data)?;
            std.consistency_check(u_data, rmods)?;
            buffs.consistency_check(u_data, rmods)?;
            deps.consistency_check(u_data)?;
            revs.consistency_check(u_data, rmods)?;
            rah.consistency_check(u_data)?;
            self.rmods_source_consistency_check(u_data)?;
            Ok(())
        }
        // Modifiers registered for an effect have to be the ones the effect generates from current
        // data, otherwise they were generated from some other data
        fn rmods_source_consistency_check(&self, u_data: &UData) -> DebugResult {
            let mut generated_rmods = Vec::new();
            for (espec, rmods) in self.rmods.iter_by_espec() {
                let item = u_data.items.get(espec.item_uid);
                let effect = u_data.r_data.get_effect_by_rid(espec.effect_rid);
                self.generate_mods_for_effect(&mut generated_rmods, u_data, espec.item_uid, item, effect);
                for rmod in rmods {
                    // Modifiers of partially defined buffs depend on attribute values, and are not
                    // generated here
                    if rmod.buff_type_attr_rid.is_some() {
                        continue;
                    }
                    // Not every generated modifier gets registered
                    match generated_rmods.iter().position(|v| v == rmod) {
                        Some(idx) => {
                            generated_rmods.swap_remove(idx);
                        }
                        None => return Err(Default::default()),
                    }
                }
            }
            Ok(())
        }
    }
}
