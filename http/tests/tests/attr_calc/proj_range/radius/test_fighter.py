from tests import approx, check_no_field, range_c2c_to_api, range_s2s_to_api


def test_proj_add_change_outgoing(client, consts):
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
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
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    # Autocharge radius should be ignored
    eve_autocharge_id = client.mk_eve_item(
        attrs={
            eve_radius_attr_id: 5000,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect2_id],
        defeff_id=eve_effect2_id)
    eve_affector_fighter_id = client.mk_eve_item(
        attrs={
            eve_radius_attr_id: 25,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000,
            eve_autocharge_attr_id: eve_autocharge_id},
        eff_ids=[eve_effect1_id, eve_autocharge_effect_id],
        defeff_id=eve_effect1_id)
    # Affector ship radius should be ignored
    eve_affector_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 500, eve_affectee_attr2_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affector_ship_id)
    api_affector_fighter1 = api_affector_fit.add_fighter(
        type_id=eve_affector_fighter_id,
        state=consts.ApiMinionState.engaging)
    api_affector_fighter2 = api_affector_fit.add_fighter(
        type_id=eve_affector_fighter_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit1 = api_sol.create_fit()
    api_affectee_ship1 = api_affectee_fit1.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_fit2 = api_sol.create_fit()
    api_affectee_ship2 = api_affectee_fit2.set_ship(type_id=eve_affectee_ship_id)
    api_affector_fighter1.change_fighter(add_projs=[(api_affectee_ship1.id, range_c2c_to_api(val=11000))])
    # Verification
    assert api_affector_fighter1.update().projs[api_affectee_ship1.id] == (11000, 10975)
    api_affectee_ship1.update()
    assert api_affectee_ship1.attrs[eve_affectee_attr1_id].dogma == approx(286.763177)
    assert api_affectee_ship1.attrs[eve_affectee_attr2_id].dogma == approx(286.763177)
    # Action
    api_affector_fighter2.change_fighter(add_projs=[(api_affectee_ship2.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_fighter2.update().projs[api_affectee_ship2.id] == (11025, 11000)
    api_affectee_ship2.update()
    assert api_affectee_ship2.attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship2.attrs[eve_affectee_attr2_id].dogma == approx(287.5)
    # Action
    api_affector_fighter1.change_fighter(change_projs=[(api_affectee_ship1.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_fighter1.update().projs[api_affectee_ship1.id] == (11025, 11000)
    api_affectee_ship1.update()
    assert api_affectee_ship1.attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship1.attrs[eve_affectee_attr2_id].dogma == approx(287.5)
    # Action
    api_affector_fighter2.change_fighter(change_projs=[(api_affectee_ship2.id, range_c2c_to_api(val=11000))])
    # Verification
    assert api_affector_fighter2.update().projs[api_affectee_ship2.id] == (11000, 10975)
    api_affectee_ship2.update()
    assert api_affectee_ship2.attrs[eve_affectee_attr1_id].dogma == approx(286.763177)
    assert api_affectee_ship2.attrs[eve_affectee_attr2_id].dogma == approx(286.763177)


def test_proj_add_change_incoming(client, consts):
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
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_fighter_id = client.mk_eve_item(attrs={eve_radius_attr_id: 100, eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_drone = api_affector_fit.add_drone(type_id=eve_affector_drone_id, state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_fighter1 = api_affectee_fit.add_fighter(type_id=eve_affectee_fighter_id)
    api_affectee_fighter2 = api_affectee_fit.add_fighter(type_id=eve_affectee_fighter_id)
    api_affector_drone.change_drone(add_projs=[(api_affectee_fighter1.id, range_c2c_to_api(val=11000))])
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter1.id] == (11000, 10900)
    assert api_affectee_fighter1.update().attrs[eve_affectee_attr_id].dogma == approx(569.09709)
    # Action
    api_affector_drone.change_drone(add_projs=[(api_affectee_fighter2.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter2.id] == (11100, 11000)
    assert api_affectee_fighter2.update().attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_affector_drone.change_drone(change_projs=[(api_affectee_fighter1.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter1.id] == (11100, 11000)
    assert api_affectee_fighter1.update().attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_affector_drone.change_drone(change_projs=[(api_affectee_fighter2.id, range_c2c_to_api(val=11000))])
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter2.id] == (11000, 10900)
    assert api_affectee_fighter2.update().attrs[eve_affectee_attr_id].dogma == approx(569.09709)


def test_switch_type_id_outgoing(client, consts):
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
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
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_autocharge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect2_id],
        defeff_id=eve_effect2_id)

    def make_eve_fighter(*, radius: float | None) -> int:
        attrs = {
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000,
            eve_autocharge_attr_id: eve_autocharge_id}
        if radius is not None:
            attrs[eve_radius_attr_id] = radius
        return client.mk_eve_item(
            attrs=attrs,
            eff_ids=[eve_effect1_id, eve_autocharge_effect_id],
            defeff_id=eve_effect1_id)

    eve_affector_fighter1_id = make_eve_fighter(radius=25)
    eve_affector_fighter2_id = make_eve_fighter(radius=500)
    eve_affector_fighter3_id = make_eve_fighter(radius=None)
    eve_affector_fighter4_id = client.alloc_item_id()
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 500, eve_affectee_attr2_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fighter = api_affector_fit.add_fighter(
        type_id=eve_affector_fighter1_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_fighter.change_fighter(add_projs=[(api_affectee_ship.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 11000)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(287.5)
    # Action
    api_affector_fighter.change_fighter(type_id=eve_affector_fighter2_id)
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 10525)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(273.390775)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(273.390775)
    # Action
    api_affector_fighter.change_fighter(type_id=eve_affector_fighter3_id)
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 11025)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(288.236112)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(288.236112)
    # Action
    api_affector_fighter.change_fighter(type_id=eve_affector_fighter4_id)
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 11025)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(500)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(500)
    # Action
    api_affector_fighter.change_fighter(type_id=eve_affector_fighter1_id)
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 11000)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(287.5)


def test_switch_type_id_incoming(client, consts):
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
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_fighter1_id = client.mk_eve_item(attrs={eve_radius_attr_id: 25, eve_affectee_attr_id: 1000})
    eve_affectee_fighter2_id = client.mk_eve_item(attrs={eve_radius_attr_id: 500, eve_affectee_attr_id: 1000})
    eve_affectee_fighter3_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 1000})
    eve_affectee_fighter4_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_drone = api_affector_fit.add_drone(type_id=eve_affector_drone_id, state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_fighter = api_affectee_fit.add_fighter(type_id=eve_affectee_fighter1_id)
    api_affector_drone.change_drone(add_projs=[(api_affectee_fighter.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 11000)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_affectee_fighter.change_fighter(type_id=eve_affectee_fighter2_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 10525)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(546.78155)
    # Action
    api_affectee_fighter.change_fighter(type_id=eve_affectee_fighter3_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 11025)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(576.472223)
    # Action
    api_affectee_fighter.change_fighter(type_id=eve_affectee_fighter4_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 11025)
    api_affectee_fighter.update()
    with check_no_field():
        api_affectee_fighter.attrs  # noqa: B018
    # Action
    api_affectee_fighter.change_fighter(type_id=eve_affectee_fighter1_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 11000)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(575)


def test_switch_src_outgoing(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d3 = client.mk_eve_data()
    eve_d4 = client.mk_eve_data()
    eve_radius_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3, eve_d4], id_=consts.EveAttr.radius)
    eve_affector_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3])
    eve_affectee_attr1_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3, eve_d4])
    eve_affectee_attr2_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3, eve_d4])
    eve_optimal_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3])
    eve_falloff_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3])
    eve_autocharge_attr_id = client.mk_eve_attr(
        datas=[eve_d1, eve_d2, eve_d3],
        id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr1_id)
    eve_effect1_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2, eve_d3],
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
        datas=[eve_d1, eve_d2, eve_d3],
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod2])
    eve_autocharge_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2, eve_d3],
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_d1_autocharge_id = client.alloc_item_id(datas=[eve_d1, eve_d2, eve_d3])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_d1_autocharge_id,
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect2_id],
        defeff_id=eve_effect2_id)
    eve_d2_autocharge_id = client.alloc_item_id(datas=[eve_d1, eve_d2, eve_d3])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_d2_autocharge_id,
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect2_id],
        defeff_id=eve_effect2_id)
    eve_d3_autocharge_id = client.alloc_item_id(datas=[eve_d1, eve_d2, eve_d3])
    client.mk_eve_item(
        datas=[eve_d3],
        id_=eve_d3_autocharge_id,
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect2_id],
        defeff_id=eve_effect2_id)
    eve_affector_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2, eve_d3, eve_d4])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_affector_fighter_id,
        attrs={
            eve_radius_attr_id: 25,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000,
            eve_autocharge_attr_id: eve_d1_autocharge_id},
        eff_ids=[eve_effect1_id, eve_autocharge_effect_id],
        defeff_id=eve_effect1_id)
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_affector_fighter_id,
        attrs={
            eve_radius_attr_id: 500,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000,
            eve_autocharge_attr_id: eve_d2_autocharge_id},
        eff_ids=[eve_effect1_id, eve_autocharge_effect_id],
        defeff_id=eve_effect1_id)
    client.mk_eve_item(
        datas=[eve_d3],
        id_=eve_affector_fighter_id,
        attrs={
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000,
            eve_autocharge_attr_id: eve_d3_autocharge_id},
        eff_ids=[eve_effect1_id, eve_autocharge_effect_id],
        defeff_id=eve_effect1_id)
    eve_affectee_ship_id = client.mk_eve_ship(
        datas=[eve_d1, eve_d2, eve_d3, eve_d4],
        attrs={eve_affectee_attr1_id: 500, eve_affectee_attr2_id: 500})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_affector_fit = api_sol.create_fit()
    api_affector_fighter = api_affector_fit.add_fighter(
        type_id=eve_affector_fighter_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_fighter.change_fighter(add_projs=[(api_affectee_ship.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 11000)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(287.5)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 10525)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(273.390775)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(273.390775)
    # Action
    api_sol.change_src(data=eve_d3)
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 11025)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(288.236112)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(288.236112)
    # Action
    api_sol.change_src(data=eve_d4)
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 11025)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(500)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(500)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_affector_fighter.update().projs[api_affectee_ship.id] == (11025, 11000)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(287.5)


