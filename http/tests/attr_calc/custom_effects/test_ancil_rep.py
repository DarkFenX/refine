from pytest import approx


def test_local_aar(client, consts):
    # Check that paste boost works on local ancillary repairer
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.charged_armor_dmg_mult)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.armor_dmg_amount)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.fueled_armor_repair)
    eve_aar_item = client.mk_eve_item(attrs={eve_src_attr.id: 3, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_paste_item = client.mk_eve_item(id_=consts.Item.nanite_repair_paste)
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_aar_item = api_fit.add_mod(type_id=eve_aar_item.id, rack=consts.Rack.low, charge_type_id=eve_paste_item.id)
    api_aar_item.update()
    assert api_aar_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_tgt_attr.id].extra == approx(300)


def test_remote_aar(client, consts):
    # Check that paste boost works on remote ancillary repairer
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.charged_armor_dmg_mult)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.armor_dmg_amount)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.ship_module_arar)
    eve_aar_item = client.mk_eve_item(attrs={eve_src_attr.id: 3, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_paste_item = client.mk_eve_item(id_=consts.Item.nanite_repair_paste)
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_aar_item = api_fit.add_mod(type_id=eve_aar_item.id, rack=consts.Rack.high, charge_type_id=eve_paste_item.id)
    api_aar_item.update()
    assert api_aar_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_tgt_attr.id].extra == approx(300)


def test_charge_switch(client, consts):
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.charged_armor_dmg_mult)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.armor_dmg_amount)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.fueled_armor_repair)
    eve_aar_item = client.mk_eve_item(attrs={eve_src_attr.id: 3, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_paste_item = client.mk_eve_item(id_=consts.Item.nanite_repair_paste)
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_aar_item = api_fit.add_mod(type_id=eve_aar_item.id, rack=consts.Rack.low)
    api_aar_item.update()
    assert api_aar_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_tgt_attr.id].extra == approx(100)
    api_aar_item.change_mod(charge=eve_paste_item.id)
    api_aar_item.update()
    assert api_aar_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_tgt_attr.id].extra == approx(300)
    api_aar_item.change_mod(charge=None)
    api_aar_item.update()
    assert api_aar_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_tgt_attr.id].extra == approx(100)


def test_mult_change(client, consts):
    eve_ship_item = client.mk_eve_item()
    eve_aar_src_attr = client.mk_eve_attr(id_=consts.Attr.charged_armor_dmg_mult)
    eve_aar_tgt_attr = client.mk_eve_attr(id_=consts.Attr.armor_dmg_amount)
    eve_mod_src_attr = client.mk_eve_attr()
    eve_aar_effect = client.mk_eve_effect(id_=consts.Effect.fueled_armor_repair)
    eve_aar_item = client.mk_eve_item(
        attrs={eve_aar_src_attr.id: 3, eve_aar_tgt_attr.id: 100},
        eff_ids=[eve_aar_effect.id])
    eve_paste_item = client.mk_eve_item(id_=consts.Item.nanite_repair_paste)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.ModFunc.loc,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_mod_src_attr.id,
        tgt_attr_id=eve_aar_src_attr.id)
    eve_mod_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_mod_item = client.mk_eve_item(attrs={eve_mod_src_attr.id: 25}, eff_ids=[eve_mod_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_ship(eve_ship_item.id)
    api_aar_item = api_fit.add_mod(type_id=eve_aar_item.id, rack=consts.Rack.low, charge_type_id=eve_paste_item.id)
    api_aar_item.update()
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].extra == approx(300)
    api_mod_item = api_fit.add_rig(type_id=eve_mod_item.id)
    api_aar_item.update()
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].extra == approx(375)
    api_mod_item.remove()
    api_aar_item.update()
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].extra == approx(300)
