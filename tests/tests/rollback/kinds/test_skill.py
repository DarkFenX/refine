from fw import approx, check_no_field
from tests.rollback.kinds import rollback_batch, setup_mod_basics


def setup_skill_effects(*, client, consts, basic_info):
    eve_level_attr_id = client.mk_eve_attr(id_=consts.EveAttr.skill_level)
    eve_level_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_level_attr_id,
        affectee_attr_id=basic_info.affectee_attr_id)
    eve_level_effect_id = client.mk_eve_effect(mod_info=[eve_level_mod])
    eve_effect_id = client.mk_eve_effect(mod_info=[basic_info.ship_mod])
    return eve_level_attr_id, eve_level_effect_id, eve_effect_id


def test_add(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_level_attr_id, eve_level_effect_id, eve_effect_id = setup_skill_effects(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_skill_id = client.mk_eve_item(
        attrs={eve_level_attr_id: 0, eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_level_effect_id, eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.add_skill(fit_id=api_fit.id, type_id=eve_skill_id, level=5)
    with check_no_field():
        api_fit.update().skills  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(100)


def test_remove(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_level_attr_id, eve_level_effect_id, eve_effect_id = setup_skill_effects(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_skill_id = client.mk_eve_item(
        attrs={eve_level_attr_id: 0, eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_level_effect_id, eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=5)
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.remove_item(item_id=api_skill.id)
    assert api_fit.update().skills.keys() == {api_skill.id}
    assert api_skill.update().level == 5
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)


def test_change(client, consts):
    eve_basic_info = setup_mod_basics(client=client, consts=consts)
    eve_level_attr_id, eve_level_effect_id, eve_effect_id = setup_skill_effects(
        client=client, consts=consts, basic_info=eve_basic_info)
    eve_skill1_id = client.mk_eve_item(
        attrs={eve_level_attr_id: 0, eve_basic_info.affector_attr_id: 20},
        eff_ids=[eve_level_effect_id, eve_effect_id])
    eve_skill2_id = client.mk_eve_item(
        attrs={eve_level_attr_id: 0, eve_basic_info.affector_attr_id: 50},
        eff_ids=[eve_level_effect_id, eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=5)
    # Verification
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)
    # Action & verification
    with rollback_batch(api_sol=api_sol) as api_sol_batch:
        api_sol_batch.change_skill(
            item_id=api_skill.id,
            type_id=eve_skill2_id,
            level=1,
            state=False,
            effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    api_skill.update()
    assert api_skill.type_id == eve_skill1_id
    assert api_skill.level == 5
    assert api_skill.state is True
    with check_no_field():
        api_skill.effect_mode_overrides  # ruff:ignore[useless-expression]
    assert api_ship.update().attrs[eve_basic_info.affectee_attr_id].modified == approx(126)
