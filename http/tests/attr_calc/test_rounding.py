from tests import approx

from tests.support.util import Default


def get_value_simple(*, client, attr_id, base_value):
    eve_attr_id = client.mk_eve_attr(id_=attr_id)
    eve_item_id = client.mk_eve_item(attrs={eve_attr_id: base_value})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    return api_item.update().attrs[eve_attr_id].dogma


def test_cpu_down(client, consts):
    assert get_value_simple(client=client, attr_id=consts.EveAttr.cpu, base_value=2.3333) == 2.33


def test_cpu_up(client, consts):
    assert get_value_simple(client=client, attr_id=consts.EveAttr.cpu, base_value=2.6666) == 2.67


def test_cpu_modified(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20.005, eve_affectee_attr_id: 1.9444},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == 2.33
    api_mod = api_item.mods[eve_affectee_attr_id].find_by_affector_item(affector_item_id=api_item.id).one()
    assert api_mod.applied_val == approx(20.005)


def test_cpu_output(client, consts):
    assert get_value_simple(client=client, attr_id=consts.EveAttr.cpu_output, base_value=2.6666) == 2.67


def test_power(client, consts):
    assert get_value_simple(client=client, attr_id=consts.EveAttr.power, base_value=2.6666) == 2.67


def test_power_output(client, consts):
    assert get_value_simple(client=client, attr_id=consts.EveAttr.power_output, base_value=2.6666) == 2.67


def test_other(client):
    assert get_value_simple(client=client, attr_id=Default, base_value=2.6666) == approx(2.6666)
