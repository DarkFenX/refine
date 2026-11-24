from fw.util import AttrDict, cast_prefixed_to_int, cast_to_prefixed_str


def attr_fw_to_http(attr_id: int | str) -> str:
    return cast_to_prefixed_str(val=attr_id, prefix='e')


def attr_http_to_fw(attr_id: str) -> int | str:
    return cast_prefixed_to_int(val=attr_id, prefix='e')


def effect_fw_to_http(effect_id: int | str) -> str:
    return cast_to_prefixed_str(val=effect_id, prefix='d')


def effect_http_to_fw(effect_id: str) -> int | str:
    return cast_prefixed_to_int(val=effect_id, prefix='d')


def src_items_http_to_fw(data: dict) -> dict[int, AttrDict]:
    return {int(type_id): AttrDict(data=item) for type_id, item in data.items()}


def src_mutated_items_http_to_fw(data: dict) -> dict[int, dict[int, AttrDict]]:
    return {
        int(base_type_id): {int(mutator_id): AttrDict(data=item) for mutator_id, item in mutators.items()}
        for base_type_id, mutators in data.items()}


def src_mutated_attr_rolls_http_to_fw(data: dict) -> dict[int | str, float]:
    return {attr_http_to_fw(attr_id=attr_id): roll for attr_id, roll in data.items()}
