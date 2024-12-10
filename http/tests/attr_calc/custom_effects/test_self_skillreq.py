from tests import approx, check_no_field


def test_missile_launcher_rof(client, consts):
    # All missile specialization skills use the same effect, which has no modifiers defined in EVE
    # data files. In EVE, they affect missile launchers which have skill which carries the effect
    # as their skill requirement
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rof_bonus)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.self_rof)
    eve_skill1_id = client.mk_eve_item(attrs={eve_affector_attr_id: -20}, eff_ids=[eve_effect_id])
    eve_skill2_id = client.mk_eve_item()
    eve_launcher1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill1_id: 5})
    eve_launcher2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill2_id: 5})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_skill1 = api_fit.add_skill(type_id=eve_skill1_id, level=5)
    api_fit.add_skill(type_id=eve_skill2_id, level=5)
    api_launcher1 = api_fit.add_mod(type_id=eve_launcher1_id)
    api_launcher2 = api_fit.add_mod(type_id=eve_launcher2_id)
    # Verification
    api_launcher1 = api_launcher1.update()
    assert api_launcher1.attrs[eve_affectee_attr_id].dogma == approx(4)
    api_mod = api_launcher1.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_skill1.id).one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(-20)
    api_launcher2 = api_launcher2.update()
    assert api_launcher2.attrs[eve_affectee_attr_id].dogma == approx(5)
    with check_no_field():
        api_launcher2.mods  # pylint: disable=W0104


def test_missile_damage_em(client, consts):
    # All basic missile skills boost damage of all missiles which need this skill
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult_bonus)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.missile_em_dmg_bonus)
    eve_skill1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_skill2_id = client.mk_eve_item()
    eve_launcher_id = client.mk_eve_item()
    eve_charge1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill1_id: 5})
    eve_charge2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill1 = api_fit.add_skill(type_id=eve_skill1_id, level=5)
    api_fit.add_skill(type_id=eve_skill2_id, level=5)
    api_launcher1 = api_fit.add_mod(type_id=eve_launcher_id, charge_type_id=eve_charge1_id)
    api_launcher2 = api_fit.add_mod(type_id=eve_launcher_id, charge_type_id=eve_charge2_id)
    # Verification
    api_launcher1 = api_launcher1.update()
    assert api_launcher1.charge.attrs[eve_affectee_attr_id].dogma == approx(6)
    api_mod = api_launcher1.charge.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_skill1.id).one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    api_launcher2 = api_launcher2.update()
    api_charge2 = api_launcher2.charge
    assert api_charge2.attrs[eve_affectee_attr_id].dogma == approx(5)
    with check_no_field():
        api_charge2.mods  # pylint: disable=W0104


def test_missile_damage_therm(client, consts):
    # All basic missile skills boost damage of all missiles which need this skill
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult_bonus)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.missile_therm_dmg_bonus)
    eve_skill1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_skill2_id = client.mk_eve_item()
    eve_launcher_id = client.mk_eve_item()
    eve_charge1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill1_id: 5})
    eve_charge2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill1 = api_fit.add_skill(type_id=eve_skill1_id, level=5)
    api_fit.add_skill(type_id=eve_skill2_id, level=5)
    api_launcher1 = api_fit.add_mod(type_id=eve_launcher_id, charge_type_id=eve_charge1_id)
    api_launcher2 = api_fit.add_mod(type_id=eve_launcher_id, charge_type_id=eve_charge2_id)
    # Verification
    api_launcher1 = api_launcher1.update()
    assert api_launcher1.charge.attrs[eve_affectee_attr_id].dogma == approx(6)
    api_mod = api_launcher1.charge.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_skill1.id).one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    api_launcher2 = api_launcher2.update()
    api_charge2 = api_launcher2.charge
    assert api_charge2.attrs[eve_affectee_attr_id].dogma == approx(5)
    with check_no_field():
        api_charge2.mods  # pylint: disable=W0104


def test_missile_damage_kin(client, consts):
    # All basic missile skills boost damage of all missiles which need this skill
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult_bonus)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.missile_kin_dmg_bonus)
    eve_skill1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_skill2_id = client.mk_eve_item()
    eve_launcher_id = client.mk_eve_item()
    eve_charge1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill1_id: 5})
    eve_charge2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill1 = api_fit.add_skill(type_id=eve_skill1_id, level=5)
    api_fit.add_skill(type_id=eve_skill2_id, level=5)
    api_launcher1 = api_fit.add_mod(type_id=eve_launcher_id, charge_type_id=eve_charge1_id)
    api_launcher2 = api_fit.add_mod(type_id=eve_launcher_id, charge_type_id=eve_charge2_id)
    # Verification
    api_launcher1 = api_launcher1.update()
    assert api_launcher1.charge.attrs[eve_affectee_attr_id].dogma == approx(6)
    api_mod = api_launcher1.charge.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_skill1.id).one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    api_launcher2 = api_launcher2.update()
    api_charge2 = api_launcher2.charge
    assert api_charge2.attrs[eve_affectee_attr_id].dogma == approx(5)
    with check_no_field():
        api_charge2.mods  # pylint: disable=W0104


def test_missile_damage_expl(client, consts):
    # All basic missile skills boost damage of all missiles which need this skill
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult_bonus)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.missile_expl_dmg_bonus)
    eve_skill1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_skill2_id = client.mk_eve_item()
    eve_launcher_id = client.mk_eve_item()
    eve_charge1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill1_id: 5})
    eve_charge2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill1 = api_fit.add_skill(type_id=eve_skill1_id, level=5)
    api_fit.add_skill(type_id=eve_skill2_id, level=5)
    api_launcher1 = api_fit.add_mod(type_id=eve_launcher_id, charge_type_id=eve_charge1_id)
    api_launcher2 = api_fit.add_mod(type_id=eve_launcher_id, charge_type_id=eve_charge2_id)
    # Verification
    api_launcher1 = api_launcher1.update()
    assert api_launcher1.charge.attrs[eve_affectee_attr_id].dogma == approx(6)
    api_mod = api_launcher1.charge.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_skill1.id).one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    api_launcher2 = api_launcher2.update()
    api_charge2 = api_launcher2.charge
    assert api_charge2.attrs[eve_affectee_attr_id].dogma == approx(5)
    with check_no_field():
        api_charge2.mods  # pylint: disable=W0104


def test_drone_dmg(client, consts):
    # Almost all the drone skills have the same effect as well
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult_bonus)
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.drone_dmg_bonus)
    eve_skill1_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_skill2_id = client.mk_eve_item()
    eve_drone1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill1_id: 5})
    eve_drone2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5}, srqs={eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill1 = api_fit.add_skill(type_id=eve_skill1_id, level=5)
    api_fit.add_skill(type_id=eve_skill2_id, level=5)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id)
    # Verification
    api_drone1 = api_drone1.update()
    assert api_drone1.attrs[eve_affectee_attr_id].dogma == approx(6)
    api_mod = api_drone1.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_skill1.id).one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    api_drone2 = api_drone2.update()
    assert api_drone2.attrs[eve_affectee_attr_id].dogma == approx(5)
    with check_no_field():
        api_drone2.mods  # pylint: disable=W0104
