from tests import Muta, Range, approx, check_no_field


def test_proj_add_change_outgoing(client, consts):
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
        id_=consts.UtilEffect.tgt_normal1,
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
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
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
    api_affector_drone1.change_drone(add_projs=[(api_affectee_ship1.id, Range.c2c_to_api(val=11000))])
    # Verification
    assert api_affector_drone1.update().projs[api_affectee_ship1.id] == (11000, 10975)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(286.763177)
    # Action
    api_affector_drone2.change_drone(add_projs=[(api_affectee_ship2.id, Range.s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (11025, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    # Action
    api_affector_drone1.change_drone(change_projs=[(api_affectee_ship1.id, Range.s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone1.update().projs[api_affectee_ship1.id] == (11025, 11000)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    # Action
    api_affector_drone2.change_drone(change_projs=[(api_affectee_ship2.id, Range.c2c_to_api(val=11000))])
    # Verification
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (11000, 10975)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(286.763177)


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
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_drone_id = client.mk_eve_item(attrs={eve_radius_attr_id: 100, eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_drone1 = api_affectee_fit.add_drone(type_id=eve_affectee_drone_id)
    api_affectee_drone2 = api_affectee_fit.add_drone(type_id=eve_affectee_drone_id)
    api_affector_module.change_module(add_projs=[(api_affectee_drone1.id, Range.c2c_to_api(val=11000))])
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone1.id] == (11000, 10900)
    assert api_affectee_drone1.update().attrs[eve_affectee_attr_id].dogma == approx(569.09709)
    # Action
    api_affector_module.change_module(add_projs=[(api_affectee_drone2.id, Range.s2s_to_api(val=11000))])
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone2.id] == (11100, 11000)
    assert api_affectee_drone2.update().attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_affector_module.change_module(change_projs=[(api_affectee_drone1.id, Range.s2s_to_api(val=11000))])
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone1.id] == (11100, 11000)
    assert api_affectee_drone1.update().attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_affector_module.change_module(change_projs=[(api_affectee_drone2.id, Range.c2c_to_api(val=11000))])
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone2.id] == (11000, 10900)
    assert api_affectee_drone2.update().attrs[eve_affectee_attr_id].dogma == approx(569.09709)


def test_mutation_outgoing(client, consts):
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
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])

    def make_eve_drone(*, radius: float | None) -> int:
        attrs = {
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000}
        if radius is not None:
            attrs[eve_radius_attr_id] = radius
        return client.mk_eve_item(attrs=attrs, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)

    eve_affector_drone1_base1_id = make_eve_drone(radius=100)
    eve_affector_drone1_base2_id = make_eve_drone(radius=10)
    eve_affector_drone1_mutated1_id = make_eve_drone(radius=500)
    eve_affector_drone1_mutated2_id = make_eve_drone(radius=1000)
    eve_affector_drone1_mutated3_id = make_eve_drone(radius=2000)
    eve_affector_drone1_mutator1_id = client.mk_eve_mutator(
        items=[
            ([eve_affector_drone1_base1_id], eve_affector_drone1_mutated1_id),
            ([eve_affector_drone1_base2_id], eve_affector_drone1_mutated2_id)],
        attrs={eve_radius_attr_id: (0.5, 1.5)})
    eve_affector_drone1_mutator2_id = client.mk_eve_mutator(
        items=[([eve_affector_drone1_base2_id], eve_affector_drone1_mutated3_id)],
        attrs={eve_radius_attr_id: (0.5, 1.5)})
    eve_affector_drone2_base1_id = make_eve_drone(radius=None)
    eve_affector_drone2_base2_id = make_eve_drone(radius=None)
    eve_affector_drone2_mutated1_id = make_eve_drone(radius=None)
    eve_affector_drone2_mutated2_id = make_eve_drone(radius=None)
    eve_affector_drone2_mutated3_id = make_eve_drone(radius=None)
    eve_affector_drone2_mutator1_id = client.mk_eve_mutator(items=[
        ([eve_affector_drone2_base1_id], eve_affector_drone2_mutated1_id),
        ([eve_affector_drone2_base2_id], eve_affector_drone2_mutated2_id)])
    eve_affector_drone2_mutator2_id = client.mk_eve_mutator(
        items=[([eve_affector_drone2_base2_id], eve_affector_drone2_mutated3_id)])
    eve_affector_drone3_base1_id = client.alloc_item_id()
    eve_affector_drone3_base2_id = client.alloc_item_id()
    eve_affector_drone3_mutator1_id = client.alloc_item_id()
    eve_affector_drone3_mutator2_id = client.alloc_item_id()
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_drone1 = api_affector_fit.add_drone(
        type_id=eve_affector_drone1_base1_id,
        state=consts.ApiMinionState.engaging)
    api_affector_drone2 = api_affector_fit.add_drone(
        type_id=eve_affector_drone2_base1_id,
        state=consts.ApiMinionState.engaging)
    api_affector_drone3 = api_affector_fit.add_drone(
        type_id=eve_affector_drone3_base1_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit1 = api_sol.create_fit()
    api_affectee_ship1 = api_affectee_fit1.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_fit2 = api_sol.create_fit()
    api_affectee_ship2 = api_affectee_fit2.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_fit3 = api_sol.create_fit()
    api_affectee_ship3 = api_affectee_fit3.set_ship(type_id=eve_affectee_ship_id)
    api_affector_drone1.change_drone(add_projs=[(api_affectee_ship1.id, Range.s2s_to_api(val=11000))])
    api_affector_drone2.change_drone(add_projs=[(api_affectee_ship2.id, Range.s2s_to_api(val=11000))])
    api_affector_drone3.change_drone(add_projs=[(api_affectee_ship3.id, Range.s2s_to_api(val=11000))])
    # Verification
    api_affector_drone1.update()
    assert api_affector_drone1.attrs[eve_radius_attr_id].dogma == approx(100)
    assert api_affector_drone1.projs[api_affectee_ship1.id] == (11100, 11000)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (11000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    assert api_affector_drone3.update().projs[api_affectee_ship3.id] == (11000, 11000)
    assert api_affectee_ship3.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action - mutate drones + mutate radius attribute on drone 1
    api_affector_drone1.change_drone(
        mutation=(eve_affector_drone1_mutator1_id, {eve_radius_attr_id: Muta.roll_to_api(val=0)}))
    api_affector_drone2.change_drone(mutation=eve_affector_drone2_mutator1_id)
    api_affector_drone3.change_drone(mutation=eve_affector_drone3_mutator1_id)
    # Verification - unmutated drone radius should be used on drone 1
    api_affector_drone1.update()
    assert api_affector_drone1.attrs[eve_radius_attr_id].dogma == approx(250)
    assert api_affector_drone1.projs[api_affectee_ship1.id] == (11100, 10600)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(275.632636)
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (11000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    assert api_affector_drone3.update().projs[api_affectee_ship3.id] == (11000, 11000)
    assert api_affectee_ship3.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action - remove projections
    api_affector_drone1.change_drone(rm_projs=[api_affectee_ship1.id])
    api_affector_drone2.change_drone(rm_projs=[api_affectee_ship2.id])
    api_affector_drone3.change_drone(rm_projs=[api_affectee_ship3.id])
    # Verification
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    assert api_affectee_ship3.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action - restore projections as they were
    api_affector_drone1.change_drone(add_projs=[(api_affectee_ship1.id, Range.s2s_to_api(val=10600))])
    api_affector_drone2.change_drone(add_projs=[(api_affectee_ship2.id, Range.s2s_to_api(val=11000))])
    api_affector_drone3.change_drone(add_projs=[(api_affectee_ship3.id, Range.s2s_to_api(val=11000))])
    # Verification - drone 1 should still use unmutated radius
    api_affector_drone1.update()
    assert api_affector_drone1.attrs[eve_radius_attr_id].dogma == approx(250)
    assert api_affector_drone1.projs[api_affectee_ship1.id] == (11100, 10600)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(275.632636)
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (11000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    assert api_affector_drone3.update().projs[api_affectee_ship3.id] == (11000, 11000)
    assert api_affectee_ship3.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action - change item's base type ID
    api_affector_drone1.change_drone(type_id=eve_affector_drone1_base2_id)
    api_affector_drone2.change_drone(type_id=eve_affector_drone2_base2_id)
    api_affector_drone3.change_drone(type_id=eve_affector_drone3_base2_id)
    # Verification
    api_affector_drone1.update()
    assert api_affector_drone1.attrs[eve_radius_attr_id].dogma == approx(500)
    assert api_affector_drone1.projs[api_affectee_ship1.id] == (11100, 10100)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(260.610008)
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (11000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    assert api_affector_drone3.update().projs[api_affectee_ship3.id] == (11000, 11000)
    assert api_affectee_ship3.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action - change item's mutator ID
    api_affector_drone1.change_drone(mutation=eve_affector_drone1_mutator2_id)
    api_affector_drone2.change_drone(mutation=eve_affector_drone2_mutator2_id)
    api_affector_drone3.change_drone(mutation=eve_affector_drone3_mutator2_id)
    # Verification
    api_affector_drone1.update()
    assert api_affector_drone1.attrs[eve_radius_attr_id].dogma == approx(1000)
    assert api_affector_drone1.projs[api_affectee_ship1.id] == (11100, 9100)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(230.298632)
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (11000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    assert api_affector_drone3.update().projs[api_affectee_ship3.id] == (11000, 11000)
    assert api_affectee_ship3.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action - unmutate item
    api_affector_drone1.change_drone(mutation=None)
    api_affector_drone2.change_drone(mutation=None)
    api_affector_drone3.change_drone(mutation=None)
    # Verification
    api_affector_drone1.update()
    assert api_affector_drone1.attrs[eve_radius_attr_id].dogma == approx(10)
    assert api_affector_drone1.projs[api_affectee_ship1.id] == (11100, 11090)
    assert api_affectee_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(290.146599)
    assert api_affector_drone2.update().projs[api_affectee_ship2.id] == (11000, 11000)
    assert api_affectee_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    assert api_affector_drone3.update().projs[api_affectee_ship3.id] == (11000, 11000)
    assert api_affectee_ship3.update().attrs[eve_affectee_attr_id].dogma == approx(500)


def test_mutation_incoming(client, consts):
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
        id_=consts.UtilEffect.tgt_normal1,
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
    eve_affectee_drone1_mutator1_id = client.mk_eve_mutator(
        items=[
            ([eve_affectee_drone1_base1_id], eve_affectee_drone1_mutated1_id),
            ([eve_affectee_drone1_base2_id], eve_affectee_drone1_mutated2_id)],
        attrs={eve_radius_attr_id: (0.5, 1.5)})
    eve_affectee_drone1_mutator2_id = client.mk_eve_mutator(
        items=[([eve_affectee_drone1_base2_id], eve_affectee_drone1_mutated3_id)],
        attrs={eve_radius_attr_id: (0.5, 1.5)})
    eve_affectee_drone2_base1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 1000})
    eve_affectee_drone2_base2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 1000})
    eve_affectee_drone2_mutated1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 1000})
    eve_affectee_drone2_mutated2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 1000})
    eve_affectee_drone2_mutated3_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 1000})
    eve_affectee_drone2_mutator1_id = client.mk_eve_mutator(items=[
        ([eve_affectee_drone2_base1_id], eve_affectee_drone2_mutated1_id),
        ([eve_affectee_drone2_base2_id], eve_affectee_drone2_mutated2_id)])
    eve_affectee_drone2_mutator2_id = client.mk_eve_mutator(
        items=[([eve_affectee_drone2_base2_id], eve_affectee_drone2_mutated3_id)])
    eve_affectee_drone3_base1_id = client.alloc_item_id()
    eve_affectee_drone3_base2_id = client.alloc_item_id()
    eve_affectee_drone3_mutator1_id = client.alloc_item_id()
    eve_affectee_drone3_mutator2_id = client.alloc_item_id()
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
    api_affectee_drone3 = api_affectee_fit.add_drone(
        type_id=eve_affectee_drone3_base1_id,
        state=consts.ApiMinionState.engaging)
    api_affector_module.change_module(add_projs=[
        (api_affectee_drone1.id, Range.s2s_to_api(val=11000)),
        (api_affectee_drone2.id, Range.s2s_to_api(val=11000)),
        (api_affectee_drone3.id, Range.s2s_to_api(val=11000))])
    # Verification
    api_affector_module.update()
    assert api_affector_module.projs[api_affectee_drone1.id] == (11100, 11000)
    assert api_affector_module.projs[api_affectee_drone2.id] == (11000, 11000)
    assert api_affector_module.projs[api_affectee_drone3.id] == (11000, 11000)
    api_affectee_drone1.update()
    assert api_affectee_drone1.attrs[eve_radius_attr_id].dogma == approx(100)
    assert api_affectee_drone1.attrs[eve_affectee_attr_id].dogma == approx(575)
    api_affectee_drone3.update()
    with check_no_field():
        api_affectee_drone3.attrs  # noqa: B018
    # Action - mutate drones + mutate radius attribute on drone 1
    api_affectee_drone1.change_drone(
        mutation=(eve_affectee_drone1_mutator1_id, {eve_radius_attr_id: Muta.roll_to_api(val=0)}))
    api_affectee_drone2.change_drone(mutation=eve_affectee_drone2_mutator1_id)
    api_affectee_drone3.change_drone(mutation=eve_affectee_drone3_mutator1_id)
    # Verification - unmutated drone radius should be used on drone 1
    api_affector_module.update()
    assert api_affector_module.projs[api_affectee_drone1.id] == (11100, 10600)
    assert api_affector_module.projs[api_affectee_drone2.id] == (11000, 11000)
    assert api_affector_module.projs[api_affectee_drone3.id] == (11000, 11000)
    api_affectee_drone1.update()
    assert api_affectee_drone1.attrs[eve_radius_attr_id].dogma == approx(250)
    assert api_affectee_drone1.attrs[eve_affectee_attr_id].dogma == approx(551.265272)
    assert api_affectee_drone2.update().attrs[eve_affectee_attr_id].dogma == approx(575.0)
    api_affectee_drone3.update()
    with check_no_field():
        api_affectee_drone3.attrs  # noqa: B018
    # Action - remove projections
    api_affector_module.change_module(rm_projs=[api_affectee_drone1.id, api_affectee_drone2.id, api_affectee_drone3.id])
    # Verification
    api_affectee_drone1.update()
    assert api_affectee_drone1.attrs[eve_radius_attr_id].dogma == approx(250)
    assert api_affectee_drone1.attrs[eve_affectee_attr_id].dogma == approx(1000)
    assert api_affectee_drone2.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    api_affectee_drone3.update()
    with check_no_field():
        api_affectee_drone3.attrs  # noqa: B018
    # Action - restore projections as they were
    api_affector_module.change_module(add_projs=[
        (api_affectee_drone1.id, Range.s2s_to_api(val=10600)),
        (api_affectee_drone2.id, Range.s2s_to_api(val=11000)),
        (api_affectee_drone3.id, Range.s2s_to_api(val=11000))])
    # Verification - drone 1 should still use unmutated radius
    api_affector_module.update()
    assert api_affector_module.projs[api_affectee_drone1.id] == (11100, 10600)
    assert api_affector_module.projs[api_affectee_drone2.id] == (11000, 11000)
    assert api_affector_module.projs[api_affectee_drone3.id] == (11000, 11000)
    api_affectee_drone1.update()
    assert api_affectee_drone1.attrs[eve_radius_attr_id].dogma == approx(250)
    assert api_affectee_drone1.attrs[eve_affectee_attr_id].dogma == approx(551.265272)
    assert api_affectee_drone2.update().attrs[eve_affectee_attr_id].dogma == approx(575.0)
    api_affectee_drone3.update()
    with check_no_field():
        api_affectee_drone3.attrs  # noqa: B018
    # Action - change item's base type ID
    api_affectee_drone1.change_drone(type_id=eve_affectee_drone1_base2_id)
    api_affectee_drone2.change_drone(type_id=eve_affectee_drone2_base2_id)
    api_affectee_drone3.change_drone(type_id=eve_affectee_drone3_base2_id)
    # Verification
    api_affector_module.update()
    assert api_affector_module.projs[api_affectee_drone1.id] == (11100, 10100)
    assert api_affector_module.projs[api_affectee_drone2.id] == (11000, 11000)
    assert api_affector_module.projs[api_affectee_drone3.id] == (11000, 11000)
    api_affectee_drone1.update()
    assert api_affectee_drone1.attrs[eve_radius_attr_id].dogma == approx(500)
    assert api_affectee_drone1.attrs[eve_affectee_attr_id].dogma == approx(521.220016)
    assert api_affectee_drone2.update().attrs[eve_affectee_attr_id].dogma == approx(575.0)
    api_affectee_drone3.update()
    with check_no_field():
        api_affectee_drone3.attrs  # noqa: B018
    # Action - change item's mutator ID
    api_affectee_drone1.change_drone(mutation=eve_affectee_drone1_mutator2_id)
    api_affectee_drone2.change_drone(mutation=eve_affectee_drone2_mutator2_id)
    api_affectee_drone3.change_drone(mutation=eve_affectee_drone3_mutator2_id)
    # Verification
    api_affector_module.update()
    assert api_affector_module.projs[api_affectee_drone1.id] == (11100, 9100)
    assert api_affector_module.projs[api_affectee_drone2.id] == (11000, 11000)
    assert api_affector_module.projs[api_affectee_drone3.id] == (11000, 11000)
    api_affectee_drone1.update()
    assert api_affectee_drone1.attrs[eve_radius_attr_id].dogma == approx(1000)
    assert api_affectee_drone1.attrs[eve_affectee_attr_id].dogma == approx(460.597263)
    assert api_affectee_drone2.update().attrs[eve_affectee_attr_id].dogma == approx(575.0)
    api_affectee_drone3.update()
    with check_no_field():
        api_affectee_drone3.attrs  # noqa: B018
    # Action - unmutate item
    api_affectee_drone1.change_drone(mutation=None)
    api_affectee_drone2.change_drone(mutation=None)
    api_affectee_drone3.change_drone(mutation=None)
    # Verification
    api_affector_module.update()
    assert api_affector_module.projs[api_affectee_drone1.id] == (11100, 11090)
    assert api_affector_module.projs[api_affectee_drone2.id] == (11000, 11000)
    assert api_affector_module.projs[api_affectee_drone3.id] == (11000, 11000)
    api_affectee_drone1.update()
    assert api_affectee_drone1.attrs[eve_radius_attr_id].dogma == approx(10)
    assert api_affectee_drone1.attrs[eve_affectee_attr_id].dogma == approx(580.293199)
    assert api_affectee_drone2.update().attrs[eve_affectee_attr_id].dogma == approx(575.0)
    api_affectee_drone3.update()
    with check_no_field():
        api_affectee_drone3.attrs  # noqa: B018


