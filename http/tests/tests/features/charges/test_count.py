from fw import Muta, approx, check_no_field


def test_flooring(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 0.49})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - if rounded, would be 10
    assert api_module.update().charge_count == 9


def test_accuracy(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.1})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 2.3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - 2.3 / 0.1 = 22.999999999999996, if just floor() was used it'd be 22
    assert api_module.update().charge_count == 23


def test_no_charge(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 0.49})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_module.update()
    with check_no_field():
        api_module.charge_count  # noqa: B018


def test_modified_capacity(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_capacity_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod])
    eve_charge_id = client.mk_eve_item(
        attrs={eve_volume_attr_id: 0.05, eve_mod_attr_id: 2},
        eff_ids=[eve_mod_effect_id])
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 0.49})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - unmodified capacity is used
    api_module.update()
    assert api_module.attrs[eve_capacity_attr_id].extra == approx(0.98)
    assert api_module.charge_count == 9


def test_modified_volume(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_volume_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod])
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.49, eve_mod_attr_id: 2},
        eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - unmodified capacity is used
    api_module.update()
    assert api_module.charge.attrs[eve_volume_attr_id].extra == approx(0.1)
    assert api_module.charge_count == 9


def test_mutation_capacity(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_base_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 0.5})
    eve_mutated_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 0.75})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_capacity_attr_id: (1, 1.5)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_base_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_module.update()
    assert api_module.attrs[eve_capacity_attr_id].extra == approx(0.5)
    assert api_module.charge_count == 10
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    # Verification - volume of mutated item is used
    api_module.update()
    assert api_module.attrs[eve_capacity_attr_id].extra == approx(0.75)
    assert api_module.charge_count == 15
    # Action
    api_module.change_module(mutation={eve_capacity_attr_id: Muta.roll_to_api(val=1)})
    # Verification - unmutated capacity attribute value of mutated item is used
    api_module.update()
    assert api_module.attrs[eve_capacity_attr_id].extra == approx(1.125)
    assert api_module.charge_count == 15
    # Action
    api_module.change_module(mutation=None)
    # Verification
    api_module.update()
    assert api_module.attrs[eve_capacity_attr_id].extra == approx(0.5)
    assert api_module.charge_count == 10


def test_zero_capacity(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().charge_count == 0


def test_zero_volume(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 0.49})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().charge_count is None


def test_not_loaded_module(client, consts):
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.05})
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().charge_count is None


def test_not_loaded_charge(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 0.49})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().charge_count is None
