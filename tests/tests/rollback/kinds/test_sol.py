from fw import Spool, approx
from tests.rollback.kinds import rollback_batch


def test_change(client, consts):
    eve_res_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_max_dmg_resonance, def_val=1)
    eve_res_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_em_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_therm_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_kin_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_expl_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_shift_attr_id = client.mk_eve_attr(id_=consts.EveAttr.resist_shift_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.duration)
    eve_rah_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.adaptive_armor_hardener,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_rah_id = client.mk_eve_item(
        attrs={
            eve_res_em_attr_id: 0.85,
            eve_res_therm_attr_id: 0.85,
            eve_res_kin_attr_id: 0.85,
            eve_res_expl_attr_id: 0.85,
            eve_res_shift_attr_id: 6,
            eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_rah_effect_id],
        defeff_id=eve_rah_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_res_em_attr_id: 0.5,
        eve_res_therm_attr_id: 0.65,
        eve_res_kin_attr_id: 0.75,
        eve_res_expl_attr_id: 0.9})
    eve_drone_id = client.mk_eve_drone()
    eve_fighter_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol(
        sec_zone=consts.ApiSecZone.hisec,
        default_incoming_dps=(1, 1, 0, 0),
        default_spool=Spool.cycles_to_api(count=0),
        default_npc_prop=consts.ApiNpcProp.cruise,
        default_optional_reloads=consts.ApiOptionalReload.disabled,
        default_rearm_minions=consts.ApiRearmMinion.disabled)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_res_em_attr_id].modified == approx(0.35)
    assert api_ship.attrs[eve_res_therm_attr_id].modified == approx(0.455)
    assert api_ship.attrs[eve_res_kin_attr_id].modified == approx(0.75)
    assert api_ship.attrs[eve_res_expl_attr_id].modified == approx(0.9)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_sol(
            sec_zone=consts.ApiSecZone.nullsec,
            default_incoming_dps=(0, 0, 1, 1),
            default_spool=Spool.cycles_to_api(count=1),
            default_npc_prop=consts.ApiNpcProp.chase,
            default_optional_reloads=consts.ApiOptionalReload.on_empty,
            default_rearm_minions=consts.ApiRearmMinion.on_first_empty)
    api_sol.update()
    assert api_sol.sec_zone == consts.ApiSecZone.hisec
    assert api_sol.default_incoming_dps == (1, 1, 0, 0)
    assert api_sol.default_spool == Spool.cycles_to_api(count=0)
    assert api_rah.update().optional_reload == consts.ApiOptionalReload.disabled
    assert api_drone.update().npc_prop == consts.ApiNpcProp.cruise
    assert api_fighter.update().rearm_minion == consts.ApiRearmMinion.disabled
    api_rah.update()
    assert api_rah.attrs[eve_res_em_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_res_therm_attr_id].modified == approx(0.7)
    assert api_rah.attrs[eve_res_kin_attr_id].modified == approx(1)
    assert api_rah.attrs[eve_res_expl_attr_id].modified == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_res_em_attr_id].modified == approx(0.35)
    assert api_ship.attrs[eve_res_therm_attr_id].modified == approx(0.455)
    assert api_ship.attrs[eve_res_kin_attr_id].modified == approx(0.75)
    assert api_ship.attrs[eve_res_expl_attr_id].modified == approx(0.9)
