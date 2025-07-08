use crate::ad;

// Specifies where effect should look for charge
#[derive(Copy, Clone)]
pub(crate) enum NEffectCharge {
    // Effect uses charge loaded into item
    Loaded(NEffectChargeDepl),
    // Effect uses charge referenced by an attribute on effect item
    Attr(ad::AAttrId),
}

// Charge depletion mode
#[derive(Copy, Clone)]
pub(crate) enum NEffectChargeDepl {
    // Charge is not depleted if loaded
    None,
    // Each module cycle removes chargeRate attr value count from count of loaded charges
    ChargeRate,
    // Only 1 charge is loaded, and used until it is destroyed
    Crystal,
}
