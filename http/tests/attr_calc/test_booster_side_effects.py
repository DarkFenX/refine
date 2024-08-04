from tests import approx


def test_switching(client, consts):
    eve_primary_affector_attr = client.mk_eve_attr()
    eve_primary_affectee_attr = client.mk_eve_attr()
    eve_side1_chance_attr = client.mk_eve_attr()
    eve_side1_affector_attr = client.mk_eve_attr()
    eve_side1_affectee_attr = client.mk_eve_attr()
    eve_side2_chance_attr = client.mk_eve_attr()
    eve_side2_affector_attr = client.mk_eve_attr()
    eve_side2_affectee_attr = client.mk_eve_attr()
    eve_primary_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_primary_affector_attr.id,
        affectee_attr_id=eve_primary_affectee_attr.id)
    eve_primary_effect = client.mk_eve_effect(mod_info=[eve_primary_mod])
    eve_side1_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_side1_affector_attr.id,
        affectee_attr_id=eve_side1_affectee_attr.id)
    eve_side1_effect = client.mk_eve_effect(chance_attr_id=eve_side1_chance_attr.id, mod_info=[eve_side1_mod])
    eve_side2_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_side2_affector_attr.id,
        affectee_attr_id=eve_side2_affectee_attr.id)
    eve_side2_effect = client.mk_eve_effect(chance_attr_id=eve_side2_chance_attr.id, mod_info=[eve_side2_mod])
    eve_booster = client.mk_eve_item(
        attrs={
            eve_primary_affector_attr.id: 20,
            eve_side1_chance_attr.id: 0.4, eve_side1_affector_attr.id: 25,
            eve_side2_chance_attr.id: 0.2, eve_side2_affector_attr.id: 10},
        eff_ids=[eve_primary_effect.id, eve_side1_effect.id, eve_side2_effect.id])
    eve_ship = client.mk_eve_ship(attrs={
        eve_primary_affectee_attr.id: 10, eve_side1_affectee_attr.id: 250, eve_side2_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_booster = api_fit.add_booster(type_id=eve_booster.id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_primary_affectee_attr.id].dogma == approx(12)
    assert api_ship.attrs[eve_side1_affectee_attr.id].dogma == approx(250)
    assert api_ship.attrs[eve_side2_affectee_attr.id].dogma == approx(100)
    # Action
    api_booster.change_booster(side_effects={eve_side1_effect.id: True})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_primary_affectee_attr.id].dogma == approx(12)
    assert api_ship.attrs[eve_side1_affectee_attr.id].dogma == approx(312.5)
    assert api_ship.attrs[eve_side2_affectee_attr.id].dogma == approx(100)
    # Action
    api_booster.change_booster(side_effects={eve_side2_effect.id: True})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_primary_affectee_attr.id].dogma == approx(12)
    assert api_ship.attrs[eve_side1_affectee_attr.id].dogma == approx(312.5)
    assert api_ship.attrs[eve_side2_affectee_attr.id].dogma == approx(110)
    # Action
    api_booster.change_booster(side_effects={eve_side1_effect.id: False})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_primary_affectee_attr.id].dogma == approx(12)
    assert api_ship.attrs[eve_side1_affectee_attr.id].dogma == approx(250)
    assert api_ship.attrs[eve_side2_affectee_attr.id].dogma == approx(110)
    # Action
    api_booster.change_booster(side_effects={eve_side2_effect.id: False})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_primary_affectee_attr.id].dogma == approx(12)
    assert api_ship.attrs[eve_side1_affectee_attr.id].dogma == approx(250)
    assert api_ship.attrs[eve_side2_affectee_attr.id].dogma == approx(100)
