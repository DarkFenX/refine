from fw import Effect, approx, check_no_field


def test_valid_to_valid_matching(client, consts):
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
        datas=[eve_d1],
        id_=eve_booster_id,
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_effect_id])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_booster_id,
        attrs={eve_chance_attr_id: 0.5, eve_affector_attr_id: 30},
        eff_ids=[eve_effect_id])
    eve_ship_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_ship(datas=[eve_d1], id_=eve_ship_id, attrs={eve_affectee_attr_id: 100})
    client.mk_eve_ship(datas=[eve_d2], id_=eve_ship_id, attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(100)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.5)
    assert api_side.state is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(30)
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(260)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.5)
    assert api_side.state is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(30)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(125)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_valid_to_valid_different(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d1_chance_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_chance_attr_id)
    eve_d1_affector_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_affector_attr_id)
    eve_d1_affectee_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_affectee_attr_id)
    eve_d2_chance_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_chance_attr_id)
    eve_d2_affector_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_affector_attr_id)
    eve_d2_affectee_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_affectee_attr_id)
    eve_d1_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_d1_affector_attr_id,
        affectee_attr_id=eve_d1_affectee_attr_id)
    eve_d2_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_d2_affector_attr_id,
        affectee_attr_id=eve_d2_affectee_attr_id)
    eve_effect_id = client.alloc_effect_id(datas=[eve_d1, eve_d2])
    client.mk_eve_effect(datas=[eve_d1], id_=eve_effect_id, chance_attr_id=eve_d1_chance_attr_id, mod_info=[eve_d1_mod])
    client.mk_eve_effect(datas=[eve_d2], id_=eve_effect_id, chance_attr_id=eve_d2_chance_attr_id, mod_info=[eve_d2_mod])
    eve_booster_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_booster_id,
        attrs={eve_d1_chance_attr_id: 0.4, eve_d1_affector_attr_id: 25},
        eff_ids=[eve_effect_id])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_booster_id,
        attrs={eve_d2_chance_attr_id: 0.5, eve_d2_affector_attr_id: 30},
        eff_ids=[eve_effect_id])
    eve_ship_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_ship(datas=[eve_d1], id_=eve_ship_id, attrs={eve_d1_affectee_attr_id: 100})
    client.mk_eve_ship(datas=[eve_d2], id_=eve_ship_id)
    eve_module_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_module_id)
    client.mk_eve_item(datas=[eve_d2], id_=eve_module_id, attrs={eve_d2_affectee_attr_id: 150})
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_d1_affectee_attr_id].extra == approx(100)
    api_module.update()
    with check_no_field():
        api_module.attrs  # noqa: B018
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_ship.update()
    with check_no_field():
        api_ship.attrs  # noqa: B018
    assert api_module.update().attrs[eve_d2_affectee_attr_id].extra == approx(150)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.5)
    assert api_side.state is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(30)
    # Action
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    api_ship.update()
    with check_no_field():
        api_ship.attrs  # noqa: B018
    assert api_module.update().attrs[eve_d2_affectee_attr_id].extra == approx(195)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.5)
    assert api_side.state is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(30)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_d1_affectee_attr_id].extra == approx(125)
    api_module.update()
    with check_no_field():
        api_module.attrs  # noqa: B018
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_regular_effect_toggle(client, consts):
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
    eve_effect_id = client.alloc_effect_id(datas=[eve_d1, eve_d2])
    client.mk_eve_effect(datas=[eve_d1], id_=eve_effect_id, chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    client.mk_eve_effect(datas=[eve_d2], id_=eve_effect_id, mod_info=[eve_mod])
    eve_booster_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_booster_id,
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_effect_id])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_booster_id,
        attrs={eve_chance_attr_id: 0.5, eve_affector_attr_id: 30},
        eff_ids=[eve_effect_id])
    eve_ship_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_ship(datas=[eve_d1], id_=eve_ship_id, attrs={eve_affectee_attr_id: 100})
    client.mk_eve_ship(datas=[eve_d2], id_=eve_ship_id, attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(100)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(260)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    # Action - attempt to switch state of an effect which is not a side effect
    api_booster.change_booster(side_effects={api_effect_id: False})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(260)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(100)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(260)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    # Action - attempt to switch state of an effect which is not a side effect
    api_booster.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(260)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(125)
    api_side = api_booster.update().side_effects[api_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.state is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_absent_to_valid(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_chance_attr_id = client.mk_eve_attr(datas=[eve_d2])
    eve_affector_attr_id = client.mk_eve_attr(datas=[eve_d2])
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(datas=[eve_d2], chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_booster_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_booster_id)
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_booster_id,
        attrs={eve_chance_attr_id: 0.5, eve_affector_attr_id: 30},
        eff_ids=[eve_effect_id])
    eve_ship_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_ship(datas=[eve_d1], id_=eve_ship_id, attrs={eve_affectee_attr_id: 100})
    client.mk_eve_ship(datas=[eve_d2], id_=eve_ship_id, attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster1 = api_fit.add_booster(type_id=eve_booster_id)
    api_booster2 = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(100)
    api_booster1.update()
    with check_no_field():
        api_booster1.side_effects  # noqa: B018
    api_booster2.update()
    with check_no_field():
        api_booster1.side_effects  # noqa: B018
    # Action
    api_booster1.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(100)
    api_booster1.update()
    with check_no_field():
        api_booster1.side_effects  # noqa: B018
    api_booster2.update()
    with check_no_field():
        api_booster1.side_effects  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(260)
    api_side1 = api_booster1.update().side_effects[api_effect_id]
    assert api_side1.chance == approx(0.5)
    assert api_side1.state is True
    assert api_side1.str.op == consts.ApiSideEffectOp.perc
    assert api_side1.str.val == approx(30)
    api_side2 = api_booster2.update().side_effects[api_effect_id]
    assert api_side2.chance == approx(0.5)
    assert api_side2.state is False
    assert api_side2.str.op == consts.ApiSideEffectOp.perc
    assert api_side2.str.val == approx(30)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(100)
    api_booster1.update()
    with check_no_field():
        api_booster1.side_effects  # noqa: B018
    api_booster2.update()
    with check_no_field():
        api_booster1.side_effects  # noqa: B018
    # Action
    api_booster2.change_booster(side_effects={api_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(100)
    api_booster1.update()
    with check_no_field():
        api_booster1.side_effects  # noqa: B018
    api_booster2.update()
    with check_no_field():
        api_booster1.side_effects  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(338)
    api_side1 = api_booster1.update().side_effects[api_effect_id]
    assert api_side1.chance == approx(0.5)
    assert api_side1.state is True
    assert api_side1.str.op == consts.ApiSideEffectOp.perc
    assert api_side1.str.val == approx(30)
    api_side2 = api_booster2.update().side_effects[api_effect_id]
    assert api_side2.chance == approx(0.5)
    assert api_side2.state is True
    assert api_side2.str.op == consts.ApiSideEffectOp.perc
    assert api_side2.str.val == approx(30)
