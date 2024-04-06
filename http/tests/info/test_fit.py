# Here we check availability of info of various items via fit info endpoint

def test_char(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.set_char(type_id=eve_item.id)
    assert api_fit.update().character.id == api_item.id


def test_skill(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_skill(type_id=eve_item.id, level=1)
    api_fit.update()
    assert len(api_fit.skills) == 1
    assert api_fit.skills[0].id == api_item.id


def test_implant(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    api_fit.update()
    assert len(api_fit.implants) == 1
    assert api_fit.implants[0].id == api_item.id


def test_booster(client):
    eve_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_booster(type_id=eve_item.id)
    api_fit.update()
    assert len(api_fit.boosters) == 1
    assert api_fit.boosters[0].id == api_item.id
