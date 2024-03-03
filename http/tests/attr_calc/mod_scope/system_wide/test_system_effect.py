from pytest import approx


def test_sw_effect_addition_removal(client, consts):
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.ModFunc.loc,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EffCat.system, mod_info=[eve_mod])
    eve_sw_effect = client.mk_eve_item(attrs={eve_attr1.id: 5}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item()
    eve_rig = client.mk_eve_item(attrs={eve_attr2.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(7.5)
    eve_sw_effect = api_ss.add_sw_effect(type_id=eve_sw_effect.id)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(37.5)
    eve_sw_effect.remove()
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(7.5)


def test_fit_addition_removal(client, consts):
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.ModFunc.loc,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EffCat.system, mod_info=[eve_mod])
    eve_sw_effect = client.mk_eve_item(attrs={eve_attr1.id: 5}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item()
    eve_rig = client.mk_eve_item(attrs={eve_attr2.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_ss.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit1 = api_ss.create_fit()
    api_fit1.set_ship(type_id=eve_ship.id)
    api_rig1 = api_fit1.add_rig(type_id=eve_rig.id)
    assert api_rig1.update().attrs[eve_attr2.id].dogma == approx(37.5)
    api_fit2 = api_ss.create_fit()
    api_fit2.set_ship(type_id=eve_ship.id)
    api_rig2 = api_fit2.add_rig(type_id=eve_rig.id)
    assert api_rig1.update().attrs[eve_attr2.id].dogma == approx(37.5)
    assert api_rig2.update().attrs[eve_attr2.id].dogma == approx(37.5)
    api_fit2.remove()
    assert api_rig1.update().attrs[eve_attr2.id].dogma == approx(37.5)
