from fw.api import ValOptions


def test_ship(client, consts):
    eve_stance_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, grp_id=consts.EveItemGrp.ship_modifier)
    eve_t3d_id = client.mk_eve_ship(id_=consts.EveItem.confessor)
    eve_other_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_t3d_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_stance=True), passing=[eve_stance_id], failing=[])
    # Action
    api_fit.set_ship(type_id=eve_other_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_stance=True), passing=[], failing=[eve_stance_id])


def test_known_failures(client, consts):
    eve_stance_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, grp_id=consts.EveItemGrp.ship_modifier)
    eve_module_id = client.mk_eve_module()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_stance = api_fit.set_stance(type_id=eve_stance_id)
    # Verification - known failure of the set stance lets other items pass, but does not apply to
    # a stance which is tried in its place
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_stance=True),
        passing=[],
        failing=[eve_module_id, eve_stance_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_stance=[api_stance.id]),
        passing=[eve_module_id],
        failing=[eve_stance_id])


def test_no_ship(client, consts):
    eve_stance_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, grp_id=consts.EveItemGrp.ship_modifier)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_stance=True), passing=[], failing=[eve_stance_id])


def test_not_loaded_ship(client, consts):
    eve_t3d_id = consts.EveItem.confessor
    eve_other_id = client.alloc_item_id()
    eve_stance_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, grp_id=consts.EveItemGrp.ship_modifier)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_t3d_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_stance=True), passing=[eve_stance_id], failing=[])
    # Action
    api_fit.set_ship(type_id=eve_other_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(ship_stance=True), passing=[], failing=[eve_stance_id])
