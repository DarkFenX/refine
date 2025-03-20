from tests import approx, check_no_field
from tests.fw.api import ValOptions


def test_multiple(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: sorted([api_subsystem1.id, api_subsystem2.id])}
    # Action
    api_subsystem1.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_different_slots(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem1_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    eve_subsystem2_id = client.mk_eve_item(attrs={eve_slot_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem1_id)
    api_fit.add_subsystem(type_id=eve_subsystem2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_subsystem3 = api_fit.add_subsystem(type_id=eve_subsystem1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: sorted([api_subsystem1.id, api_subsystem3.id])}


def test_known_failures(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem1_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    eve_subsystem2_id = client.mk_eve_item(attrs={eve_slot_attr_id: 2})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_subsystem(type_id=eve_other_id)
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem1_id)
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem2_id)
    api_subsystem3 = api_fit.add_subsystem(type_id=eve_subsystem1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=(True, [api_subsystem1.id])))
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: [api_subsystem3.id]}
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=(True, [api_subsystem3.id])))
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: [api_subsystem1.id]}
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=(True, [api_subsystem1.id, api_subsystem3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        subsystem_slot_index=(True, [api_subsystem1.id, api_other.id, api_subsystem3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_subsystem4 = api_fit.add_subsystem(type_id=eve_subsystem2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=(True, [api_subsystem1.id, api_subsystem3.id])))
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {2: sorted([api_subsystem2.id, api_subsystem4.id])}


def test_rounding(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem1_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1.2})
    eve_subsystem2_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1.4})
    eve_subsystem3_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem1_id)
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem2_id)
    api_fit.add_subsystem(type_id=eve_subsystem3_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: sorted([api_subsystem1.id, api_subsystem2.id])}


def test_modified_index(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.mk_eve_item(attrs={eve_slot_attr_id: 2})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_slot_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 3}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    assert api_subsystem1.update().attrs[eve_slot_attr_id].extra == approx(2)
    assert api_subsystem2.update().attrs[eve_slot_attr_id].extra == approx(2)
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {2: sorted([api_subsystem1.id, api_subsystem2.id])}
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification - attribute is modified, but not for purposes of validation
    assert api_subsystem1.update().attrs[eve_slot_attr_id].extra == approx(3)
    assert api_subsystem2.update().attrs[eve_slot_attr_id].extra == approx(3)
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {2: sorted([api_subsystem1.id, api_subsystem2.id])}


def test_no_attr(client, consts):
    eve_slot_attr_id = consts.EveAttr.subsystem_slot
    eve_subsystem_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: sorted([api_subsystem1.id, api_subsystem2.id])}


def test_no_value(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.mk_eve_item()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_slot_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.alloc_item_id()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_slot_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_state(client, consts):
    # Disabled subsystems still compete for slots
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem_id, state=False)
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem_id, state=False)
    # Verification
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: sorted([api_subsystem1.id, api_subsystem2.id])}


def test_criterion_item_kind(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_item1_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    eve_item2_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_item1_id, eve_slot_attr_id: 1},
        eff_ids=[eve_autocharge_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_item1_id)
    api_fit.add_booster(type_id=eve_item1_id)
    api_fit.set_character(type_id=eve_item1_id)
    api_fit.add_drone(type_id=eve_item1_id, state=consts.ApiMinionState.engaging)
    api_fit.add_drone(type_id=eve_item1_id, state=consts.ApiMinionState.engaging)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item1_id)
    api_fit.add_fw_effect(type_id=eve_item1_id)
    api_fit.add_implant(type_id=eve_item1_id)
    api_fit.add_implant(type_id=eve_item1_id)
    api_fit.add_module(type_id=eve_item1_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item1_id)
    api_fit.add_module(type_id=eve_item1_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item1_id)
    api_fit.add_rig(type_id=eve_item1_id)
    api_fit.add_rig(type_id=eve_item1_id)
    api_fit.add_service(type_id=eve_item1_id, state=consts.ApiServiceState.online)
    api_fit.add_service(type_id=eve_item1_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_item1_id)
    api_fit.add_skill(type_id=eve_item1_id, level=5)
    api_fit.add_skill(type_id=eve_item2_id, level=5)
    api_fit.set_stance(type_id=eve_item1_id)
    # Verification
    assert len(api_fighter1.autocharges) == 1
    assert len(api_fighter2.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(subsystem_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