def test_switch_type_id_outgoing(client, consts):
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
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])

    def make_eve_drone(*, radius: float | None) -> int:
        attrs = {
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000}
        if radius is not None:
            attrs[eve_radius_attr_id] = radius
        return client.mk_eve_item(attrs=attrs, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)

    eve_affector_drone1_id = make_eve_drone(radius=25)
    eve_affector_drone2_id = make_eve_drone(radius=2500)
    eve_affector_drone3_id = make_eve_drone(radius=None)
    eve_affector_drone4_id = client.alloc_item_id()
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_drone = api_affector_fit.add_drone(
        type_id=eve_affector_drone1_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_drone.change_drone(add_projs=[(api_affectee_ship.id, Range.s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    # Action
    api_affector_drone.change_drone(type_id=eve_affector_drone2_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (11025, 8525)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(212.968994)
    # Action
    api_affector_drone.change_drone(type_id=eve_affector_drone3_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (11025, 11025)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(288.236112)
    # Action
    api_affector_drone.change_drone(type_id=eve_affector_drone4_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (11025, 11025)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action
    api_affector_drone.change_drone(type_id=eve_affector_drone1_id)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)


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
        id_=consts.UtilEffect.tgt_normal1,
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
    eve_affectee_drone3_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 1000})
    eve_affectee_drone4_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone1_id)
    api_affector_module.change_module(add_projs=[(api_affectee_drone.id, Range.s2s_to_api(val=11000))])
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
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(576.472223)
    # Action
    api_affectee_drone.change_drone(type_id=eve_affectee_drone4_id)
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


def test_switch_src_outgoing(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d3 = client.mk_eve_data()
    eve_d4 = client.mk_eve_data()
    eve_radius_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3, eve_d4], id_=consts.EveAttr.radius)
    eve_affector_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3])
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3, eve_d4])
    eve_optimal_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3])
    eve_falloff_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2, eve_d3])
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2, eve_d3],
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_drone_id = client.alloc_item_id(datas=[eve_d1, eve_d2, eve_d3, eve_d4])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_affector_drone_id,
        attrs={
            eve_radius_attr_id: 25,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_affector_drone_id,
        attrs={
            eve_radius_attr_id: 2500,
            eve_affector_attr_id: -85,
            eve_optimal_attr_id: 1000,
            eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.mk_eve_item(
        datas=[eve_d3],
        id_=eve_affector_drone_id,
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(datas=[eve_d1, eve_d2, eve_d3, eve_d4], attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_affector_fit = api_sol.create_fit()
    api_affector_drone = api_affector_fit.add_drone(
        type_id=eve_affector_drone_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_drone.change_drone(add_projs=[(api_affectee_ship.id, Range.s2s_to_api(val=11000))])
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (11025, 8525)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(212.968994)
    # Action
    api_sol.change_src(data=eve_d3)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (11025, 11025)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(288.236112)
    # Action
    api_sol.change_src(data=eve_d4)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (11025, 11025)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_affector_drone.update().projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)


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
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2, eve_d3, eve_d4],
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_drone_id = client.alloc_item_id(datas=[eve_d1, eve_d2, eve_d3, eve_d4])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_affectee_drone_id,
        attrs={eve_radius_attr_id: 25, eve_affectee_attr_id: 1000})
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_affectee_drone_id,
        attrs={eve_radius_attr_id: 2500, eve_affectee_attr_id: 1000})
    client.mk_eve_item(datas=[eve_d3], id_=eve_affectee_drone_id, attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone_id)
    api_affector_module.change_module(add_projs=[(api_affectee_drone.id, Range.s2s_to_api(val=11000))])
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 11000)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 8525)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(425.937987)
    # Action
    api_sol.change_src(data=eve_d3)
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 11025)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(576.472223)
    # Action
    api_sol.change_src(data=eve_d4)
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 11025)
    api_affectee_drone.update()
    with check_no_field():
        api_affectee_drone.attrs  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 11000)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(575)


