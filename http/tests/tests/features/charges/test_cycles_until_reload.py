from tests import check_no_field


def test_charge_rate_basic(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_charge_rate)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload == 10


def test_charge_rate_rounding_cycles(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_charge_rate)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.01})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.79, eve_charge_rate_attr_id: 8},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - result is floored, just for simplicity and efficiency. Can be changed at any
    # time to ceil it up
    assert api_module.update().cycles_until_reload == 9


def test_charge_rate_rounding_charge_rate(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_charge_rate)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module1_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1.4},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1.6},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module1_id, charge_type_id=eve_charge_id)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module1.update().cycles_until_reload == 10
    assert api_module2.update().cycles_until_reload == 5


def test_charge_rate_zero_charge_rate(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_charge_rate)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 0},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload is None


def test_charge_rate_no_charge(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_charge_rate)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.79, eve_charge_rate_attr_id: 8},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_module.update()
    with check_no_field():
        api_module.cycles_until_reload  # noqa: B018


def test_charge_rate_no_charge_rate(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_charge_rate)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - by default charge rate is assumed to be 0
    assert api_module.update().cycles_until_reload == 10


def test_charge_rate_charge_not_loaded(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_charge_rate)
    eve_charge_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload == 0


def test_no_default_effect(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.cycle_charge_rate)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload is None


def test_module_not_loaded(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_reload is None
