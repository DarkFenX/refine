from pytest import approx


def test_unaffected(client, consts):
    # There isn't anything which can belong to a system-wide effect, so just check that ship and an
    # item on it are not affected
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_root_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    eve_child_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_src_item.id)
    api_fit = api_sol.create_fit()
    api_root_tgt_item = api_fit.set_ship(type_id=eve_root_tgt_item.id)
    api_child_tgt_item = api_fit.add_rig(type_id=eve_child_tgt_item.id)
    assert api_root_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_child_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)
