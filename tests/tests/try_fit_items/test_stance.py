from fw import check_no_field
from fw.api import ValOptions


def test_replaced(client, consts):
    eve_ship_id = client.mk_eve_ship(id_=consts.EveItem.confessor)
    eve_stance1_id = client.mk_eve_item(
        id_=consts.EveItem.svipul_defense_mode,
        cat_id=consts.EveItemCat.module,
        grp_id=consts.EveItemGrp.ship_modifier)
    eve_stance2_id = client.mk_eve_item(
        id_=consts.EveItem.confessor_defense_mode,
        cat_id=consts.EveItemCat.module,
        grp_id=consts.EveItemGrp.ship_modifier)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_stance = api_fit.set_stance(type_id=eve_stance1_id)
    # Verification - tried stances replace the set one, so its failure does not affect them, and it
    # is put back afterwards
    api_fit.check_try_fit_items(
        val_options=ValOptions(ship_limit=True),
        passing=[eve_stance2_id],
        failing=[eve_stance1_id])
    assert api_fit.update().stance.id == api_stance.id
    assert api_stance.update().type_id == eve_stance1_id


def test_restored(client, consts):
    eve_effect_id = client.mk_eve_effect()
    eve_stance1_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        grp_id=consts.EveItemGrp.ship_modifier,
        eff_ids=[eve_effect_id])
    eve_stance2_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, grp_id=consts.EveItemGrp.ship_modifier)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_stance = api_fit.set_stance(type_id=eve_stance1_id)
    api_stance.change_stance(state=False, effect_modes={eve_effect_id: consts.ApiEffMode.force_run})
    # Verification - stance which is put back after trying stances keeps its item ID and settings
    api_fit.check_try_fit_items(val_options=ValOptions(), passing=[eve_stance2_id], failing=[])
    assert api_fit.update().stance.id == api_stance.id
    api_stance.update()
    assert api_stance.type_id == eve_stance1_id
    assert api_stance.state is False
    assert api_stance.effects[eve_effect_id].mode == consts.ApiEffMode.force_run


def test_no_stance(client, consts):
    eve_stance_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, grp_id=consts.EveItemGrp.ship_modifier)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification - fit which had no stance has none after stances are tried
    api_fit.check_try_fit_items(val_options=ValOptions(), passing=[eve_stance_id], failing=[])
    api_fit.update()
    with check_no_field():
        api_fit.stance  # ruff:ignore[useless-expression]
