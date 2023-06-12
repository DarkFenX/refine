from pytest import approx

from tests.support.util import Default


def get_value_simple(client, attr_id, base_value):
    eve_attr = client.mk_eve_attr(id_=attr_id)
    eve_item = client.mk_eve_item(attrs={eve_attr.id: base_value})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    return api_item.update().attr_vals[eve_attr.id].dogma


def test_cpu_down(client, consts):
    value = get_value_simple(client, attr_id=consts.Attr.cpu, base_value=2.3333)
    assert value == approx(2.33)


def test_cpu_up(client, consts):
    value = get_value_simple(client, attr_id=consts.Attr.cpu, base_value=2.6666)
    assert value == approx(2.67)


def test_cpu_modified(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.cpu)
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 1.9444}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    value = api_item.update().attr_vals[eve_tgt_attr.id].dogma
    assert value == approx(2.33)


def test_cpu_output(client, consts):
    value = get_value_simple(client, attr_id=consts.Attr.cpu_output, base_value=2.6666)
    assert value == approx(2.67)


def test_power(client, consts):
    value = get_value_simple(client, attr_id=consts.Attr.power, base_value=2.6666)
    assert value == approx(2.67)


def test_power_output(client, consts):
    value = get_value_simple(client, attr_id=consts.Attr.power_output, base_value=2.6666)
    assert value == approx(2.67)


def test_other(client, consts):
    value = get_value_simple(client, attr_id=Default, base_value=2.6666)
    assert value == approx(2.6666)
