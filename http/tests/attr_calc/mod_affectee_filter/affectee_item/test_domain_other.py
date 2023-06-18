from pytest import approx


def test_affected_charge_bundled(client, consts):
    # Check that charge is affected by module if they were added simultaneously
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.other,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_module = api_fit.add_high_mod(type_id=eve_src_item.id, charge_type_id=eve_tgt_item.id)
    value = api_module.update().charge.attr_vals[eve_tgt_attr.id].dogma
    assert value == approx(120)


def test_affected_module_bundled(client, consts):
    # Check that module is affected by charge if they were added simultaneously
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.other,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_module = api_fit.add_high_mod(type_id=eve_tgt_item.id, charge_type_id=eve_src_item.id)
    value = api_module.update().attr_vals[eve_tgt_attr.id].dogma
    assert value == approx(120)
