from tests import approx, check_no_field


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(125)
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(150)


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50}, eff_ids=[eve_effect_id])
    eve_rig2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(125)
    assert len(api_val.details.calibration.users) == 2
    assert api_val.details.calibration.users[api_rig1.id] == approx(50)
    assert api_val.details.calibration.users[api_rig2.id] == approx(100)


def test_fail_multiple_struct(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50}, eff_ids=[eve_effect_id])
    eve_rig2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100}, eff_ids=[eve_effect_id])
    eve_struct_id = client.mk_eve_struct(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(125)
    assert len(api_val.details.calibration.users) == 2
    assert api_val.details.calibration.users[api_rig1.id] == approx(50)
    assert api_val.details.calibration.users[api_rig2.id] == approx(100)


def test_equal(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_modified_use(client, consts):
    # Calibration use is never modified, so the lib just uses unmodified attributes for speed
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(125)
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(150)
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(125)
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(150)


def test_modified_output(client, consts):
    # Calibration output is never modified, so the lib just uses unmodified attributes for speed
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_output_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 120})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(120)
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(150)
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(120)
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(150)


def test_rounding(client, consts):
    # Calibration shouldn't have its sum or individual values rounded
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.002}, eff_ids=[eve_effect_id])
    eve_rig2_id = client.mk_eve_item(attrs={eve_use_attr_id: 5.227}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 5.223})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(5.229)
    assert api_val.details.calibration.output == approx(5.223)
    assert len(api_val.details.calibration.users) == 2
    assert api_val.details.calibration.users[api_rig1.id] == approx(0.002)
    assert api_val.details.calibration.users[api_rig2.id] == approx(5.227)


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 5}, eff_ids=[eve_effect_id])
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(5)
    assert api_val.details.calibration.output is None
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(5)


def test_unloaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 5}, eff_ids=[eve_effect_id])
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(5)
    assert api_val.details.calibration.output is None
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(5)


def test_unloaded_user(client, consts):
    # Just check that nothing crashes, unloaded items are not supposed to even be registered
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_rig_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_rig2_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_rig3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    api_fit.add_rig(type_id=eve_rig3_id)
    # Verification - items with negative and 0 use are not exposed
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(140)
    assert api_val.details.calibration.output == approx(125)
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig2.id] == approx(150)


def test_no_attr_use(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_use_attr_id = consts.EveAttr.upgrade_cost
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(125)
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(150)


def test_no_attr_output(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = consts.EveAttr.upgrade_capacity
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(125)
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(150)


def test_criterion_state(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id, state=False)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_rig.change_rig(state=True)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(125)
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(150)
    # Action
    api_rig.change_rig(state=False)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_criterion_effect(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(125)
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(150)
    # Action
    api_rig.change_rig(effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_rig.change_rig(state=True, effect_modes={eve_effect_id: consts.ApiEffMode.full_compliance})
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.output == approx(125)
    assert len(api_val.details.calibration.users) == 1
    assert api_val.details.calibration.users[api_rig.id] == approx(150)
    # Action
    api_rig.change_rig(effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_criterion_rig(client, consts):
    # Validation applies only to rigs
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot, cat_id=consts.EveEffCat.passive)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_rig_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.calibration])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
