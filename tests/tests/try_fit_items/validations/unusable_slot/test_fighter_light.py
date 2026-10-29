from fw import approx
from fw.api import ValOptions


def test_fail_single(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[],
        failing=[eve_fighter_id])


def test_one(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_bay)
    api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[eve_fighter_id],
        failing=[])


def test_known_failures(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    eve_fighter2_id = client.mk_eve_fighter()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter1_id, state=consts.ApiMinionState.in_bay)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter1_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=[api_fighter1.id]),
        passing=[],
        failing=[eve_fighter2_id, eve_fighter1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=[api_fighter2.id]),
        passing=[],
        failing=[eve_fighter2_id, eve_fighter1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=[api_fighter1.id, api_fighter2.id]),
        passing=[eve_fighter2_id],
        failing=[eve_fighter1_id])


def test_modified_fighter_type(client, consts):
    # Fighter type is never modified, so the lib just uses unmodified attributes for access to the
    # attr value
    eve_skill_id = client.mk_eve_item()
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_ftr_type_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1}, srqs={eve_skill_id: 1})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 0}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter2_id, state=consts.ApiMinionState.in_bay)
    # Verification
    assert api_fighter.update().attrs[eve_ftr_type_attr_id].modified == approx(1)
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[eve_fighter2_id],
        failing=[eve_fighter1_id])


def test_modified_max(client, consts):
    # Unrealistic scenario, but modification of max count is supported
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(0)
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[],
        failing=[eve_fighter_id])
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(1)
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[eve_fighter_id],
        failing=[])


def test_fractional_fighter_type(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_fighter1_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 0.4})
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 0.6})
    eve_fighter3_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: -0.1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[],
        failing=[eve_fighter1_id, eve_fighter2_id, eve_fighter3_id])


def test_fractional_max(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    eve_ship1_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0.4})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[],
        failing=[eve_fighter_id])
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification - value is rounded up to int
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[eve_fighter_id],
        failing=[])


def test_no_ship(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[],
        failing=[eve_fighter_id])


def test_no_value_max(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[],
        failing=[eve_fighter_id])


def test_no_attr_max(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    eve_max_attr_id = consts.EveAttr.ftr_light_slots
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - when output attr does not exist, it is assumed to be 0
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[],
        failing=[eve_fighter_id])


def test_no_attr_fighter_type(client, consts):
    eve_ftr_type_attr_id = consts.EveAttr.ftr_sq_is_light
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - when fighter type attribute is not available, it doesn't take a slot
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[eve_fighter_id],
        failing=[])


def test_not_loaded_ship(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[],
        failing=[eve_fighter_id])


def test_criterion_fighter_type(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    eve_other_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_fighter1_id = client.mk_eve_fighter()
    eve_fighter2_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 0})
    eve_fighter3_id = client.mk_eve_fighter(attrs={eve_other_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[eve_fighter1_id, eve_fighter2_id, eve_fighter3_id],
        failing=[])


def test_criterion_item_kind(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_light_slots)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster(attrs={eve_ftr_type_attr_id: 1})
    eve_charge_id = client.mk_eve_charge(attrs={eve_ftr_type_attr_id: 1})
    eve_drone_id = client.mk_eve_drone(attrs={eve_ftr_type_attr_id: 1})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_ftr_type_attr_id: 1})
    eve_implant_id = client.mk_eve_implant(attrs={eve_ftr_type_attr_id: 1})
    eve_module_id = client.mk_eve_module(attrs={eve_ftr_type_attr_id: 1})
    eve_rig_id = client.mk_eve_rig(attrs={eve_ftr_type_attr_id: 1})
    eve_service_id = client.mk_eve_service(attrs={eve_ftr_type_attr_id: 1})
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_ftr_type_attr_id: 1})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_light_fighter=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_implant_id,
            eve_module_id,
            eve_rig_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_fighter_id])
