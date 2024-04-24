from pytest import approx


def test_affected_multiple(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_tgt_item1 = api_fit1.set_ship(type_id=eve_tgt_item.id)
    api_tgt_item2 = api_fit2.set_ship(type_id=eve_tgt_item.id)
    api_src_item = api_sol.add_sw_effect(type_id=eve_src_item.id, state=False)
    assert api_tgt_item1.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_tgt_item2.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    api_src_item.change_sw_effect(state=True)
    assert api_tgt_item1.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_tgt_item2.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    api_src_item.change_sw_effect(state=False)
    assert api_tgt_item1.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_tgt_item2.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_other_domain(client, consts):
    # Make sure "top" entities described by other domains are not affected
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_src_item.id)
    api_fit = api_sol.create_fit()
    api_tgt_item = api_fit.set_struct(type_id=eve_tgt_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # Check that items (in this case rig) are not affected if they belong to location even if
    # its "owner" (in this case ship) is affected
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    eve_ship_item = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_src_item.id)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_item.id)
    api_tgt_item = api_fit.add_rig(type_id=eve_tgt_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_replace_root(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item1 = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    eve_tgt_item2 = client.mk_eve_item(attrs={eve_tgt_attr.id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_tgt_item1 = api_fit.set_ship(type_id=eve_tgt_item1.id)
    api_sol.add_sw_effect(type_id=eve_src_item.id)
    assert api_tgt_item1.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    api_tgt_item2 = api_fit.set_ship(type_id=eve_tgt_item2.id)
    assert api_tgt_item2.update().attrs[eve_tgt_attr.id].dogma == approx(60)
