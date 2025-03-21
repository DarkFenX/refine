from tests import approx, check_no_field
from tests.fw.api import ValOptions


def test_add_remove(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked)
    eve_module1_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_module2_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module2.id]
    # Action
    api_module2.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_state_switch(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked)
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module.id]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module.id]
    # Action
    api_module.change_module(state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked)
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_rig(type_id=eve_other_id)
    api_module1 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module2 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module2.id]
    api_val = api_fit.validate(options=ValOptions(activation_blocked=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module1.id]
    api_val = api_fit.validate(options=ValOptions(activation_blocked=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        activation_blocked=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
