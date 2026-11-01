from fw import approx, check_no_field
from tests.rollback.kinds import rollback_batch, setup_mod_basics


def test_set(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_stance1_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_stance2_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 50}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_stance = api_fit.set_stance(type_id=eve_stance1_id)
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification - new stance replaces the old one
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.set_stance(fit_id=api_fit.id, type_id=eve_stance2_id)
    assert api_fit.update().stance.id == api_stance.id
    assert api_stance.update().type_id == eve_stance1_id
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_unset(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_stance_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_stance = api_fit.set_stance(type_id=eve_stance_id)
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.unset_stance(fit_id=api_fit.id)
    assert api_fit.update().stance.id == api_stance.id
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_change(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_basic_info.ship_mod])
    eve_stance1_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_stance2_id = client.mk_eve_item(attrs={eve_basic_info.affector_attr_id: 50}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_stance = api_fit.set_stance(type_id=eve_stance1_id)
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_stance_via_fit_id(
            fit_id=api_fit.id,
            type_id=eve_stance2_id,
            state=False,
            effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    api_stance.update()
    assert api_stance.type_id == eve_stance1_id
    assert api_stance.state is True
    with check_no_field():
        api_stance.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
