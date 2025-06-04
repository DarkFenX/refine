from tests import approx, check_no_field
from tests.fw.api import FitValOptions


def test_multiple(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant1 = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_implant2 = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {1: sorted([api_implant1.id, api_implant2.id])}
    # Action
    api_implant1.remove()
    # Verification
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_different_slots(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant1_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    eve_implant2_id = client.mk_eve_item(attrs={eve_slot_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant1 = api_fit.add_implant(type_id=eve_implant1_id)
    api_fit.add_implant(type_id=eve_implant2_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_implant3 = api_fit.add_implant(type_id=eve_implant1_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {1: sorted([api_implant1.id, api_implant3.id])}


def test_known_failures(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant1_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    eve_implant2_id = client.mk_eve_item(attrs={eve_slot_attr_id: 2})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_rig(type_id=eve_other_id)
    api_implant1 = api_fit.add_implant(type_id=eve_implant1_id)
    api_implant2 = api_fit.add_implant(type_id=eve_implant2_id)
    api_implant3 = api_fit.add_implant(type_id=eve_implant1_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=(True, [api_implant1.id])))
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {1: [api_implant3.id]}
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=(True, [api_implant3.id])))
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {1: [api_implant1.id]}
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=(True, [api_implant1.id, api_implant3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=FitValOptions(
        implant_slot_index=(True, [api_implant1.id, api_other.id, api_implant3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_implant4 = api_fit.add_implant(type_id=eve_implant2_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=(True, [api_implant1.id, api_implant3.id])))
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {2: sorted([api_implant2.id, api_implant4.id])}


def test_rounding(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant1_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1.2})
    eve_implant2_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1.4})
    eve_implant3_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant1 = api_fit.add_implant(type_id=eve_implant1_id)
    api_implant2 = api_fit.add_implant(type_id=eve_implant2_id)
    api_fit.add_implant(type_id=eve_implant3_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {1: sorted([api_implant1.id, api_implant2.id])}


def test_modified_index(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant_id = client.mk_eve_item(attrs={eve_slot_attr_id: 2})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_slot_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 3}, eff_ids=[eve_mod_effect_id])
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_implant1 = api_fit.add_implant(type_id=eve_implant_id)
    api_implant2 = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_implant1.update().attrs[eve_slot_attr_id].extra == approx(2)
    assert api_implant2.update().attrs[eve_slot_attr_id].extra == approx(2)
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {2: sorted([api_implant1.id, api_implant2.id])}
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification - attribute is modified, but not for purposes of validation
    assert api_implant1.update().attrs[eve_slot_attr_id].extra == approx(3)
    assert api_implant2.update().attrs[eve_slot_attr_id].extra == approx(3)
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {2: sorted([api_implant1.id, api_implant2.id])}


def test_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_implant_state(client, consts):
    # Disabled implants still compete for slots
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_implant_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant1 = api_fit.add_implant(type_id=eve_implant_id, state=False)
    api_implant2 = api_fit.add_implant(type_id=eve_implant_id, state=False)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.implant_slot_index == {1: sorted([api_implant1.id, api_implant2.id])}


def test_criterion_item_kind(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
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
    api_fit.add_subsystem(type_id=eve_item1_id)
    api_fit.add_subsystem(type_id=eve_item1_id)
    # Verification
    assert len(api_fighter1.autocharges) == 1
    assert len(api_fighter2.autocharges) == 1
    api_val = api_fit.validate(options=FitValOptions(implant_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
