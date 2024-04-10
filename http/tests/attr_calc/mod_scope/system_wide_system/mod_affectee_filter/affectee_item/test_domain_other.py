from pytest import approx


def test_unaffected_self(client, consts):
    # Check that other reference from system-wide effect doesn't affect anything
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_item = api_ss.add_sw_effect(type_id=eve_item.id)
    assert api_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_parent(client, consts):
    # Check that other reference from system-wide effect doesn't affect anything
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ss.add_sw_effect(type_id=eve_src_item.id)
    api_tgt_item = api_fit.set_ship(type_id=eve_tgt_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # Check that other reference from system-wide effect doesn't affect anything
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_module = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    eve_tgt_charge = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    eve_ship_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ss.add_sw_effect(type_id=eve_src_item.id)
    api_fit.set_ship(type_id=eve_ship_item.id)
    api_tgt_item = api_fit.add_mod(type_id=eve_tgt_module.id, charge_type_id=eve_tgt_charge.id)
    api_tgt_item.update()
    assert api_tgt_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_tgt_item.charge.attrs[eve_tgt_attr.id].dogma == approx(100)
