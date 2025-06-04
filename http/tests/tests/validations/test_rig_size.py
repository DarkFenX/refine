from tests import approx, check_no_field
from tests.fw.api import FitValOptions


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
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.rig_sizes == {api_rig1.id: 1}
    # Action
    api_rig1.remove()
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


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
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.rig_sizes == {api_rig1.id: 1}
    # Action
    api_rig1.remove()
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig1_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_rig2_id = client.mk_eve_item(attrs={eve_attr_id: 3})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig1_id)
    api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_size=(True, [api_rig1.id])))
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.rig_sizes == {api_rig2.id: 1}
    api_val = api_fit.validate(options=FitValOptions(rig_size=(True, [api_rig2.id])))
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.rig_sizes == {api_rig1.id: 1}
    api_val = api_fit.validate(options=FitValOptions(rig_size=(True, [api_rig1.id, api_rig2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


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
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 2.95
    assert api_val.details.rig_size.rig_sizes == {api_rig1.id: 1.2, api_rig2.id: 2.9}


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
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.rig_sizes == {api_rig1.id: 1}


def test_ship_absent(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_non_positive(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig1_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_rig2_id = client.mk_eve_item(attrs={eve_attr_id: -3})
    eve_ship1_id = client.mk_eve_ship(attrs={eve_attr_id: 0})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_attr_id: -3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 0
    assert api_val.details.rig_size.rig_sizes == {api_rig2.id: -3}
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == -3
    assert api_val.details.rig_size.rig_sizes == {api_rig1.id: 0}


def test_no_value_ship(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.mk_eve_item(attrs={eve_attr_id: 2})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_value_rig(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 2
    assert api_val.details.rig_size.rig_sizes == {api_rig.id: None}


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
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


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
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


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
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.rig_sizes == {api_rig.id: 1}


def test_criterion_item_kind(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_ship_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_autocharge_id = client.mk_eve_item(attrs={eve_attr_id: 2})
    eve_booster_id = client.mk_eve_item(attrs={eve_attr_id: 3})
    eve_character_id = client.mk_eve_item(attrs={eve_attr_id: 4})
    eve_charge_id = client.mk_eve_item(attrs={eve_attr_id: 5})
    eve_drone_id = client.mk_eve_item(attrs={eve_attr_id: 6})
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_autocharge_id, eve_attr_id: 7},
        eff_ids=[eve_autocharge_effect_id])
    eve_fw_effect_id = client.mk_eve_item(attrs={eve_attr_id: 8})
    eve_implant_id = client.mk_eve_item(attrs={eve_attr_id: 9})
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 10})
    eve_service_id = client.mk_eve_item(attrs={eve_attr_id: 11})
    eve_skill_id = client.mk_eve_item(attrs={eve_attr_id: 12})
    eve_stance_id = client.mk_eve_item(attrs={eve_attr_id: 13})
    eve_subsystem_id = client.mk_eve_item(attrs={eve_attr_id: 14})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster_id)
    api_fit.set_character(type_id=eve_character_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload, charge_type_id=eve_charge_id)
    api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_skill(type_id=eve_skill_id, level=5)
    api_fit.set_stance(type_id=eve_stance_id)
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=FitValOptions(rig_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
