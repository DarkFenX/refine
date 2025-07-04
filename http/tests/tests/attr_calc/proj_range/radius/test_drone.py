from tests import approx, check_no_field, range_c2c_to_api, range_s2s_to_api


def test_proj_add_change(client, consts):
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_drone_id = client.mk_eve_item(
        attrs={
            eve_radius_attr_id: 25,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    # Affector ship radius should be ignored
    eve_affector_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 1000, eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affector_ship_id)
    api_affector_drone1 = api_affector_fit.add_drone(
        type_id=eve_affector_drone_id,
        state=consts.ApiMinionState.engaging)
    api_affector_drone2 = api_affector_fit.add_drone(
        type_id=eve_affector_drone_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit1 = api_sol.create_fit()
    api_affectee_ship1 = api_affectee_fit1.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_fit2 = api_sol.create_fit()
    api_affectee_ship2 = api_affectee_fit2.set_ship(type_id=eve_affectee_ship_id)
    api_affector_drone1.change_drone(add_projs=[(api_affectee_ship1.id, range_c2c_to_api(val=11000))])
    # Verification
    assert api_affector_drone1.update().projs[api_affectee_ship1.id] == (11000, 9975)
    api_affectee_ship1.update()
    assert api_affectee_ship1.attrs[eve_affectee_attr_id].dogma == approx(256.83146)
    api_mod = api_affectee_ship1.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-85)
    assert api_mod.range_mult == approx(0.5721613)
    assert api_mod.applied_val == approx(-48.633708)
    # Action
    api_affector_drone2.change_drone(add_projs=[(api_affectee_ship2.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (12025, 11000)
    api_affectee_ship2.update()
    assert api_affectee_ship2.attrs[eve_affectee_attr_id].dogma == approx(287.5)
    api_mod = api_affectee_ship2.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-85)
    assert api_mod.range_mult == approx(0.5)
    assert api_mod.applied_val == approx(-42.5)
    # Action
    api_affector_drone1.change_drone(change_projs=[(api_affectee_ship1.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone1.update().projs[api_affectee_ship1.id] == (12025, 11000)
    api_affectee_ship1.update()
    assert api_affectee_ship1.attrs[eve_affectee_attr_id].dogma == approx(287.5)
    api_mod = api_affectee_ship1.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-85)
    assert api_mod.range_mult == approx(0.5)
    assert api_mod.applied_val == approx(-42.5)
    # Action
    api_affector_drone2.change_drone(change_projs=[(api_affectee_ship2.id, range_c2c_to_api(val=11000))])
    # Verification
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (11000, 9975)
    api_affectee_ship2.update()
    assert api_affectee_ship2.attrs[eve_affectee_attr_id].dogma == approx(256.83146)
    api_mod = api_affectee_ship2.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-85)
    assert api_mod.range_mult == approx(0.5721613)
    assert api_mod.applied_val == approx(-48.633708)


def test_outgoing_switch_type_id(client, consts):
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_drone1_id = client.mk_eve_item(
        attrs={
            eve_radius_attr_id: 25,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affector_drone2_id = client.mk_eve_item(
        attrs={
            eve_radius_attr_id: 2500,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affector_drone3_id = client.alloc_item_id()
    eve_affector_ship_id = client.mk_eve_ship()
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 1000, eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affector_ship_id)
    api_affector_drone = api_affector_fit.add_drone(
        type_id=eve_affector_drone1_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_drone.change_drone(add_projs=[(api_affectee_ship.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (12025, 11000)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(287.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-85)
    assert api_mod.range_mult == approx(0.5)
    assert api_mod.applied_val == approx(-42.5)
    # Action
    api_affector_drone.change_drone(type_id=eve_affector_drone2_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (12025, 8525)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(212.968994)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-85)
    assert api_mod.range_mult == approx(0.6753671)
    assert api_mod.applied_val == approx(-57.406201)
    # Action
    api_affector_drone.change_drone(type_id=eve_affector_drone3_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (12025, 11025)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_ship.mods  # noqa: B018
