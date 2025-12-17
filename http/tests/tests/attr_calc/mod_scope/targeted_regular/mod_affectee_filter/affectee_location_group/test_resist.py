from fw import approx


def test_resisted_value_change(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_boost_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
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
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr_id,
        affectee_attr_id=eve_resist_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_affectee_rig_id = client.mk_eve_item(attrs={eve_boost_attr_id: -25}, eff_ids=[eve_rig_effect_id])
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_resist_attr_id: 0.4})
    eve_affectee_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_module = api_affectee_fit.add_module(type_id=eve_affectee_module_id)
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(380)
    api_affectee_rig = api_affectee_fit.add_rig(type_id=eve_affectee_rig_id)
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(410)
    api_affectee_rig.remove()
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(380)
