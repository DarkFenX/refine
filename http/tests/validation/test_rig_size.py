from tests import approx, check_no_field


def test_ship(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig1_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_rig2_id = client.mk_eve_item(attrs={eve_attr_id: 3})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.mismatches == {api_rig1.id: 1}
    # Action
    api_rig1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_struct(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig1_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_rig2_id = client.mk_eve_item(attrs={eve_attr_id: 3})
    eve_struct_id = client.mk_eve_struct(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.mismatches == {api_rig1.id: 1}
    # Action
    api_rig1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_rounding(client, consts):
    # Unrealistic scenario - EVE rig size has only integer value. But here we check that no rounding
    # happens, just straight comparison
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig1_id = client.mk_eve_item(attrs={eve_attr_id: 1.2})
    eve_rig2_id = client.mk_eve_item(attrs={eve_attr_id: 2.9})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 2.95})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 2.95
    assert api_val.details.rig_size.mismatches == {api_rig1.id: 1.2, api_rig2.id: 2.9}


def test_modified(client, consts):
    # Unrealistic scenario since EVE rig size is never modified, we check that unmodified values are
    # taken as detail of implementation
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_rig1_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_rig2_id = client.mk_eve_item(attrs={eve_attr_id: 3})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod1, eve_mod2])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 2}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    assert api_ship.update().attrs[eve_attr_id].extra == approx(2)
    assert api_rig1.update().attrs[eve_attr_id].extra == approx(2)
    assert api_rig2.update().attrs[eve_attr_id].extra == approx(2)
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.mismatches == {api_rig1.id: 1}


def test_ship_absent(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_ship_not_loaded(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_rig_not_loaded(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_no_attr(client, consts):
    # Nothing changes when there is no attribute
    eve_attr_id = consts.EveAttr.rig_size
    eve_rig1_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_rig2_id = client.mk_eve_item(attrs={eve_attr_id: 3})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.mismatches == {api_rig1.id: 1}
    # Action
    api_rig1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_state(client, consts):
    # Disabled rigs are still a subject for check
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id, state=False)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.mismatches == {api_rig.id: 1}


def test_other_item_type(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