def test_switch_src_incoming(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d3 = client.mk_eve_data()
    eve_d4 = client.mk_eve_data()
    eve_radius_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3, eve_d4], id_=consts.EveAttr.radius)
    eve_affector_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3, eve_d4])
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3, eve_d4])
    eve_optimal_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3, eve_d4])
    eve_falloff_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3, eve_d4])
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2, eve_d3, eve_d4],
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_drone_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2, eve_d3, eve_d4],
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2, eve_d3, eve_d4])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_affectee_fighter_id,
        attrs={eve_radius_attr_id: 25, eve_affectee_attr_id: 1000})
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_affectee_fighter_id,
        attrs={eve_radius_attr_id: 500, eve_affectee_attr_id: 1000})
    client.mk_eve_item(datas=[eve_d3], id_=eve_affectee_fighter_id, attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_affector_fit = api_sol.create_fit()
    api_affector_drone = api_affector_fit.add_drone(type_id=eve_affector_drone_id, state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_fighter = api_affectee_fit.add_fighter(type_id=eve_affectee_fighter_id)
    api_affector_drone.change_drone(add_projs=[(api_affectee_fighter.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 11000)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 10525)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(546.78155)
    # Action
    api_sol.change_src(data=eve_d3)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 11025)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(576.472223)
    # Action
    api_sol.change_src(data=eve_d4)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 11025)
    api_affectee_fighter.update()
    with check_no_field():
        api_affectee_fighter.attrs  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 11000)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(575)


