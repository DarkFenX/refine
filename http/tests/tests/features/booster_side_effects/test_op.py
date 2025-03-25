from tests import approx, effect_dogma_to_api


def test_preassign(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = effect_dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str is None
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification - effect is applied, just its strength info is not exposed
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(25)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str is None


def test_premul(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 1.25},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = effect_dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(250)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_prediv(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.pre_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 0.8},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = effect_dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(250)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_prediv_zero(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.pre_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 0},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = effect_dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str is None
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str is None


def test_add(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = effect_dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.add
    assert api_side.str.val == approx(25)
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(225)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.add
    assert api_side.str.val == approx(25)


def test_sub(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_sub,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = effect_dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.add
    assert api_side.str.val == approx(-25)
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(175)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.add
    assert api_side.str.val == approx(-25)


def test_postmul(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 1.25},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = effect_dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(250)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_postdiv(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 0.8},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = effect_dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(250)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_postdiv_zero(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 0},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = effect_dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str is None
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str is None


def test_postperc(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = effect_dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(250)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_postassign(client, consts):
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_booster_id = client.mk_eve_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = effect_dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str is None
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification - effect is applied, just its strength info is not exposed
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(25)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str is None
