from tests import approx, check_no_field, range_c2c_to_api, range_s2s_to_api


def test_outgoing_proj_add_change(client, consts):
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
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(256.83146)
    # Action
    api_affector_drone2.change_drone(add_projs=[(api_affectee_ship2.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (12025, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    # Action
    api_affector_drone1.change_drone(change_projs=[(api_affectee_ship1.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone1.update().projs[api_affectee_ship1.id] == (12025, 11000)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    # Action
    api_affector_drone2.change_drone(change_projs=[(api_affectee_ship2.id, range_c2c_to_api(val=11000))])
    # Verification
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (11000, 9975)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(256.83146)


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

    def make_eve_drone(*, radius: float) -> int:
        return client.mk_eve_item(
            attrs={
                eve_radius_attr_id: radius,
                eve_affector_attr_id: -85,
                eve_optimal_attr_id: 1000,
                eve_falloff_attr_id: 10000},
            eff_ids=[eve_effect_id],
            defeff_id=eve_effect_id)

    eve_affector_drone1_id = make_eve_drone(radius=25)
    eve_affector_drone2_id = make_eve_drone(radius=2500)
    eve_affector_drone3_id = client.alloc_item_id()
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 1000, eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_drone = api_affector_fit.add_drone(
        type_id=eve_affector_drone1_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_drone.change_drone(add_projs=[(api_affectee_ship.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (12025, 11000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    # Action
    api_affector_drone.change_drone(type_id=eve_affector_drone2_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (12025, 8525)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(212.968994)
    # Action
    api_affector_drone.change_drone(type_id=eve_affector_drone3_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (12025, 11025)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action
    api_affector_drone.change_drone(type_id=eve_affector_drone1_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (12025, 11000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)


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
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_drone1_id = client.mk_eve_item(attrs={eve_radius_attr_id: 25, eve_affectee_attr_id: 1000})
    eve_affectee_drone2_id = client.mk_eve_item(attrs={eve_radius_attr_id: 2500, eve_affectee_attr_id: 1000})
    eve_affectee_drone3_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone1_id)
    api_affector_module.change_module(add_projs=[(api_affectee_drone.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 11000)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_affectee_drone.change_drone(type_id=eve_affectee_drone2_id)
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 8525)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(425.937987)
    # Action
    api_affectee_drone.change_drone(type_id=eve_affectee_drone3_id)
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 11025)
    api_affectee_drone.update()
    with check_no_field():
        api_affectee_drone.attrs  # noqa: B018
    # Action
    api_affectee_drone.change_drone(type_id=eve_affectee_drone1_id)
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 11000)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(575)


def test_outgoing_mutation(client, consts):
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

    def make_eve_drone(*, radius: float) -> int:
        return client.mk_eve_item(
            attrs={
                eve_radius_attr_id: radius,
                eve_affector_attr_id: -85,
                eve_optimal_attr_id: 1000,
                eve_falloff_attr_id: 10000},
            eff_ids=[eve_effect_id],
            defeff_id=eve_effect_id)

    eve_affector_drone1_base1_id = make_eve_drone(radius=100)
    eve_affector_drone1_base2_id = make_eve_drone(radius=10)
    eve_affector_drone1_mutated1_id = make_eve_drone(radius=500)
    eve_affector_drone1_mutated2_id = make_eve_drone(radius=1000)
    eve_affector_drone1_mutated3_id = make_eve_drone(radius=2000)
    eve_affector_drone1_mutator1_id = client.mk_eve_mutator(items=[
        ([eve_affector_drone1_base1_id], eve_affector_drone1_mutated1_id),
        ([eve_affector_drone1_base2_id], eve_affector_drone1_mutated2_id)])
    eve_affector_drone1_mutator2_id = client.mk_eve_mutator(
        items=[([eve_affector_drone1_base2_id], eve_affector_drone1_mutated3_id)])
    eve_affector_drone2_base1_id = client.alloc_item_id()
    eve_affector_drone2_base2_id = client.alloc_item_id()
    eve_affector_drone2_mutator1_id = client.alloc_item_id()
    eve_affector_drone2_mutator2_id = client.alloc_item_id()
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 1000, eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_drone1 = api_affector_fit.add_drone(
        type_id=eve_affector_drone1_base1_id,
        state=consts.ApiMinionState.engaging)
    api_affector_drone2 = api_affector_fit.add_drone(
        type_id=eve_affector_drone2_base1_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit1 = api_sol.create_fit()
    api_affectee_ship1 = api_affectee_fit1.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_fit2 = api_sol.create_fit()
    api_affectee_ship2 = api_affectee_fit2.set_ship(type_id=eve_affectee_ship_id)
    api_affector_drone1.change_drone(add_projs=[(api_affectee_ship1.id, range_s2s_to_api(val=11000))])
    api_affector_drone2.change_drone(add_projs=[(api_affectee_ship2.id, range_s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone1.update().projs[api_affectee_ship1.id] == (12100, 11000)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (12000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action - mutate item
    api_affector_drone1.change_drone(mutation=eve_affector_drone1_mutator1_id)
    api_affector_drone2.change_drone(mutation=eve_affector_drone2_mutator1_id)
    # Verification
    assert api_affector_drone1.update().projs[api_affectee_ship1.id] == (12100, 10600)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(275.632636)
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (12000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action - change item's base type ID
    api_affector_drone1.change_drone(type_id=eve_affector_drone1_base2_id)
    api_affector_drone2.change_drone(type_id=eve_affector_drone2_base2_id)
    # Verification
    assert api_affector_drone1.update().projs[api_affectee_ship1.id] == (12100, 10100)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(260.610008)
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (12000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action - change item's mutator ID
    api_affector_drone1.change_drone(mutation=eve_affector_drone1_mutator2_id)
    api_affector_drone2.change_drone(mutation=eve_affector_drone2_mutator2_id)
    # Verification
    assert api_affector_drone1.update().projs[api_affectee_ship1.id] == (12100, 9100)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(230.298632)
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (12000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action - unmutate item
    api_affector_drone1.change_drone(mutation=None)
    api_affector_drone2.change_drone(mutation=None)
    # Verification
    assert api_affector_drone1.update().projs[api_affectee_ship1.id] == (12100, 11090)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(290.146599)
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (12000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(500)


def test_incoming_mutation(client, consts):
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
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_drone1_base1_id = client.mk_eve_item(attrs={eve_radius_attr_id: 100, eve_affectee_attr_id: 1000})
    eve_affectee_drone1_base2_id = client.mk_eve_item(attrs={eve_radius_attr_id: 10, eve_affectee_attr_id: 1000})
    eve_affectee_drone1_mutated1_id = client.mk_eve_item(attrs={eve_radius_attr_id: 500, eve_affectee_attr_id: 1000})
    eve_affectee_drone1_mutated2_id = client.mk_eve_item(attrs={eve_radius_attr_id: 1000, eve_affectee_attr_id: 1000})
    eve_affectee_drone1_mutated3_id = client.mk_eve_item(attrs={eve_radius_attr_id: 2000, eve_affectee_attr_id: 1000})
    eve_affectee_drone1_mutator1_id = client.mk_eve_mutator(items=[
        ([eve_affectee_drone1_base1_id], eve_affectee_drone1_mutated1_id),
        ([eve_affectee_drone1_base2_id], eve_affectee_drone1_mutated2_id)])
    eve_affectee_drone1_mutator2_id = client.mk_eve_mutator(
        items=[([eve_affectee_drone1_base2_id], eve_affectee_drone1_mutated3_id)])
    eve_affectee_drone2_base1_id = client.alloc_item_id()
    eve_affectee_drone2_base2_id = client.alloc_item_id()
    eve_affectee_drone2_mutator1_id = client.alloc_item_id()
    eve_affectee_drone2_mutator2_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_drone1 = api_affectee_fit.add_drone(
        type_id=eve_affectee_drone1_base1_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_drone2 = api_affectee_fit.add_drone(
        type_id=eve_affectee_drone2_base1_id,
        state=consts.ApiMinionState.engaging)
    api_affector_module.change_module(add_projs=[
        (api_affectee_drone1.id, range_s2s_to_api(val=11000)),
        (api_affectee_drone2.id, range_s2s_to_api(val=11000))])
    # Verification
    api_affector_module.update()
    assert api_affector_module.projs[api_affectee_drone1.id] == (11100, 11000)
    assert api_affector_module.projs[api_affectee_drone2.id] == (11000, 11000)
    assert api_affectee_drone1.update().attrs[eve_affectee_attr_id].dogma == approx(575)
    api_affectee_drone2.update()
    with check_no_field():
        api_affectee_drone2.attrs  # noqa: B018
    # Action - mutate item
    api_affectee_drone1.change_drone(mutation=eve_affectee_drone1_mutator1_id)
    api_affectee_drone2.change_drone(mutation=eve_affectee_drone2_mutator1_id)
    # Verification
    api_affector_module.update()
    assert api_affector_module.projs[api_affectee_drone1.id] == (11100, 10600)
    assert api_affector_module.projs[api_affectee_drone2.id] == (11000, 11000)
    assert api_affectee_drone1.update().attrs[eve_affectee_attr_id].dogma == approx(551.265272)
    api_affectee_drone2.update()
    with check_no_field():
        api_affectee_drone2.attrs  # noqa: B018
    # Action - change item's base type ID
    api_affectee_drone1.change_drone(type_id=eve_affectee_drone1_base2_id)
    api_affectee_drone2.change_drone(type_id=eve_affectee_drone2_base2_id)
    # Verification
    api_affector_module.update()
    assert api_affector_module.projs[api_affectee_drone1.id] == (11100, 10100)
    assert api_affector_module.projs[api_affectee_drone2.id] == (11000, 11000)
    assert api_affectee_drone1.update().attrs[eve_affectee_attr_id].dogma == approx(521.220016)
    api_affectee_drone2.update()
    with check_no_field():
        api_affectee_drone2.attrs  # noqa: B018
    # Action - change item's mutator ID
    api_affectee_drone1.change_drone(mutation=eve_affectee_drone1_mutator2_id)
    api_affectee_drone2.change_drone(mutation=eve_affectee_drone2_mutator2_id)
    # Verification
    api_affector_module.update()
    assert api_affector_module.projs[api_affectee_drone1.id] == (11100, 9100)
    assert api_affector_module.projs[api_affectee_drone2.id] == (11000, 11000)
    assert api_affectee_drone1.update().attrs[eve_affectee_attr_id].dogma == approx(460.597263)
    api_affectee_drone2.update()
    with check_no_field():
        api_affectee_drone2.attrs  # noqa: B018
    # Action - unmutate item
    api_affectee_drone1.change_drone(mutation=None)
    api_affectee_drone2.change_drone(mutation=None)
    # Verification
    api_affector_module.update()
    assert api_affector_module.projs[api_affectee_drone1.id] == (11100, 11090)
    assert api_affector_module.projs[api_affectee_drone2.id] == (11000, 11000)
    assert api_affectee_drone1.update().attrs[eve_affectee_attr_id].dogma == approx(580.293199)
    api_affectee_drone2.update()
    with check_no_field():
        api_affectee_drone2.attrs  # noqa: B018
