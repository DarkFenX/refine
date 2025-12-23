from fw import approx


def test_unaffected(client, consts):
    # There isn't anything which can belong to a projected effect, so just check that ship and an
    # item on it are not affected
    eve_grp_id = client.mk_eve_ship_group()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.other,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 100})
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr_id].modified == approx(100)
    assert api_rig.update().attrs[eve_affectee_attr_id].modified == approx(100)
