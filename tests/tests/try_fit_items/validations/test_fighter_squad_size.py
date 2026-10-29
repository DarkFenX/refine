from fw.api import ValOptions


def test_known_failures(client, consts):
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter_id, count_override=10)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter_id, count_override=11)
    # Verification - fighters are tried without squad size override
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_squad_size=[api_fighter1.id]),
        passing=[],
        failing=[eve_fighter_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_squad_size=[api_fighter2.id]),
        passing=[],
        failing=[eve_fighter_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(fighter_squad_size=[api_fighter1.id, api_fighter2.id]),
        passing=[eve_fighter_id],
        failing=[])
