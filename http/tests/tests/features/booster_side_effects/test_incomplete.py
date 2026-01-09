from fw import approx, check_no_field


def test_not_loaded_item(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_chance_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_affector_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(datas=[eve_d1, eve_d2], chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_booster_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_booster_id,
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(datas=[eve_d1, eve_d2], attrs={eve_affectee_attr_id: 200})
    # Create an item which has the effect, just to prevent it from being cleaned up
    eve_other_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_other_id, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(250)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: False})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(200)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_no_chance_attr(client, consts):
    # When chance attribute reference has ID defined, but does not exist in EVE data, it is
    # considered as a regular non-side-effect modification
    eve_chance_attr_id = client.alloc_attr_id()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_booster_id = client.mk_eve_item(attrs={eve_affector_attr_id: 25}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(250)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: False})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(250)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(250)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # noqa: B018


def test_no_str_attr(client, consts):
    # When modification strength attribute does not exist, modification is not actually applied, and
    # its strength is not exposed
    eve_chance_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.alloc_attr_id()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_booster_id = client.mk_eve_item(attrs={eve_chance_attr_id: 0.4}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is False
    assert api_side.str is None
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True})
    # Verification
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is True
    assert api_side.str is None
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: False})
    # Verification
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is False
    assert api_side.str is None