def test_modified_radius_outgoing(client, consts):
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
        id_=consts.UtilEffect.tgt_normal1,
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
        defeff_id=eve_effect_id,
        srqs={eve_skill_id: 1})
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
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
    api_affector_drone = api_affector_fit.add_drone(
        type_id=eve_affector_drone_id,
        state=consts.ApiMinionState.engaging)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_drone.change_drone(add_projs=[(api_affectee_ship.id, Range.s2s_to_api(val=11000))])
    # Verification - modified radius is 1000, but unmodified radius is used for projections
    api_affector_drone.update()
    assert api_affector_drone.projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affector_drone.attrs[eve_radius_attr_id].dogma == approx(1000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    # Action
    api_rig.remove()
    # Verification
    api_affector_drone.update()
    assert api_affector_drone.projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affector_drone.attrs[eve_radius_attr_id].dogma == approx(25)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)
    # Action
    api_affector_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_affector_drone.update()
    assert api_affector_drone.projs[api_affectee_ship.id] == (11025, 11000)
    assert api_affector_drone.attrs[eve_radius_attr_id].dogma == approx(1000)
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(287.5)


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
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_drone_id = client.mk_eve_item(
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
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_rig = api_affectee_fit.add_rig(type_id=eve_rig_id)
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone_id)
    api_affector_module.change_module(add_projs=[(api_affectee_drone.id, Range.s2s_to_api(val=11000))])
    # Verification - modified radius is 1000, but unmodified radius is used for projections
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 11000)
    api_affectee_drone.update()
    assert api_affectee_drone.attrs[eve_radius_attr_id].dogma == approx(1000)
    assert api_affectee_drone.attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_rig.remove()
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 11000)
    api_affectee_drone.update()
    assert api_affectee_drone.attrs[eve_radius_attr_id].dogma == approx(25)
    assert api_affectee_drone.attrs[eve_affectee_attr_id].dogma == approx(575)
    # Action
    api_affectee_fit.add_rig(type_id=eve_rig_id)
    # Verification
    assert api_affector_module.update().projs[api_affectee_drone.id] == (11025, 11000)
    api_affectee_drone.update()
    assert api_affectee_drone.attrs[eve_radius_attr_id].dogma == approx(1000)
    assert api_affectee_drone.attrs[eve_affectee_attr_id].dogma == approx(575)
