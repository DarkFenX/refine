import typing

from fw import approx

if typing.TYPE_CHECKING:
    from fw.request import Request


def test_error_body_malformed(client):
    api_src = client.create_source()
    # Action & verification
    api_src.get_mutated_attr_rolls(
        mutation='random',
        values={},
        status_code=400,
        json_predicate={'code': 'JSN-001', 'message': 're:.+'})


def test_error_no_src(client):
    eve_base_item_id = client.alloc_item_id()
    eve_mutator_id = client.alloc_item_id()
    api_src = client.create_source()
    client.remove_source(src_alias=api_src.alias)
    # Action & verification
    api_src.get_mutated_attr_rolls(
        mutation=(eve_base_item_id, eve_mutator_id),
        values={},
        status_code=404,
        json_predicate={'code': 'SRC-001', 'message': f'alias "{api_src.alias}" not found'})


def test_error_src_alias_malformed(client):

    def hook_req(req: Request):
        req.set_url(url=req.get_url().replace(f'/src/{api_src.alias}/', '/src/_..-/'))

    eve_base_item_id = client.alloc_item_id()
    eve_mutator_id = client.alloc_item_id()
    api_src = client.create_source()
    # Action & verification
    api_src.get_mutated_attr_rolls(
        mutation=(eve_base_item_id, eve_mutator_id),
        values={},
        hook_req=hook_req,
        status_code=404,
        json_predicate={'code': 'SRC-003', 'message': 'alias "_..-" not found: alias is empty after pruning'})


def test_range_direct(client):
    # Values out of mutation range are clamped
    eve_lower_attr_id = client.mk_eve_attr()
    eve_min_attr_id = client.mk_eve_attr()
    eve_within_attr_id = client.mk_eve_attr()
    eve_max_attr_id = client.mk_eve_attr()
    eve_higher_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={
        eve_lower_attr_id: 100,
        eve_min_attr_id: 100,
        eve_within_attr_id: 100,
        eve_max_attr_id: 100,
        eve_higher_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_lower_attr_id: (0.8, 1.2),
        eve_min_attr_id: (0.8, 1.2),
        eve_within_attr_id: (0.8, 1.2),
        eve_max_attr_id: (0.8, 1.2),
        eve_higher_attr_id: (0.8, 1.2)})
    api_src = client.create_source()
    # Action
    api_rolls = api_src.get_mutated_attr_rolls(mutation=(eve_base_item_id, eve_mutator_id), values={
        eve_lower_attr_id: 53.02,
        eve_min_attr_id: 80,
        eve_within_attr_id: 104,
        eve_max_attr_id: 120,
        eve_higher_attr_id: 1009.99})
    # Verification
    assert api_rolls.keys() == {
        eve_lower_attr_id, eve_min_attr_id, eve_within_attr_id, eve_max_attr_id, eve_higher_attr_id}
    assert api_rolls[eve_lower_attr_id] == 0
    assert api_rolls[eve_min_attr_id] == approx(0)
    assert api_rolls[eve_within_attr_id] == approx(0.6)
    assert api_rolls[eve_max_attr_id] == approx(1)
    assert api_rolls[eve_higher_attr_id] == 1


def test_range_reverse(client):
    # Roll 0 corresponds to mutator's "min" multiplier, even when it is higher than the "max" one
    eve_lower_attr_id = client.mk_eve_attr()
    eve_within_attr_id = client.mk_eve_attr()
    eve_higher_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={
        eve_lower_attr_id: 100,
        eve_within_attr_id: 100,
        eve_higher_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_lower_attr_id: (1.2, 0.8),
        eve_within_attr_id: (1.2, 0.8),
        eve_higher_attr_id: (1.2, 0.8)})
    api_src = client.create_source()
    # Action
    api_rolls = api_src.get_mutated_attr_rolls(mutation=(eve_base_item_id, eve_mutator_id), values={
        eve_lower_attr_id: 53.02,
        eve_within_attr_id: 104,
        eve_higher_attr_id: 1009.99})
    # Verification
    assert api_rolls.keys() == {eve_lower_attr_id, eve_within_attr_id, eve_higher_attr_id}
    assert api_rolls[eve_lower_attr_id] == 1
    assert api_rolls[eve_within_attr_id] == approx(0.4)
    assert api_rolls[eve_higher_attr_id] == 0


