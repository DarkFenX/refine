from tests import check_no_field


def test_skill_add_remove(client, consts):
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
