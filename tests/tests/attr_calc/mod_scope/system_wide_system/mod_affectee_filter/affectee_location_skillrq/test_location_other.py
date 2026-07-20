from fw import approx


def test_unaffected(client, consts):
    # There isn't anything which can belong to a system-wide effect, so just check that ship and an
    # item on it are not affected
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_root_affectee_item_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_child_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_affector_item_id)
    api_fit = api_sol.create_fit()
    api_root_affectee_item = api_fit.set_ship(type_id=eve_root_affectee_item_id)
    api_child_affectee_item = api_fit.add_rig(type_id=eve_child_affectee_item_id)
    assert api_root_affectee_item.update().attrs[eve_affectee_attr_id].modified == approx(100)
    assert api_child_affectee_item.update().attrs[eve_affectee_attr_id].modified == approx(100)
