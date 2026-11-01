from fw import approx, check_no_field
from tests.rollback.kinds import rollback_batch, setup_mod_basics


def test_add(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_booster_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.add_booster(fit_id=api_fit.id, type_id=eve_booster_id)
    with check_no_field():
        api_fit.update().boosters  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(100)


def test_remove(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_booster_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.remove_item(item_id=api_booster.id)
    assert api_fit.update().boosters.keys() == {api_booster.id}
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_change(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_booster1_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_booster2_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 50}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster1_id)
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_booster(
            item_id=api_booster.id,
            type_id=eve_booster2_id,
            state=False,
            effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    api_booster.update()
    assert api_booster.type_id == eve_booster1_id
    assert api_booster.state is True
    with check_no_field():
        api_booster.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_side_effects(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_chance_attr_id = client.mk_eve_attr()
    eve_side_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_basic_info.ship_mod])
    eve_booster1_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_side_effect_id])
    eve_booster2_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_basic_info.affector_attr_id: 50},
        eff_ids=[eve_side_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster1 = api_fit.add_booster(type_id=eve_booster1_id, side_effects={eve_side_effect_id: True})
    api_booster2 = api_fit.add_booster(type_id=eve_booster2_id)
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_booster(item_id=api_booster1.id, side_effects={eve_side_effect_id: False})
        api_sol_batch.change_booster(item_id=api_booster2.id, side_effects={eve_side_effect_id: True})
    assert api_booster1.update().side_effects[eve_side_effect_id].state is True
    assert api_booster2.update().side_effects[eve_side_effect_id].state is False
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
