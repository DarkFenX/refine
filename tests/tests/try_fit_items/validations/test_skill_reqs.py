from fw.api import ValOptions


def test_skill_add_remove_lower(client):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_module(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_module(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=2)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])
    # Action
    api_skill.remove()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])


def test_skill_add_remove_equal(client):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_module(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_module(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=3)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])
    # Action
    api_skill.remove()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])


def test_skill_add_remove_higher(client):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_module(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_module(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=4)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])
    # Action
    api_skill.remove()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])


def test_skill_add_remove_zero(client):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_module(srqs={eve_skill1_id: 0})
    eve_module2_id = client.mk_eve_module(srqs={eve_skill1_id: 0, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification - zero is distinct from absence of skill just for the sake of consistency with
    # general EVE approach. E.g. old shield compensation skills left +3% passive resists on active
    # hardeners if they were not trained, and turned it to 0% at lvl 0.
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=0)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])
    # Action
    api_skill.remove()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])


def test_item_add_remove(client):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_module(srqs={eve_skill1_id: 0})
    eve_module2_id = client.mk_eve_module(srqs={eve_skill1_id: 0, eve_skill2_id: 5})
    eve_module3_id = client.mk_eve_module(srqs={eve_skill1_id: 3})
    eve_module4_id = client.mk_eve_module(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill1_id, level=0)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id, eve_module3_id, eve_module4_id])


def test_skill_level_change(client):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_module(srqs={eve_skill1_id: 3})
    eve_module2_id = client.mk_eve_module(srqs={eve_skill1_id: 3, eve_skill2_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=2)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])
    # Action - lower to equal
    api_skill.change_skill(level=3)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])
    # Action - equal to higher
    api_skill.change_skill(level=4)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])
    # Action - higher to lower
    api_skill.change_skill(level=2)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])
    # Action - lower to lower
    api_skill.change_skill(level=0)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])
    # Action - lower to higher
    api_skill.change_skill(level=5)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])
    # Action - higher to higher
    api_skill.change_skill(level=4)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])
    # Action - higher to equal
    api_skill.change_skill(level=3)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])
    # Action - equal to lower
    api_skill.change_skill(level=1)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])


def test_known_failures(client):
    eve_skill_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_module(srqs={eve_skill_id: 3})
    eve_module2_id = client.mk_eve_module()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module1_id)
    api_module2 = api_fit.add_module(type_id=eve_module1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=[api_module1.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=[api_module2.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=[api_module1.id, api_module2.id]),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_skill_switch_type_id(client):
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_module(srqs={eve_skill1_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill1_id, level=3)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[eve_module_id], failing=[])
    # Action
    api_skill.change_skill(type_id=eve_skill2_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[], failing=[eve_module_id])
    # Action
    api_skill.change_skill(type_id=eve_skill1_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[eve_module_id], failing=[])


def test_module_switch_type_id(client):
    eve_skill_id = client.mk_eve_item()
    eve_module1_id = client.mk_eve_module(srqs={eve_skill_id: 3})
    eve_module2_id = client.mk_eve_module()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill_id, level=2)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_mutation(client):
    # Actual use-case, mutated drones switch spec skill
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_base_drone_id = client.mk_eve_drone(srqs={eve_skill1_id: 2})
    eve_mutated_drone_id = client.mk_eve_drone(srqs={eve_skill2_id: 4})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_drone_id], eve_mutated_drone_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill1 = api_fit.add_skill(type_id=eve_skill1_id, level=2)
    api_drone = api_fit.add_drone(type_id=eve_base_drone_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_base_drone_id],
        failing=[eve_mutated_drone_id])
    # Action
    api_drone.change_drone(mutation=eve_mutator_id)
    api_fit.add_skill(type_id=eve_skill2_id, level=4)
    api_skill1.remove()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_mutated_drone_id],
        failing=[eve_base_drone_id])


def test_skill_state(client):
    # Disabled skill still satisfies skill requirements
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_module(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill_id, level=3, state=False)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[eve_module_id], failing=[])


def test_item_kinds(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster(srqs={eve_skill_id: 3})
    eve_charge_id = client.mk_eve_charge(srqs={eve_skill_id: 3})
    eve_drone_id = client.mk_eve_drone(srqs={eve_skill_id: 3})
    eve_fighter_id = client.mk_eve_fighter(srqs={eve_skill_id: 3})
    eve_implant_id = client.mk_eve_implant(srqs={eve_skill_id: 3})
    eve_module_high_id = client.mk_eve_module(rack=consts.Rack.high, srqs={eve_skill_id: 3})
    eve_module_mid_id = client.mk_eve_module(rack=consts.Rack.mid, srqs={eve_skill_id: 3})
    eve_module_low_id = client.mk_eve_module(rack=consts.Rack.low, srqs={eve_skill_id: 3})
    eve_rig_id = client.mk_eve_rig(srqs={eve_skill_id: 3})
    eve_service_id = client.mk_eve_service(srqs={eve_skill_id: 3})
    eve_subsystem_id = client.mk_eve_subsystem(srqs={eve_skill_id: 3})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification - rigs and services are ignored
    api_fit.check_try_fit_items(
        val_options=ValOptions(skill_reqs=True),
        passing=[eve_rig_id, eve_service_id],
        failing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_module_high_id,
            eve_module_mid_id,
            eve_module_low_id,
            eve_subsystem_id])


def test_not_loaded_skill(client):
    eve_skill_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_module(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[], failing=[eve_module_id])
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=2)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[], failing=[eve_module_id])
    # Action
    api_skill.change_skill(level=3)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[eve_module_id], failing=[])
    # Action
    api_skill.change_skill(level=0)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[], failing=[eve_module_id])
    # Action
    api_skill.remove()
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[], failing=[eve_module_id])


def test_failed_replacement(client):
    # Check that failed attempt to replace skill doesn't affect validation
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_module(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=3)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[eve_module_id], failing=[])
    # Action
    api_fit.add_skill(type_id=eve_skill_id, level=2, status_code=400)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[eve_module_id], failing=[])
    # Action
    api_skill.change_skill(level=2)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[], failing=[eve_module_id])
    # Action
    api_fit.add_skill(type_id=eve_skill_id, level=4, status_code=400)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(skill_reqs=True), passing=[], failing=[eve_module_id])
