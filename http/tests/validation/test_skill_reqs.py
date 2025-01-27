from tests import check_no_field


def test_skill_add_remove_lower(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=2)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (2, 3)}}
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}


def test_skill_add_remove_equal(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=3)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}


def test_skill_add_remove_higher(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=4)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}


def test_skill_level_change(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=2)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (2, 3)}}
    # Action - lower to equal
    api_skill.change_skill(level=3)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action - equal to higher
    api_skill.change_skill(level=4)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action - higher to lower
    api_skill.change_skill(level=2)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (2, 3)}}
    # Action - lower to lower
    api_skill.change_skill(level=0)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (0, 3)}}
    # Action - lower to higher
    api_skill.change_skill(level=5)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action - higher to higher
    api_skill.change_skill(level=4)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action - higher to equal
    api_skill.change_skill(level=3)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action - equal to lower
    api_skill.change_skill(level=1)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (1, 3)}}


def test_not_loaded_skill(client, consts):
    eve_skill_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(srqs={eve_skill_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}
    # Action
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=2)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (2, 3)}}
    # Action
    api_skill.change_skill(level=3)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_skill.change_skill(level=0)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (0, 3)}}
    # Action
    api_skill.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.skill_reqs])
    assert api_val.passed is False
    assert api_val.details.skill_reqs == {api_module.id: {eve_skill_id: (None, 3)}}
