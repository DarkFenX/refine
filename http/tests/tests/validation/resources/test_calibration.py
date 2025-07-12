from tests import approx, check_no_field
from tests.fw.api import StatsFitOptions, ValOptions


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.max == approx(125)
    assert api_val.details.calibration.users == {api_rig.id: approx(150)}


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_rig2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.max == approx(125)
    assert api_val.details.calibration.users == {api_rig1.id: approx(50), api_rig2.id: approx(100)}


def test_fail_multiple_struct(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_rig2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.max == approx(125)
    assert api_val.details.calibration.users == {api_rig1.id: approx(50), api_rig2.id: approx(100)}


def test_equal(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(150))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_rig2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_rig3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10})
    eve_rig4_id = client.mk_eve_item(attrs={eve_use_attr_id: 0})
    eve_rig5_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=(True, [api_rig1.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(250), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=(True, [api_rig1.id])))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(250)
    assert api_val.details.calibration.max == approx(125)
    assert api_val.details.calibration.users == {api_rig2.id: 100}
    api_val = api_fit.validate(options=ValOptions(calibration=(True, [api_rig2.id])))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(250)
    assert api_val.details.calibration.max == approx(125)
    assert api_val.details.calibration.users == {api_rig1.id: 150}
    api_val = api_fit.validate(options=ValOptions(calibration=(True, [api_rig1.id, api_rig2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(calibration=(True, [api_rig1.id, api_other.id, api_rig2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_rig3 = api_fit.add_rig(type_id=eve_rig3_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(240), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=(True, [api_rig1.id, api_rig2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_rig3.remove()
    api_rig4 = api_fit.add_rig(type_id=eve_rig4_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(250), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=(True, [api_rig1.id, api_rig2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_rig4.remove()
    api_rig5 = api_fit.add_rig(type_id=eve_rig5_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(250.5), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=(True, [api_rig1.id, api_rig2.id])))
    assert api_val.passed is False
    assert api_val.details.calibration.used == 250.5
    assert api_val.details.calibration.max == 125
    assert api_val.details.calibration.users == {api_rig5.id: 0.5}


def test_modified_use(client, consts):
    # Calibration use is never modified, so the lib just uses unmodified attributes for faster
    # access to the attr value
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    assert api_rig.update().attrs[eve_use_attr_id].extra == approx(75)
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.max == approx(125)
    assert api_val.details.calibration.users == {api_rig.id: approx(150)}
    # Action
    api_implant.remove()
    # Verification
    assert api_rig.update().attrs[eve_use_attr_id].extra == approx(150)
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.max == approx(125)
    assert api_val.details.calibration.users == {api_rig.id: approx(150)}


def test_modified_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 120})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == approx(120)
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(120))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.max == approx(120)
    assert api_val.details.calibration.users == {api_rig.id: approx(150)}
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == approx(180)
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(180))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_rounding(client, consts):
    # Calibration shouldn't have its sum or individual values rounded
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.002})
    eve_rig2_id = client.mk_eve_item(attrs={eve_use_attr_id: 5.227})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5.223})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(5.229), approx(5.223))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(5.229)
    assert api_val.details.calibration.max == approx(5.223)
    assert api_val.details.calibration.users == {api_rig1.id: approx(0.002), api_rig2.id: approx(5.227)}


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(5), None)
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(5)
    assert api_val.details.calibration.max is None
    assert api_val.details.calibration.users == {api_rig.id: approx(5)}


def test_not_loaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(5), None)
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(5)
    assert api_val.details.calibration.max is None
    assert api_val.details.calibration.users == {api_rig.id: approx(5)}


def test_not_loaded_user(client, consts):
    # Just check that nothing crashes, not loaded items are not supposed to even be registered
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_rig_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(0), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0})
    eve_rig2_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_rig3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    api_fit.add_rig(type_id=eve_rig3_id)
    # Verification - items with negative and 0 use are not exposed
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(140), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(140)
    assert api_val.details.calibration.max == approx(125)
    assert api_val.details.calibration.users == {api_rig2.id: approx(150)}


def test_no_value_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_rig2_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.max == approx(125)
    assert api_val.details.calibration.users == {api_rig1.id: approx(150)}


def test_no_value_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(0))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.max == approx(0)
    assert api_val.details.calibration.users == {api_rig.id: approx(150)}


def test_criterion_rig_state(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id, state=False)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(0), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_rig.change_rig(state=True)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is False
    assert api_val.details.calibration.used == approx(150)
    assert api_val.details.calibration.max == approx(125)
    assert api_val.details.calibration.users == {api_rig.id: approx(150)}
    # Action
    api_rig.change_rig(state=False)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(0), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    # Validation applies only to rigs
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_item_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125, eve_use_attr_id: 150})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_item_id, eve_use_attr_id: 150},
        eff_ids=[eve_autocharge_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_item_id)
    api_fit.set_character(type_id=eve_item_id)
    api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item_id)
    api_fit.add_service(type_id=eve_item_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_stats = api_fit.get_stats(options=StatsFitOptions(calibration=True))
    assert api_stats.calibration == (approx(0), approx(125))
    api_val = api_fit.validate(options=ValOptions(calibration=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
