from pytest import approx


def test_unaffected(client, consts):
    # There isn't anything which can belong to a projected effect, so just check that ship and an
    # item on it are not affected
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.item,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(100)
