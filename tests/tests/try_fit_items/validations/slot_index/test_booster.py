from fw import approx
from fw.api import ValOptions


def test_multiple(client):
    eve_booster1_id = client.mk_eve_booster(slot=1)
    eve_booster2_id = client.mk_eve_booster(slot=2)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(booster_slot_index=True),
        passing=[eve_booster2_id],
        failing=[eve_booster1_id])


def test_different_slots(client):
    eve_booster1_id = client.mk_eve_booster(slot=1)
    eve_booster2_id = client.mk_eve_booster(slot=2)
    eve_booster3_id = client.mk_eve_booster(slot=3)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster1_id)
    api_fit.add_booster(type_id=eve_booster2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(booster_slot_index=True),
        passing=[eve_booster3_id],
        failing=[eve_booster1_id, eve_booster2_id])


def test_known_failures(client):
    eve_booster1_id = client.mk_eve_booster(slot=1)
    eve_booster2_id = client.mk_eve_booster(slot=2)
    eve_booster3_id = client.mk_eve_booster(slot=3)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster1 = api_fit.add_booster(type_id=eve_booster1_id)
    api_fit.add_booster(type_id=eve_booster2_id)
    api_booster3 = api_fit.add_booster(type_id=eve_booster1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(booster_slot_index=[api_booster1.id]),
        passing=[],
        failing=[eve_booster3_id, eve_booster1_id, eve_booster2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(booster_slot_index=[api_booster3.id]),
        passing=[],
        failing=[eve_booster3_id, eve_booster1_id, eve_booster2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(booster_slot_index=[api_booster1.id, api_booster3.id]),
        passing=[eve_booster3_id],
        failing=[eve_booster1_id, eve_booster2_id])


def test_rounding(client):
    eve_booster1_id = client.mk_eve_booster(slot=1.2)
    eve_booster2_id = client.mk_eve_booster(slot=1.4)
    eve_booster3_id = client.mk_eve_booster(slot=1.6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(booster_slot_index=True),
        passing=[eve_booster3_id],
        failing=[eve_booster2_id])


def test_modified_index(client, consts):
    eve_slot_attr_id = consts.EveAttr.boosterness
    eve_booster1_id = client.mk_eve_booster(slot=2)
    eve_booster2_id = client.mk_eve_booster(slot=3)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_slot_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 3}, eff_ids=[eve_mod_effect_id])
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_booster = api_fit.add_booster(type_id=eve_booster1_id)
    # Verification
    assert api_booster.update().attrs[eve_slot_attr_id].modified == approx(2)
    api_fit.check_try_fit_items(
        val_options=ValOptions(booster_slot_index=True),
        passing=[eve_booster2_id],
        failing=[eve_booster1_id])
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification - attribute is modified, but not for purposes of validation
    assert api_booster.update().attrs[eve_slot_attr_id].modified == approx(3)
    api_fit.check_try_fit_items(
        val_options=ValOptions(booster_slot_index=True),
        passing=[eve_booster2_id],
        failing=[eve_booster1_id])


def test_criterion_booster_state(client):
    # Disabled boosters still compete for slots
    eve_booster_id = client.mk_eve_booster(slot=1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster_id, state=False)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(booster_slot_index=True), passing=[], failing=[eve_booster_id])


def test_criterion_item_kind(client, consts):
    # Implants are not checked, since the attribute makes an implant undetectable
    eve_slot_attr_id = consts.EveAttr.boosterness
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster(slot=1)
    eve_charge_id = client.mk_eve_charge(attrs={eve_slot_attr_id: 1})
    eve_drone_id = client.mk_eve_drone(attrs={eve_slot_attr_id: 1})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_slot_attr_id: 1})
    eve_module_id = client.mk_eve_module(attrs={eve_slot_attr_id: 1})
    eve_rig_id = client.mk_eve_rig(attrs={eve_slot_attr_id: 1})
    eve_service_id = client.mk_eve_service(attrs={eve_slot_attr_id: 1})
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_slot_attr_id: 1})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(booster_slot_index=True),
        passing=[
            eve_charge_id,
            eve_drone_id,
            eve_fighter_id,
            eve_module_id,
            eve_rig_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_booster_id])
