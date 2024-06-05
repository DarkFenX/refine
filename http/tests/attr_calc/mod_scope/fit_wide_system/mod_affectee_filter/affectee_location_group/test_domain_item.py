from pytest import approx


def test_unaffected(client, consts):
    # There isn't anything which can belong to a fit-wide effect, so just check that ship and an
    # item on it are not affected
    eve_grp = client.mk_eve_ship_group()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.item,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_root_affectee_item = client.mk_eve_ship(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 100})
    eve_child_affectee_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    api_root_affectee_item = api_fit.set_ship(type_id=eve_root_affectee_item.id)
    api_child_affectee_item = api_fit.add_rig(type_id=eve_child_affectee_item.id)
    assert api_root_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    assert api_child_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)
