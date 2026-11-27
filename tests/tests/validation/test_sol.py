from fw import Absent, check_no_field
from fw.api import ValOptions


def test_fit_ids_absent(client):
    # All fits are validated when fit IDs are not specified
    eve_loaded_id = client.mk_eve_item()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_implant(type_id=eve_loaded_id)
    api_fit2 = api_sol.create_fit()
    api_implant2 = api_fit2.add_implant(type_id=eve_not_loaded_id)
    api_fit3 = api_sol.create_fit()
    api_implant3 = api_fit3.add_implant(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_sol.validate(fit_ids=Absent, options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.fits.keys() == {api_fit2.id, api_fit3.id}
    assert api_val.fits[api_fit2.id].not_loaded_item == [api_implant2.id]
    assert api_val.fits[api_fit3.id].not_loaded_item == [api_implant3.id]
    # Action
    api_implant2.remove()
    api_implant3.remove()
    # Verification
    api_val = api_sol.validate(fit_ids=Absent, options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]


def test_fit_ids_empty(client):
    # No fits are validated when the list of fit IDs is empty
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_not_loaded_id)
    # Verification
    api_val = api_sol.validate(fit_ids=[], options=ValOptions(not_loaded_item=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]


def test_fit_ids_listed(client):
    # Only listed fits are validated; repeated IDs and IDs of fits which do not exist are ignored
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_implant1 = api_fit1.add_implant(type_id=eve_not_loaded_id)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_implant(type_id=eve_not_loaded_id)
    api_fit3 = api_sol.create_fit()
    api_fit3.remove()
    # Verification
    api_val = api_sol.validate(
        fit_ids=[api_fit1.id, api_fit3.id, api_fit1.id],
        options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.fits.keys() == {api_fit1.id}
    assert api_val.fits[api_fit1.id].not_loaded_item == [api_implant1.id]
