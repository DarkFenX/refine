
def test_only_invalid_char(client):
    client.mk_eve_data(alias='*(((@')
    client.create_sources(
        status_code=403,
        json_predicate={
            'code': 'SRC-004',
            'message': '"*(((@" cannot be used as a source alias: alias is empty after pruning'})


def test_only_invalid_outer_char(client):
    client.mk_eve_data(alias='_..-')
    client.create_sources(
        status_code=403,
        json_predicate={
            'code': 'SRC-004',
            'message': '"_..-" cannot be used as a source alias: alias is empty after pruning'})


def test_capital_letters(client):
    eve_data = client.mk_eve_data(alias='TranQuilitY-53')
    client.create_sources()
    api_sol = client.create_sol(data=eve_data)
    assert api_sol.src_alias == 'tranquility-53'


def test_strip_leading(client):
    eve_data = client.mk_eve_data(alias='__.tranquility.22')
    client.create_sources()
    api_sol = client.create_sol(data=eve_data)
    assert api_sol.src_alias == 'tranquility.22'


def test_strip_trailing(client):
    eve_data = client.mk_eve_data(alias='test-tranquility._-_')
    client.create_sources()
    api_sol = client.create_sol(data=eve_data)
    assert api_sol.src_alias == 'test-tranquility'


def test_length_limit(client):
    eve_data = client.mk_eve_data(alias='a' * 150)
    client.create_sources()
    api_sol = client.create_sol(data=eve_data)
    assert api_sol.src_alias == 'a' * 100
