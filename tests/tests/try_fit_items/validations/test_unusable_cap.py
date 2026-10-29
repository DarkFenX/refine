from fw.api import ValOptions


def test_state(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module1_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 700},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module3_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 500},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - validation checks even modules which are not active
    api_fit.check_try_fit_items(
        val_options=ValOptions(unusable_cap=True),
        passing=[eve_module2_id, eve_module3_id],
        failing=[eve_module1_id])


def test_multiple_items(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module1_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 1200},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unusable_cap=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])


def test_multiple_effects(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use1_attr_id = client.mk_eve_attr()
    eve_use2_attr_id = client.mk_eve_attr()
    eve_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use1_attr_id)
    eve_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use2_attr_id)
    eve_module1_id = client.mk_eve_module(
        attrs={eve_use1_attr_id: 1000, eve_use2_attr_id: 1200},
        eff_ids=[eve_effect1_id, eve_effect2_id],
        defeff_id=eve_effect1_id)
    eve_module2_id = client.mk_eve_module(
        attrs={eve_use1_attr_id: 1000},
        eff_ids=[eve_effect1_id],
        defeff_id=eve_effect1_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - highest value is used
    api_fit.check_try_fit_items(
        val_options=ValOptions(unusable_cap=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_asb(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_cap_bonus_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cap_need_bonus)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fueled_shield_boosting,
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_module(
        attrs={
            eve_use_attr_id: 1320,
            eve_cycle_time_attr_id: 5000,
            eve_reload_attr_id: 60000,
            eve_capacity_attr_id: 112},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge_id = client.mk_eve_charge(
        grp_id=consts.EveItemGrp.capacitor_booster_charge,
        attrs={eve_volume_attr_id: 12, eve_cap_bonus_attr_id: -100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.disabled, charge_type_id=eve_charge_id)
    # Verification - validation passes even if non-charged cap use is more than ship has, but
    # without charge validation fails
    api_fit.check_try_fit_items(
        val_options=ValOptions(unusable_cap=True),
        passing=[eve_charge_id],
        failing=[eve_module_id])


def test_known_failures(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module1_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 1200},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module3_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 500},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id)
    api_module2 = api_fit.add_module(type_id=eve_module2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unusable_cap=[api_module1.id]),
        passing=[],
        failing=[eve_module3_id, eve_module1_id, eve_module2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(unusable_cap=[api_module2.id]),
        passing=[],
        failing=[eve_module3_id, eve_module1_id, eve_module2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(unusable_cap=[api_module1.id, api_module2.id]),
        passing=[eve_module3_id],
        failing=[eve_module1_id, eve_module2_id])


def test_negative_use(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_module(
        attrs={eve_use_attr_id: -1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(unusable_cap=True), passing=[eve_module_id], failing=[])


def test_modified_use(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -30}, eff_ids=[eve_implant_effect_id])
    eve_use_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(unusable_cap=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(unusable_cap=True), passing=[eve_module_id], failing=[])


def test_modified_max(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_implant_effect_id])
    eve_use_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(unusable_cap=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(unusable_cap=True), passing=[eve_module_id], failing=[])


def test_no_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(unusable_cap=True), passing=[eve_module_id], failing=[])


def test_no_attr_use(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.alloc_attr_id()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - when use attribute doesn't exist, use is assumed to be 0
    api_fit.check_try_fit_items(val_options=ValOptions(unusable_cap=True), passing=[eve_module_id], failing=[])


def test_no_attr_max(client, consts):
    eve_max_attr_id = consts.EveAttr.capacitor_capacity
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module1_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 100},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_module()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - no cap amount attribute is considered as its value of 0
    api_fit.check_try_fit_items(
        val_options=ValOptions(unusable_cap=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_not_loaded_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(unusable_cap=True), passing=[eve_module_id], failing=[])


def test_criterion_item_kind(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_booster_id = client.mk_eve_booster(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge_id = client.mk_eve_charge(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 1000}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_implant_id = client.mk_eve_implant(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_module(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_rig(attrs={eve_use_attr_id: 1000}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_service_id = client.mk_eve_service(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_subsystem_id = client.mk_eve_subsystem(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unusable_cap=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_rig_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_module_id])
