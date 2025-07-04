from tests import approx, check_no_field, range_c2c_to_api, range_s2s_to_api


def test_outgoing_proj_add_change(client, consts):
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr1_id)
    eve_effect1_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr2_id)
    eve_effect2_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod2])
    eve_affector_module_id = client.mk_eve_item(
        attrs={
            # Affector module radius should be ignored
            eve_radius_attr_id: 5000,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect1_id],
        defeff_id=eve_effect1_id)
    eve_affector_charge_id = client.mk_eve_item(
        attrs={
            # Affector charge radius should be ignored
            eve_radius_attr_id: 5000,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect2_id],
        defeff_id=eve_effect2_id)
    eve_affector_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    eve_affectee_ship_id = client.mk_eve_ship(
        attrs={eve_radius_attr_id: 1000, eve_affectee_attr1_id: 500, eve_affectee_attr2_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affector_ship_id)
    api_affector_module1 = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        rack=consts.ApiRack.high,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_affector_charge_id)
    api_affector_module2 = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_affector_charge_id)
    api_affectee_fit1 = api_sol.create_fit()
    api_affectee_ship1 = api_affectee_fit1.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_fit2 = api_sol.create_fit()
    api_affectee_ship2 = api_affectee_fit2.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module1.change_module(add_projs=[(api_affectee_ship1.id, range_c2c_to_api(val=11000))])
    # Verification
    assert api_affector_module1.update().projs[api_affectee_ship1.id] == (11000, 8000)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr1_id].dogma == approx(197.389333)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr2_id].dogma == approx(197.389333)
    # Action
    api_affector_module2.change_module(add_projs=[(api_affectee_ship2.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_module2.update().projs[api_affectee_ship2.id] == (14000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr2_id].dogma == approx(287.5)
    # Action
    api_affector_module1.change_module(change_projs=[(api_affectee_ship1.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_module1.update().projs[api_affectee_ship1.id] == (14000, 11000)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr2_id].dogma == approx(287.5)
    # Action
    api_affector_module2.change_module(change_projs=[(api_affectee_ship2.id, range_c2c_to_api(val=11000))])
    # Verification
    assert api_affector_module2.update().projs[api_affectee_ship2.id] == (11000, 8000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr1_id].dogma == approx(197.389333)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr2_id].dogma == approx(197.389333)


def test_outgoing_switch_type_id(client, consts):
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr1_id)
    eve_effect1_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr2_id)
    eve_effect2_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod2])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect1_id],
        defeff_id=eve_effect1_id)
    eve_affector_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect2_id],
        defeff_id=eve_effect2_id)
    eve_affector_ship1_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    eve_affector_ship2_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 1000})
    eve_affector_ship3_id = client.alloc_item_id()
    eve_affectee_ship_id = client.mk_eve_ship(
        attrs={eve_radius_attr_id: 1000, eve_affectee_attr1_id: 500, eve_affectee_attr2_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_ship = api_affector_fit.set_ship(type_id=eve_affector_ship1_id)
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        rack=consts.ApiRack.low,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_affector_charge_id)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module.change_module(add_projs=[(api_affectee_ship.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_module.update().projs[api_affectee_ship.id] == (14000, 11000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship.update().attrs[eve_affectee_attr2_id].dogma == approx(287.5)
    # Action
    api_affector_ship.change_ship(type_id=eve_affector_ship2_id)
    # Verification
    assert api_affector_module.update().projs[api_affectee_ship.id] == (14000, 12000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr1_id].dogma == approx(316.285838)
    assert api_affectee_ship.update().attrs[eve_affectee_attr2_id].dogma == approx(316.285838)
    # Action
    api_affector_ship.change_ship(type_id=eve_affector_ship3_id)
    # Verification
    assert api_affector_module.update().projs[api_affectee_ship.id] == (14000, 13000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr1_id].dogma == approx(343.358896)
    assert api_affectee_ship.update().attrs[eve_affectee_attr2_id].dogma == approx(343.358896)
    # Action
    api_affector_ship.change_ship(type_id=eve_affector_ship1_id)
    # Verification
    assert api_affector_module.update().projs[api_affectee_ship.id] == (14000, 11000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship.update().attrs[eve_affectee_attr2_id].dogma == approx(287.5)


def test_incoming_switch_type_id(client, consts):
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
    eve_affector_fighter_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_ship1_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 25, eve_affectee_attr_id: 1000})
    eve_affectee_ship2_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 500, eve_affectee_attr_id: 1000})
    eve_affectee_ship3_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fighter = api_affector_fit.add_fighter(
        type_id=eve_affector_fighter_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship1_id)
    api_affector_fighter.change_fighter(add_projs=[(api_affectee_ship.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_affectee_ship.change_ship(type_id=eve_affectee_ship2_id)
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 10525)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(546.78155)
    # Action
    api_affectee_ship.change_ship(type_id=eve_affectee_ship3_id)
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 11025)
    api_affectee_ship.update()
    with check_no_field():
        api_affectee_ship.attrs  # noqa: B018
    # Action
    api_affectee_ship.change_ship(type_id=eve_affectee_ship1_id)
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(575)
