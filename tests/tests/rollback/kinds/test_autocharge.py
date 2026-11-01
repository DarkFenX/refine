from fw import approx, check_no_field
from tests.rollback.kinds import rollback_batch, setup_mod_basics


def test_change(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_autocharge,
        cat_id=consts.EveEffCat.active)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        mod_info=[eve_basic_info.tgt_mod])
    eve_charge_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_charge_id},
        eff_ids=[eve_autocharge_effect_id],
        defeff_id=eve_autocharge_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id])
    api_autocharge = api_fighter.autocharges[eve_autocharge_effect_id]
    # Verification
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_autocharge(
            item_id=api_autocharge.id,
            state=False,
            effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    api_autocharge.update()
    assert api_autocharge.state is True
    with check_no_field():
        api_autocharge.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
