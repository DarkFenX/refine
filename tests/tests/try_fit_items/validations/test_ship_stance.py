from fw.api import ValOptions


def test_known_failures(client, consts):
    eve_stance_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, grp_id=consts.EveItemGrp.ship_modifier)
    eve_module_id = client.mk_eve_module()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_stance = api_fit.set_stance(type_id=eve_stance_id)
    # Verification - stances are not tried, and fit can have only one stance which can fail
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_stance=True),
        passing=[],
        failing=[eve_module_id, eve_stance_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_stance=[api_stance.id]),
        passing=[eve_module_id],
        failing=[eve_stance_id])