def test_modified_radius_outgoing(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr1_id = client.mk_eve_attr()
    eve_affectee_attr2_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
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
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_autocharge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect2_id],
        defeff_id=eve_effect2_id)
    eve_affector_fighter_id = client.mk_eve_item(
        attrs={
            eve_radius_attr_id: 25,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000,
            eve_autocharge_attr_id: eve_autocharge_id},
        eff_ids=[eve_effect1_id, eve_autocharge_effect_id],
        defeff_id=eve_effect1_id,
        srqs={eve_skill_id: 1})
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr1_id: 500, eve_affectee_attr2_id: 500})
    eve_radius_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_radius_attr_id)
    eve_radius_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_radius_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1000}, eff_ids=[eve_radius_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_rig = api_affector_fit.add_rig(type_id=eve_rig_id)
    api_affector_fighter = api_affector_fit.add_fighter(
        type_id=eve_affector_fighter_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_fighter.change_fighter(add_projs=[(api_affectee_ship.id, range_s2s_to_api(val=11000))])
    # Verification - modified radius is 1000, but unmodified radius is used for projections
    api_affector_fighter.update()
    assert api_affector_fighter.projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affector_fighter.attrs[eve_radius_attr_id].dogma == approx(1000)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(287.5)
    # Action
    api_rig.remove()
    # Verification
    api_affector_fighter.update()
    assert api_affector_fighter.projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affector_fighter.attrs[eve_radius_attr_id].dogma == approx(25)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(287.5)
    # Action
    api_affector_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_affector_fighter.update()
    assert api_affector_fighter.projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affector_fighter.attrs[eve_radius_attr_id].dogma == approx(1000)
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr1_id].dogma == approx(287.5)
    assert api_affectee_ship.attrs[eve_affectee_attr2_id].dogma == approx(287.5)


def test_modified_radius_incoming(client, consts):
    eve_skill_id = client.mk_eve_item()
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
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_fighter_id = client.mk_eve_item(
        attrs={eve_radius_attr_id: 25, eve_affectee_attr_id: 1000},
        srqs={eve_skill_id: 1})
    eve_radius_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_radius_attr_id)
    eve_radius_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_radius_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1000}, eff_ids=[eve_radius_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_drone = api_affector_fit.add_drone(type_id=eve_affector_drone_id, state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_rig = api_affectee_fit.add_rig(type_id=eve_rig_id)
    api_affectee_fighter = api_affectee_fit.add_fighter(type_id=eve_affectee_fighter_id)
    api_affector_drone.change_drone(add_projs=[(api_affectee_fighter.id, range_s2s_to_api(val=11000))])
    # Verification - modified radius is 1000, but unmodified radius is used for projections
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 11000)
    api_affectee_fighter.update()
    assert api_affectee_fighter.attrs[eve_radius_attr_id].dogma == approx(1000)
    assert api_affectee_fighter.attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_rig.remove()
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 11000)
    api_affectee_fighter.update()
    assert api_affectee_fighter.attrs[eve_radius_attr_id].dogma == approx(25)
    assert api_affectee_fighter.attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_affectee_fit.add_rig(type_id=eve_rig_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_fighter.id] == (11025, 11000)
    api_affectee_fighter.update()
    assert api_affectee_fighter.attrs[eve_radius_attr_id].dogma == approx(1000)
    assert api_affectee_fighter.attrs[eve_affectee_attr_id].dogma == approx(575)
