from fw import approx, check_no_field
from tests.rollback.kinds import rollback_batch, setup_mod_basics


def setup_charge_effects(*, client, consts, basic_info):
    eve_charge_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[basic_info.tgt_mod])
    eve_module_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_module_effect_id], defeff_id=eve_module_effect_id)
    return eve_charge_effect_id, eve_module_id


def test_add(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_charge_effect_id, eve_module_id = setup_charge_effects(client=client, consts=consts, basic_info=eve_basic_info)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        proj_item_ids=[api_tgt_ship.id])
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, charge_type_id=eve_charge_id)
    with check_no_field():
        api_module.update().charge  # ruff:ignore[useless-expression]
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(100)


def test_remove(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_charge_effect_id, eve_module_id = setup_charge_effects(client=client, consts=consts, basic_info=eve_basic_info)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id,
        proj_item_ids=[api_tgt_ship.id])
    api_charge_id = api_module.charge.id
    # Verification
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, charge_type_id=None)
    assert api_module.update().charge.id == api_charge_id
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_replace(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_charge_effect_id, eve_module_id = setup_charge_effects(client=client, consts=consts, basic_info=eve_basic_info)
    eve_charge1_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
    eve_charge2_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 50},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id,
        proj_item_ids=[api_tgt_ship.id])
    api_charge = api_module.charge
    # Verification
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification - new charge replaces the old one
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_module(item_id=api_module.id, charge_type_id=eve_charge2_id)
    assert api_module.update().charge.id == api_charge.id
    assert api_charge.update().type_id == eve_charge1_id
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_change(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_charge_effect_id, eve_module_id = setup_charge_effects(client=client, consts=consts, basic_info=eve_basic_info)
    eve_charge1_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
    eve_charge2_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 50},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id,
        proj_item_ids=[api_tgt_ship.id])
    api_charge = api_module.charge
    # Verification
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_charge(
            item_id=api_charge.id,
            type_id=eve_charge2_id,
            state=False,
            effect_modes={eve_charge_effect_id: consts.ApiEffMode.force_stop})
    api_charge.update()
    assert api_charge.type_id == eve_charge1_id
    assert api_charge.state is True
    with check_no_field():
        api_charge.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
