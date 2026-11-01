from fw import approx, check_no_field
from tests.rollback.kinds import rollback_batch, setup_mod_basics


def setup_autocharge_effects(*, client, consts, basic_info):
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_autocharge,
        cat_id=consts.EveEffCat.active)
    eve_charge_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        mod_info=[basic_info.tgt_mod])
    return eve_autocharge_attr_id, eve_autocharge_effect_id, eve_charge_effect_id


def test_add(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_autocharge_attr_id, eve_autocharge_effect_id, eve_charge_effect_id = setup_autocharge_effects(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
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
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.add_fighter(
            fit_id=api_fit.id,
            type_id=eve_fighter_id,
            state=consts.ApiMinionState.engaging,
            proj_item_ids=[api_tgt_ship.id])
    with check_no_field():
        api_fit.update().fighters  # ruff:ignore[useless-expression]
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(100)


def test_remove(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_autocharge_attr_id, eve_autocharge_effect_id, eve_charge_effect_id = setup_autocharge_effects(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
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
    api_autocharge_id = api_fighter.autocharges[eve_autocharge_effect_id].id
    # Verification
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.remove_item(item_id=api_fighter.id)
    assert api_fit.update().fighters.keys() == {api_fighter.id}
    api_fighter.update()
    assert api_fighter.projs.keys() == {api_tgt_ship.id}
    assert api_fighter.autocharges[eve_autocharge_effect_id].id == api_autocharge_id
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_change(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_autocharge_attr_id, eve_autocharge_effect_id, eve_charge_effect_id = setup_autocharge_effects(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
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
    # Autocharge effect has no range attributes, so it applies only when fighter is at its projectee
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        count_override=3,
        proj_item_ids=[api_tgt_ship.id])
    # Verification
    assert api_fighter.update().projs[api_tgt_ship.id] == (approx(0), approx(0))
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_fighter(
            item_id=api_fighter.id,
            state=consts.ApiMinionState.in_space,
            count_override=None,
            rearm_minion_override=consts.ApiRearmMinion.disabled,
            coordinates=(0, 5000, 0),
            movement=(90, 0, 1),
            effect_modes={eve_autocharge_effect_id: consts.ApiEffMode.force_stop})
    api_fighter.update()
    assert api_fighter.state == consts.ApiMinionState.engaging
    assert api_fighter.count_override == 3
    with check_no_field():
        api_fighter.rearm_minion_override  # ruff:ignore[useless-expression]
    assert api_fighter.coordinates == (0, 0, 0)
    assert api_fighter.movement == (0, 0, 0)
    with check_no_field():
        api_fighter.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert api_fighter.projs[api_tgt_ship.id] == (approx(0), approx(0))
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_change_projs(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_autocharge_attr_id, eve_autocharge_effect_id, eve_charge_effect_id = setup_autocharge_effects(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_charge_id},
        eff_ids=[eve_autocharge_effect_id],
        defeff_id=eve_autocharge_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit1 = api_sol.create_fit()
    api_tgt_ship1 = api_tgt_fit1.set_ship(type_id=eve_ship_id)
    api_tgt_fit2 = api_sol.create_fit()
    api_tgt_ship2 = api_tgt_fit2.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship1.id])
    # Verification
    assert api_tgt_ship1.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_fighter(
            item_id=api_fighter.id,
            rm_proj_item_ids=[api_tgt_ship1.id],
            add_proj_item_ids=[api_tgt_ship2.id])
    assert api_fighter.update().projs.keys() == {api_tgt_ship1.id}
    assert api_tgt_ship1.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    assert api_tgt_ship2.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(100)


def test_change_type(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_autocharge_attr_id, eve_autocharge_effect_id, eve_charge_effect_id = setup_autocharge_effects(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_charge1_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
    eve_charge2_id = client.mk_eve_item(
        attrs={eve_basic_info.affector_attr_id: 50},
        eff_ids=[eve_charge_effect_id],
        defeff_id=eve_charge_effect_id)
    eve_fighter1_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_charge1_id},
        eff_ids=[eve_autocharge_effect_id],
        defeff_id=eve_autocharge_effect_id)
    eve_fighter2_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_charge2_id},
        eff_ids=[eve_autocharge_effect_id],
        defeff_id=eve_autocharge_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter1_id,
        state=consts.ApiMinionState.engaging,
        proj_item_ids=[api_tgt_ship.id])
    api_autocharge = api_fighter.autocharges[eve_autocharge_effect_id]
    # Verification
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)
    # Action & verification - type change replaces the autocharge
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_fighter(item_id=api_fighter.id, type_id=eve_fighter2_id)
    api_fighter.update()
    assert api_fighter.type_id == eve_fighter1_id
    assert len(api_fighter.autocharges) == 1
    assert api_fighter.autocharges[eve_autocharge_effect_id].id == api_autocharge.id
    assert api_autocharge.update().type_id == eve_charge1_id
    assert api_tgt_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(120)


def test_change_abilities(client, consts):
    eve_affector_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_ab_speed_bonus)
    eve_affectee_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_affector_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_mwd_sig_radius_bonus)
    eve_affectee_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_primary_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_ab, cat_id=consts.EveEffCat.active)
    eve_secondary_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_mwd, cat_id=consts.EveEffCat.active)
    eve_primary_abil_id = client.mk_eve_abil(id_=consts.EveAbil.ab)
    eve_secondary_abil_id = client.mk_eve_abil(id_=consts.EveAbil.mwd)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={
            eve_affector_attr1_id: 400, eve_affectee_attr1_id: 1017.5,
            eve_affector_attr2_id: 150, eve_affectee_attr2_id: 100},
        eff_ids=[eve_primary_effect_id, eve_secondary_effect_id],
        defeff_id=eve_primary_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_primary_abil_id), client.mk_eve_item_abil(id_=eve_secondary_abil_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fighter.update()
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(5087.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_fighter(
            item_id=api_fighter.id,
            abilities={eve_primary_abil_id: False, eve_secondary_abil_id: True})
    api_fighter.update()
    assert api_fighter.abilities[eve_primary_abil_id].state is True
    assert api_fighter.abilities[eve_secondary_abil_id].state is False
    assert api_fighter.attrs[eve_affectee_attr1_id].modified == approx(5087.5)
    assert api_fighter.attrs[eve_affectee_attr2_id].modified == approx(100)
