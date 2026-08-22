from fw.util import cast_prefixed_to_int, cast_to_prefixed_str


def attr_fw_to_http(attr_id: int | str) -> str:
    return cast_to_prefixed_str(val=attr_id, prefix='e')


def attr_http_to_fw(attr_id: str) -> int | str:
    return cast_prefixed_to_int(val=attr_id, prefix='e')


def effect_fw_to_http(effect_id: int | str) -> str:
    return cast_to_prefixed_str(val=effect_id, prefix='d')


def effect_http_to_fw(effect_id: str) -> int | str:
    return cast_prefixed_to_int(val=effect_id, prefix='d')
