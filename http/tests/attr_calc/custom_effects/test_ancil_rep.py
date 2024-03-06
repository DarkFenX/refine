from pytest import approx


def test_local_rep(client, consts):
    # Check that paste boost works on local ancillary repairer
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.charged_armor_dmg_mult)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.armor_dmg_amount)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.fueled_armor_repair)
    eve_item_rep = client.mk_eve_item(attrs={eve_src_attr.id: 3, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_item_paste = client.mk_eve_item(id_=consts.Item.nanite_repair_paste)
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_rep.id, rack=consts.Rack.low, charge_type_id=eve_item_paste.id)
    assert api_item.update().attrs[eve_tgt_attr.id].dogma == approx(300)


def test_remote_rep(client, consts):
    # Check that paste boost works on remote ancillary repairer
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.charged_armor_dmg_mult)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.armor_dmg_amount)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.ship_module_arar)
    eve_item_rep = client.mk_eve_item(attrs={eve_src_attr.id: 3, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_item_paste = client.mk_eve_item(id_=consts.Item.nanite_repair_paste)
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_rep.id, rack=consts.Rack.low, charge_type_id=eve_item_paste.id)
    assert api_item.update().attrs[eve_tgt_attr.id].dogma == approx(300)
