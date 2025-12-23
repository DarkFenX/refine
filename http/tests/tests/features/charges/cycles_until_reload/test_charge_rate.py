from fw import Muta, approx, check_no_field


def test_basic(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_charge_rate,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_empty == 10


def test_rounding_cycles(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_charge_rate,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.01})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.79, eve_charge_rate_attr_id: 8, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - result is floored, since most effects are supposed to be unable to run without
    # full stack of charges
    assert api_module.update().cycles_until_empty == 9


def test_rounding_cycles_aar(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_local_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fueled_armor_repair,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_remote_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_armor_repairer,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.01})
    eve_local_aar_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.73, eve_charge_rate_attr_id: 8, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_local_effect_id],
        defeff_id=eve_local_effect_id)
    eve_remote_aar_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.74, eve_charge_rate_attr_id: 8, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_remote_effect_id],
        defeff_id=eve_remote_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_local_aar = api_fit.add_module(type_id=eve_local_aar_id, charge_type_id=eve_charge_id)
    api_remote_aar = api_fit.add_module(type_id=eve_remote_aar_id, charge_type_id=eve_charge_id)
    # Verification - armor ancillary modules are an exception to this rule and can run on partial
    # stack of charges
    assert api_local_aar.update().cycles_until_empty == 10
    assert api_remote_aar.update().cycles_until_empty == 10


def test_rounding_charge_rate(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_charge_rate,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module1_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1.4, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1.6, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module1_id, charge_type_id=eve_charge_id)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module1.update().cycles_until_empty == 10
    assert api_module2.update().cycles_until_empty == 5


def test_zero_charge_rate(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_charge_rate,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 0, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_empty is None


def test_modified_charge_rate(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_charge_rate_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_charge_rate,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_volume_attr_id: 0.05, eve_mod_attr_id: 2},
        eff_ids=[eve_mod_effect_id])
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - unmodified charge rate is still used
    api_module.update()
    assert api_module.attrs[eve_charge_rate_attr_id].modified == approx(2)
    assert api_module.cycles_until_empty == 10


def test_mutation_charge_rate(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_charge_rate,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_base_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_mutated_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 2, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_charge_rate_attr_id: (1, 1.5)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_base_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_module.update()
    assert api_module.attrs[eve_charge_rate_attr_id].modified == approx(1)
    assert api_module.cycles_until_empty == 10
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    # Verification - value from mutated item is used
    api_module.update()
    assert api_module.attrs[eve_charge_rate_attr_id].modified == approx(2)
    assert api_module.cycles_until_empty == 5
    # Action
    api_module.change_module(mutation={eve_charge_rate_attr_id: Muta.roll_to_api(val=1)})
    # Verification - but attribute mutation is ignored
    api_module.update()
    assert api_module.attrs[eve_charge_rate_attr_id].modified == approx(3)
    assert api_module.cycles_until_empty == 5
    # Action
    api_module.change_module(mutation=None)
    # Verification
    api_module.update()
    assert api_module.attrs[eve_charge_rate_attr_id].modified == approx(1)
    assert api_module.cycles_until_empty == 10


def test_no_charge(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_charge_rate,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.79, eve_charge_rate_attr_id: 8, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_module.update()
    with check_no_field():
        api_module.cycles_until_empty  # noqa: B018


def test_no_charge_rate(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_charge_rate,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - by default charge rate is assumed to be 0
    assert api_module.update().cycles_until_empty == 10


def test_not_loaded_charge(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.cycle_charge_rate,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_charge_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.50, eve_charge_rate_attr_id: 1, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().cycles_until_empty is None
