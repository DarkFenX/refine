from tests import Effect, approx


def test_influence(client, consts):
    eve_primary_affector_attr_id = client.mk_eve_attr()
    eve_primary_affectee_attr_id = client.mk_eve_attr()
    eve_side1_chance_attr_id = client.mk_eve_attr()
    eve_side1_affector_attr_id = client.mk_eve_attr()
    eve_side1_affectee_attr_id = client.mk_eve_attr()
    eve_side2_chance_attr_id = client.mk_eve_attr()
    eve_side2_affector_attr_id = client.mk_eve_attr()
    eve_side2_affectee_attr_id = client.mk_eve_attr()
    eve_primary_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_primary_affector_attr_id,
        affectee_attr_id=eve_primary_affectee_attr_id)
    eve_primary_effect_id = client.mk_eve_effect(mod_info=[eve_primary_mod])
    eve_side1_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_side1_affector_attr_id,
        affectee_attr_id=eve_side1_affectee_attr_id)
    eve_side1_effect_id = client.mk_eve_effect(chance_attr_id=eve_side1_chance_attr_id, mod_info=[eve_side1_mod])
    eve_side2_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_side2_affector_attr_id,
        affectee_attr_id=eve_side2_affectee_attr_id)
    eve_side2_effect_id = client.mk_eve_effect(chance_attr_id=eve_side2_chance_attr_id, mod_info=[eve_side2_mod])
    eve_booster_id = client.mk_eve_item(
        attrs={
            eve_primary_affector_attr_id: 20,
            eve_side1_chance_attr_id: 0.4, eve_side1_affector_attr_id: 25,
            eve_side2_chance_attr_id: 0.2, eve_side2_affector_attr_id: 10},
        eff_ids=[eve_primary_effect_id, eve_side1_effect_id, eve_side2_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_primary_affectee_attr_id: 10, eve_side1_affectee_attr_id: 250, eve_side2_affectee_attr_id: 100})
    client.create_sources()
    api_side1_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_side1_effect_id)
    api_side2_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_side2_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_primary_affectee_attr_id].dogma == approx(12)
    assert api_ship.attrs[eve_side1_affectee_attr_id].dogma == approx(250)
    assert api_ship.attrs[eve_side2_affectee_attr_id].dogma == approx(100)
    assert len(api_ship.mods) == 1
    api_booster.update()
    api_side1 = api_booster.side_effects[api_side1_effect_id]
    assert api_side1.chance == approx(0.4)
    assert api_side1.state is False
    assert api_side1.str.op == consts.ApiSideEffectOp.perc
    assert api_side1.str.val == approx(25)
    api_side2 = api_booster.side_effects[api_side2_effect_id]
    assert api_side2.chance == approx(0.2)
    assert api_side2.state is False
    assert api_side2.str.op == consts.ApiSideEffectOp.perc
    assert api_side2.str.val == approx(10)
    # Action
    api_booster.change_booster(side_effects={api_side1_effect_id: True})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_primary_affectee_attr_id].dogma == approx(12)
    assert api_ship.attrs[eve_side1_affectee_attr_id].dogma == approx(312.5)
    assert api_ship.attrs[eve_side2_affectee_attr_id].dogma == approx(100)
    assert len(api_ship.mods) == 2
    api_mod1 = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_side1_affectee_attr_id,
        affector_item_id=api_booster.id).one()
    assert api_mod1.op == consts.ApiModOp.post_percent
    assert api_mod1.initial_val == approx(25)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(25)
    api_booster.update()
    api_side1 = api_booster.side_effects[api_side1_effect_id]
    assert api_side1.chance == approx(0.4)
    assert api_side1.state is True
    assert api_side1.str.op == consts.ApiSideEffectOp.perc
    assert api_side1.str.val == approx(25)
    api_side2 = api_booster.side_effects[api_side2_effect_id]
    assert api_side2.chance == approx(0.2)
    assert api_side2.state is False
    assert api_side2.str.op == consts.ApiSideEffectOp.perc
    assert api_side2.str.val == approx(10)
    # Action
    api_booster.change_booster(side_effects={api_side2_effect_id: True})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_primary_affectee_attr_id].dogma == approx(12)
    assert api_ship.attrs[eve_side1_affectee_attr_id].dogma == approx(312.5)
    assert api_ship.attrs[eve_side2_affectee_attr_id].dogma == approx(110)
    assert len(api_ship.mods) == 3
    api_mod1 = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_side1_affectee_attr_id,
        affector_item_id=api_booster.id).one()
    assert api_mod1.op == consts.ApiModOp.post_percent
    assert api_mod1.initial_val == approx(25)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(25)
    api_mod2 = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_side2_affectee_attr_id,
        affector_item_id=api_booster.id).one()
    assert api_mod2.op == consts.ApiModOp.post_percent
    assert api_mod2.initial_val == approx(10)
    assert api_mod2.stacking_mult is None
    assert api_mod2.applied_val == approx(10)
    api_booster.update()
    api_side1 = api_booster.side_effects[api_side1_effect_id]
    assert api_side1.chance == approx(0.4)
    assert api_side1.state is True
    assert api_side1.str.op == consts.ApiSideEffectOp.perc
    assert api_side1.str.val == approx(25)
    api_side2 = api_booster.side_effects[api_side2_effect_id]
    assert api_side2.chance == approx(0.2)
    assert api_side2.state is True
    assert api_side2.str.op == consts.ApiSideEffectOp.perc
    assert api_side2.str.val == approx(10)
    # Action
    api_booster.change_booster(side_effects={api_side1_effect_id: False})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_primary_affectee_attr_id].dogma == approx(12)
    assert api_ship.attrs[eve_side1_affectee_attr_id].dogma == approx(250)
    assert api_ship.attrs[eve_side2_affectee_attr_id].dogma == approx(110)
    assert len(api_ship.mods) == 2
    api_mod2 = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_side2_affectee_attr_id,
        affector_item_id=api_booster.id).one()
    assert api_mod2.op == consts.ApiModOp.post_percent
    assert api_mod2.initial_val == approx(10)
    assert api_mod2.stacking_mult is None
    assert api_mod2.applied_val == approx(10)
    api_booster.update()
    api_side1 = api_booster.side_effects[api_side1_effect_id]
    assert api_side1.chance == approx(0.4)
    assert api_side1.state is False
    assert api_side1.str.op == consts.ApiSideEffectOp.perc
    assert api_side1.str.val == approx(25)
    api_side2 = api_booster.side_effects[api_side2_effect_id]
    assert api_side2.chance == approx(0.2)
    assert api_side2.state is True
    assert api_side2.str.op == consts.ApiSideEffectOp.perc
    assert api_side2.str.val == approx(10)
    # Action
    api_booster.change_booster(side_effects={api_side2_effect_id: False})
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_primary_affectee_attr_id].dogma == approx(12)
    assert api_ship.attrs[eve_side1_affectee_attr_id].dogma == approx(250)
    assert api_ship.attrs[eve_side2_affectee_attr_id].dogma == approx(100)
    assert len(api_ship.mods) == 1
    api_booster.update()
    api_side1 = api_booster.side_effects[api_side1_effect_id]
    assert api_side1.chance == approx(0.4)
    assert api_side1.state is False
    assert api_side1.str.op == consts.ApiSideEffectOp.perc
    assert api_side1.str.val == approx(25)
    api_side2 = api_booster.side_effects[api_side2_effect_id]
    assert api_side2.chance == approx(0.2)
    assert api_side2.state is False
    assert api_side2.str.op == consts.ApiSideEffectOp.perc
    assert api_side2.str.val == approx(10)


def test_booster_state(client, consts):
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
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id, side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(250)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_booster.change_booster(state=False)
    # Verification
    # Side effect modification is disabled, because parent booster is disabled
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    # Side effect status is reported regardless of if effect is running or not
    assert api_side.state is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
