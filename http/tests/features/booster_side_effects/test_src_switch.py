from tests import approx, check_no_field


def test_valid_to_valid_matching(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_chance_attr_id = eve_d1.mk_attr().id
    eve_affector_attr_id = eve_d1.mk_attr().id
    eve_affectee_attr_id = eve_d1.mk_attr().id
    eve_d2.mk_attr(id_=eve_chance_attr_id)
    eve_d2.mk_attr(id_=eve_affector_attr_id)
    eve_d2.mk_attr(id_=eve_affectee_attr_id)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = eve_d1.mk_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod]).id
    eve_d2.mk_effect(id_=eve_effect_id, chance_attr_id=eve_chance_attr_id, mod_info=[eve_mod])
    eve_booster_id = eve_d1.mk_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_effect_id]).id
    eve_d2.mk_item(
        id_=eve_booster_id,
        attrs={eve_chance_attr_id: 0.5, eve_affector_attr_id: 30},
        eff_ids=[eve_effect_id])
    eve_ship_id = eve_d1.mk_ship(attrs={eve_affectee_attr_id: 100}).id
    eve_d2.mk_ship(id_=eve_ship_id, attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(100)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.5)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(30)
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(260)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.5)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(30)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(125)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_valid_to_valid_different(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d1_chance_attr = eve_d1.mk_attr()
    eve_d1_affector_attr = eve_d1.mk_attr()
    eve_d1_affectee_attr = eve_d1.mk_attr()
    eve_d2_chance_attr = eve_d2.mk_attr()
    eve_d2_affector_attr = eve_d2.mk_attr()
    eve_d2_affectee_attr = eve_d2.mk_attr()
    eve_d1_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_d1_affector_attr.id,
        affectee_attr_id=eve_d1_affectee_attr.id)
    eve_d2_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_d2_affector_attr.id,
        affectee_attr_id=eve_d2_affectee_attr.id)
    eve_effect_id = eve_d1.mk_effect(chance_attr_id=eve_d1_chance_attr.id, mod_info=[eve_d1_mod]).id
    eve_d2.mk_effect(id_=eve_effect_id, chance_attr_id=eve_d2_chance_attr.id, mod_info=[eve_d2_mod])
    eve_booster_id = eve_d1.mk_item(
        attrs={eve_d1_chance_attr.id: 0.4, eve_d1_affector_attr.id: 25},
        eff_ids=[eve_effect_id]).id
    eve_d2.mk_item(
        id_=eve_booster_id,
        attrs={eve_d2_chance_attr.id: 0.5, eve_d2_affector_attr.id: 30},
        eff_ids=[eve_effect_id])
    eve_ship_id = eve_d1.mk_ship(attrs={eve_d1_affectee_attr.id: 100}).id
    eve_d2.mk_ship(id_=eve_ship_id)
    eve_module_id = eve_d1.mk_item().id
    eve_d2.mk_item(id_=eve_module_id, attrs={eve_d2_affectee_attr.id: 150})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_d1_affectee_attr.id].extra == approx(100)
    api_module.update()
    with check_no_field():
        api_module.attrs  # pylint: disable=W0104
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_ship.update()
    with check_no_field():
        api_ship.attrs  # pylint: disable=W0104
    assert api_module.update().attrs[eve_d2_affectee_attr.id].extra == approx(150)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.5)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(30)
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True})
    # Verification
    api_ship.update()
    with check_no_field():
        api_ship.attrs  # pylint: disable=W0104
    assert api_module.update().attrs[eve_d2_affectee_attr.id].extra == approx(195)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.5)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(30)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_d1_affectee_attr.id].extra == approx(125)
    api_module.update()
    with check_no_field():
        api_module.attrs  # pylint: disable=W0104
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)


def test_absent_retains_state(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_chance_attr_id = eve_d1.mk_attr().id
    eve_affector_attr_id = eve_d1.mk_attr().id
    eve_affectee_attr_id = eve_d1.mk_attr().id
    eve_d2.mk_attr(id_=eve_chance_attr_id)
    eve_d2.mk_attr(id_=eve_affector_attr_id)
    eve_d2.mk_attr(id_=eve_affectee_attr_id)
    eve_d1_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = eve_d1.mk_effect(chance_attr_id=eve_chance_attr_id, mod_info=[eve_d1_mod]).id
    eve_d2.mk_effect(id_=eve_effect_id)
    eve_booster_id = eve_d1.mk_item(
        attrs={eve_chance_attr_id: 0.4, eve_affector_attr_id: 25},
        eff_ids=[eve_effect_id]).id
    eve_d2.mk_item(id_=eve_booster_id, attrs={eve_chance_attr_id: 0.5, eve_affector_attr_id: 30})
    eve_ship_id = eve_d1.mk_ship(attrs={eve_affectee_attr_id: 100}).id
    eve_d2.mk_ship(id_=eve_ship_id, attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_booster = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(100)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    # Action - attempt to switch state of an effect which is not a side effect
    api_booster.change_booster(
        side_effects={eve_effect_id: False},
        status_code=409,
        json_predicate={
            'code': 'EXC-019',
            'message': f'effect {eve_effect_id} is not a side effect'})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(100)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is False
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_booster.change_booster(side_effects={eve_effect_id: True})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(125)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    # Action - attempt to switch state of an effect which is not a side effect
    api_booster.change_booster(
        side_effects={eve_effect_id: False},
        status_code=409,
        json_predicate={
            'code': 'EXC-019',
            'message': f'effect {eve_effect_id} is not a side effect'})
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(200)
    api_booster.update()
    with check_no_field():
        api_booster.side_effects  # pylint: disable=W0104
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].extra == approx(125)
    api_side = api_booster.update().side_effects[eve_effect_id]
    assert api_side.chance == approx(0.4)
    assert api_side.status is True
    assert api_side.str.op == consts.ApiSideEffectOp.perc
    assert api_side.str.val == approx(25)