def test_unmutated_value_source(client):
    # Unmutated value is taken from the base type, and from the mutated type when the base type
    # does not define the attribute
    eve_both_attr_id = client.mk_eve_attr()
    eve_base_attr_id = client.mk_eve_attr()
    eve_mutated_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_both_attr_id: 100, eve_base_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(
        is_dynamic=True,
        attrs={eve_both_attr_id: 200, eve_mutated_attr_id: 200})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_both_attr_id: (0.8, 1.2),
        eve_base_attr_id: (0.8, 1.2),
        eve_mutated_attr_id: (0.8, 1.2)})
    api_src = client.create_source()
    # Action
    api_rolls = api_src.get_mutated_attr_rolls(mutation=(eve_base_item_id, eve_mutator_id), values={
        eve_both_attr_id: 110,
        eve_base_attr_id: 110,
        eve_mutated_attr_id: 220})
    # Verification
    assert api_rolls.keys() == {eve_both_attr_id, eve_base_attr_id, eve_mutated_attr_id}
    assert api_rolls[eve_both_attr_id] == approx(0.75)
    assert api_rolls[eve_base_attr_id] == approx(0.75)
    assert api_rolls[eve_mutated_attr_id] == approx(0.75)


def test_no_roll(client):
    eve_normal_attr_id = client.mk_eve_attr()
    eve_unknown_attr_id = client.alloc_attr_id()
    eve_no_range_attr_id = client.mk_eve_attr()
    eve_no_value_attr_id = client.mk_eve_attr()
    eve_zero_range_attr_id = client.mk_eve_attr()
    eve_zero_value_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={
        eve_normal_attr_id: 100,
        eve_no_range_attr_id: 100,
        eve_zero_range_attr_id: 100,
        eve_zero_value_attr_id: 0})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)], attrs={
        eve_normal_attr_id: (0.8, 1.2),
        eve_no_value_attr_id: (0.8, 1.2),
        eve_zero_range_attr_id: (1.1, 1.1),
        eve_zero_value_attr_id: (0.8, 1.2)})
    api_src = client.create_source()
    # Action
    api_rolls = api_src.get_mutated_attr_rolls(mutation=(eve_base_item_id, eve_mutator_id), values={
        eve_normal_attr_id: 104,
        eve_unknown_attr_id: 104,
        eve_no_range_attr_id: 104,
        eve_no_value_attr_id: 104,
        eve_zero_range_attr_id: 110,
        eve_zero_value_attr_id: 0})
    # Verification - no roll for attributes which are not known to the source, which are not changed
    # by the mutator, which have no unmutated value, and whose mutation range has zero width (either
    # by itself, or due to zero unmutated value)
    assert api_rolls.keys() == {eve_normal_attr_id}
    assert api_rolls[eve_normal_attr_id] == approx(0.6)


def test_unknown_mutation(client):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    eve_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_other_id = client.alloc_item_id()
    api_src = client.create_source()
    # Action
    with api_src.batch() as api_src_batch:
        api_rolls1 = api_src_batch.get_mutated_attr_rolls(
            mutation=(eve_other_id, eve_mutator_id),
            values={eve_attr_id: 104})
        api_rolls2 = api_src_batch.get_mutated_attr_rolls(
            mutation=(eve_base_item_id, eve_other_id),
            values={eve_attr_id: 104})
        api_rolls3 = api_src_batch.get_mutated_attr_rolls(
            mutation=(eve_item_id, eve_mutator_id),
            values={eve_attr_id: 104})
        api_rolls4 = api_src_batch.get_mutated_attr_rolls(
            mutation=(eve_base_item_id, eve_mutator_id),
            values={eve_attr_id: 104})
    # Verification - no rolls for unknown base types, unknown mutators, and base types the mutator
    # does not accept
    assert api_rolls1 == {}
    assert api_rolls2 == {}
    assert api_rolls3 == {}
    assert api_rolls4.keys() == {eve_attr_id}
    assert api_rolls4[eve_attr_id] == approx(0.6)


def test_repeated_attr(client):

    def hook_req(req: Request):
        # A dictionary cannot hold repeated keys, so the 2nd value is added to the serialized body
        api_attr_entry = f'"e{eve_attr_id}": 104'
        body = req.get_body()
        assert body.count(api_attr_entry) == 1
        req.set_body(body=body.replace(api_attr_entry, f'{api_attr_entry}, "e{eve_attr_id}": 116'))

    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    api_src = client.create_source()
    # Action
    api_rolls = api_src.get_mutated_attr_rolls(
        mutation=(eve_base_item_id, eve_mutator_id),
        values={eve_attr_id: 104},
        hook_req=hook_req)
    # Verification - the first value is used. If the attribute was reported twice, the second roll
    # would replace the first one when response is parsed
    assert api_rolls.keys() == {eve_attr_id}
    assert api_rolls[eve_attr_id] == approx(0.6)


def test_no_values(client):
    eve_attr_id = client.mk_eve_attr()
    eve_base_item_id = client.mk_eve_item(attrs={eve_attr_id: 100})
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_item_id], eve_mutated_item_id)],
        attrs={eve_attr_id: (0.8, 1.2)})
    api_src = client.create_source()
    # Action
    api_rolls = api_src.get_mutated_attr_rolls(mutation=(eve_base_item_id, eve_mutator_id), values={})
    # Verification
    assert api_rolls == {}
