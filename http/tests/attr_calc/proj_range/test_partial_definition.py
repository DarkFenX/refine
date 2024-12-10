from tests import approx, check_no_field


def test_optimal_undefined(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60, eve_falloff_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_affectee_struct_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiState.active)
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_struct.mods  # pylint: disable=W0104
    # Action
    api_affector_module.change_mod(add_projs=[(api_affectee_struct.id, None)])
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.mods[eve_affectee_attr_id].one().range_mult is None
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_affector_module.change_mod(change_projs=[(api_affectee_struct.id, 5000)])
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(350)
    assert api_affectee_struct.mods[eve_affectee_attr_id].one().range_mult == approx(0.5)
    # Action
    api_affector_module.change_mod(rm_projs=[api_affectee_struct.id])
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_struct.mods  # pylint: disable=W0104


def test_falloff_undefined(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60, eve_optimal_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_affectee_struct_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiState.active)
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_struct.mods  # pylint: disable=W0104
    # Action
    api_affector_module.change_mod(add_projs=[(api_affectee_struct.id, None)])
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(200)
    assert api_affectee_struct.mods[eve_affectee_attr_id].one().range_mult is None
    # Action
    api_affector_module.change_mod(change_projs=[(api_affectee_struct.id, 10000)])
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(200)
    assert api_affectee_struct.mods[eve_affectee_attr_id].one().range_mult == approx(1)
    # Action
    api_affector_module.change_mod(change_projs=[(api_affectee_struct.id, 10000.01)])
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(500)
    # Here, modification was filtered out as ineffective
    with check_no_field():
        api_affectee_struct.mods  # pylint: disable=W0104
    # Action
    api_affector_module.change_mod(rm_projs=[api_affectee_struct.id])
    # Verification
    api_affectee_struct.update()
    assert api_affectee_struct.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_struct.mods  # pylint: disable=W0104
