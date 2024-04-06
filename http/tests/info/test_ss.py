# Here we check availability of info of various items via solar system info endpoint

from pytest import raises


def test_sw_effect(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_item = api_ss.add_sw_effect(type_id=eve_item.id)
    api_ss.update()
    assert len(api_ss.sw_effects) == 1
    assert api_ss.sw_effects[0].id == api_item.id
    api_item.remove()
    with raises(AttributeError):
        api_ss.update().sw_effects  # pylint: disable=W0106


def test_proj_effect(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_item = api_ss.add_proj_effect(type_id=eve_item.id)
    api_ss.update()
    assert len(api_ss.proj_effects) == 1
    assert api_ss.proj_effects[0].id == api_item.id
    api_item.remove()
    with raises(AttributeError):
        api_ss.update().proj_effects  # pylint: disable=W0106
