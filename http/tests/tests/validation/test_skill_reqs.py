from tests import check_no_field
from tests.fw.api import ValOptions


def test_skill_add_remove_lower(client):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_item(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_item(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module1_id)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=2)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (2, 3)},
        api_module2.id: {eve_skill1_id: (2, 3), eve_skill2_id: (None, 5)}}
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}


def test_skill_add_remove_equal(client):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_item(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_item(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module1_id)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=3)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}


def test_skill_add_remove_higher(client):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_item(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_item(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module1_id)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=4)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (None, 3)},
        api_module2.id: {eve_skill1_id: (None, 3), eve_skill2_id: (None, 5)}}


def test_skill_level_change(client):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_item(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_item(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=2)
    api_module1 = api_fit.add_mod(type_id=eve_module1_id)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (2, 3)},
        api_module2.id: {eve_skill1_id: (2, 3), eve_skill2_id: (None, 5)}}
    # Action - lower to equal
    api_skill.change_skill(level=3)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action - equal to higher
    api_skill.change_skill(level=4)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action - higher to lower
    api_skill.change_skill(level=2)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (2, 3)},
        api_module2.id: {eve_skill1_id: (2, 3), eve_skill2_id: (None, 5)}}
    # Action - lower to lower
    api_skill.change_skill(level=0)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (0, 3)},
        api_module2.id: {eve_skill1_id: (0, 3), eve_skill2_id: (None, 5)}}
    # Action - lower to higher
    api_skill.change_skill(level=5)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action - higher to higher
    api_skill.change_skill(level=4)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action - higher to equal
    api_skill.change_skill(level=3)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module2.id: {eve_skill2_id: (None, 5)}}
    # Action - equal to lower
    api_skill.change_skill(level=1)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_module1.id: {eve_skill1_id: (1, 3)},
        api_module2.id: {eve_skill1_id: (1, 3), eve_skill2_id: (None, 5)}}


def test_mutation(client):
    # Actual use-case, mutated drones switch spec skill
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_base_drone_id = client.mk_eve_item(srqs={eve_skill1_id: 2})
    eve_mutated_drone_id = client.mk_eve_item(srqs={eve_skill2_id: 4})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_drone_id], eve_mutated_drone_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_base_drone_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_drone.id: {eve_skill1_id: (None, 2)}}
    # Action
    api_skill1 = api_fit.add_skill(type_id=eve_skill1_id, level=2)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_drone.id: {eve_skill2_id: (None, 4)}}
    # Action
    api_fit.add_skill(type_id=eve_skill2_id, level=4)
    api_skill1.remove()
    # Verification - only 2nd skill is needed, skill requirements are overwritten, not merged
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(mutation=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_drone.id: {eve_skill1_id: (None, 2)}}


def test_self_req(client):
    # Unrealistic scenario, but check what happens anyway
    eve_skill_id = client.alloc_item_id()
    client.mk_eve_item(id_=eve_skill_id, srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=2)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_skill.id: {eve_skill_id: (2, 3)}}
    # Action
    api_skill.change_skill(level=3)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_skill.change_skill(level=0)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_skill.id: {eve_skill_id: (0, 3)}}


def test_skill_state(client):
    # Disabled skill still satisfies skill requirements
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill_id, level=3, state=False)
    api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_item_state(client, consts):
    # Items in the lowest state still have skill requirements
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill_id, level=3)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.ghost)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_item_kinds(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_item_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    eve_ac_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_ac_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_ac_attr_id: eve_item_id},
        eff_ids=[eve_ac_effect_id],
        srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_item_id)
    api_skill = api_fit.add_skill(type_id=eve_item_id, level=0)
    api_implant = api_fit.add_implant(type_id=eve_item_id)
    api_booster = api_fit.add_booster(type_id=eve_item_id)
    api_ship = api_fit.set_ship(type_id=eve_item_id)
    api_fit.set_stance(type_id=eve_item_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_item_id)
    api_module_high = api_fit.add_mod(type_id=eve_item_id, rack=consts.ApiRack.high, charge_type_id=eve_item_id)
    api_module_mid = api_fit.add_mod(type_id=eve_item_id, rack=consts.ApiRack.mid, charge_type_id=eve_item_id)
    api_module_low = api_fit.add_mod(type_id=eve_item_id, rack=consts.ApiRack.low, charge_type_id=eve_item_id)
    api_fit.add_rig(type_id=eve_item_id)
    api_drone = api_fit.add_drone(type_id=eve_item_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    api_fit.add_fw_effect(type_id=eve_item_id)
    # Verification - characters, stances, rigs and FW effects are ignored
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {
        api_skill.id: {eve_skill_id: (None, 3)},
        api_implant.id: {eve_skill_id: (None, 3)},
        api_booster.id: {eve_skill_id: (None, 3)},
        api_ship.id: {eve_skill_id: (None, 3)},
        api_subsystem.id: {eve_skill_id: (None, 3)},
        api_module_high.id: {eve_skill_id: (None, 3)},
        api_module_high.charge.id: {eve_skill_id: (None, 3)},
        api_module_mid.id: {eve_skill_id: (None, 3)},
        api_module_mid.charge.id: {eve_skill_id: (None, 3)},
        api_module_low.id: {eve_skill_id: (None, 3)},
        api_module_low.charge.id: {eve_skill_id: (None, 3)},
        api_drone.id: {eve_skill_id: (None, 3)},
        api_fighter.id: {eve_skill_id: (None, 3)}}


def test_not_loaded_skill(client):
    eve_skill_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=2)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (2, 3)}}
    # Action
    api_skill.change_skill(level=3)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_skill.change_skill(level=0)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (0, 3)}}
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}


def test_failed_replacement(client):
    # Check that failed attempt to replace skill doesn't affect validation
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=3)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.add_skill(type_id=eve_skill_id, level=2, status_code=409)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_skill.change_skill(level=2)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (2, 3)}}
    # Action
    api_fit.add_skill(type_id=eve_skill_id, level=4, status_code=409)
    # Verification
    api_val = api_fit.validate(options=ValOptions(skill_reqs=True))
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (2, 3)}}
