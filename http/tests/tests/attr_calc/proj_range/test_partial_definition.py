from fw import approx, check_no_field


def test_optimal_undefined(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
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
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60, eve_falloff_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_struct_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_struct_id, coordinates=(0, 0, 0))
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_struct.mods  # noqa: B018
    # Action
    api_affector_module.change_module(add_projs=[api_affectee_struct.id])
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.mods[eve_affectee_attr_id].one().range_mult == approx(1)
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_affectee_struct.change_ship(coordinates=(5000, 0, 0))
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(350)
    assert api_affectee_struct.mods[eve_affectee_attr_id].one().range_mult == approx(0.5)
    # Action
    api_affector_module.change_module(rm_projs=[api_affectee_struct.id])
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_struct.mods  # noqa: B018


def test_falloff_undefined(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
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
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60, eve_optimal_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_struct_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_struct_id, coordinates=(0, 0, 0))
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_struct.mods  # noqa: B018
    # Action
    api_affector_module.change_module(add_projs=[api_affectee_struct.id])
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(200)
    assert api_affectee_struct.mods[eve_affectee_attr_id].one().range_mult == approx(1)
    # Action
    api_affectee_struct.change_ship(coordinates=(10000, 0, 0))
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(200)
    assert api_affectee_struct.mods[eve_affectee_attr_id].one().range_mult == approx(1)
    # Action
    api_affectee_struct.change_ship(coordinates=(10000.01, 0, 0))
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(500)
    # Here, modification was filtered out as ineffective
    with check_no_field():
        api_affectee_struct.mods  # noqa: B018
    # Action
    api_affector_module.change_module(rm_projs=[api_affectee_struct.id])
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_struct.mods  # noqa: B018
