from fw import approx, check_no_field


def test_cutoff(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_module_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_drone1_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: -0.00011})
    eve_affectee_drone2_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: -0.0001})
    eve_affectee_drone3_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.0001})
    eve_affectee_drone4_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.00011})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_drone1 = api_affectee_fit.add_drone(type_id=eve_affectee_drone1_id)
    api_affectee_drone2 = api_affectee_fit.add_drone(type_id=eve_affectee_drone2_id)
    api_affectee_drone3 = api_affectee_fit.add_drone(type_id=eve_affectee_drone3_id)
    api_affectee_drone4 = api_affectee_fit.add_drone(type_id=eve_affectee_drone4_id)
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module.change_module(
        add_projs=[api_affectee_drone1.id, api_affectee_drone2.id, api_affectee_drone3.id, api_affectee_drone4.id])
    # Verification - only drones 2 and 3 have resist below threshold, drone 1 and 4 completely
    # nullify effect
    api_affectee_drone1.update()
    assert api_affectee_drone1.attrs[eve_affectee_attr_id].dogma == approx(500.033)
    api_mod = api_affectee_drone1.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-60)
    assert api_mod.resist_mult == approx(-0.00011)
    assert api_mod.applied_val == approx(0.0066)
    api_affectee_drone2.update()
    assert api_affectee_drone2.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_drone2.mods  # noqa: B018
    api_affectee_drone3.update()
    assert api_affectee_drone3.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_drone3.mods  # noqa: B018
    api_affectee_drone4.update()
    assert api_affectee_drone4.attrs[eve_affectee_attr_id].dogma == approx(499.967)
    api_mod = api_affectee_drone4.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-60)
    assert api_mod.resist_mult == approx(0.00011)
    assert api_mod.applied_val == approx(-0.0066)
