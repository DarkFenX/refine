from pytest import approx


def test_proj_unproj(client, consts):
    # Check that effects are applied/removed when projected effect is applied/unapplied
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_attr1.id: 5}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    eve_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    api_fit = api_ss.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(7.5)
    eve_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(37.5)
    eve_proj_effect.change_proj_effect(rm_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(7.5)


def test_remove(client, consts):
    # Check that effects are removed when projected effect is removed
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_attr1.id: 5}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    eve_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    eve_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(37.5)
    eve_proj_effect.remove()
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(7.5)
