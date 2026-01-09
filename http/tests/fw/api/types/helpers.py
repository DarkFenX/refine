from fw.api.aliases import MutaAdd, MutaChange
from fw.util import Absent, cast_prefixed_to_int, cast_to_prefixed_str


def attr_fw_to_http(attr_id: int | str) -> str:
    return cast_to_prefixed_str(val=attr_id, prefix='e')


def attr_http_to_fw(attr_id: str) -> int | str:
    return cast_prefixed_to_int(val=attr_id, prefix='e')


def effect_fw_to_http(effect_id: int | str) -> str:
    return cast_to_prefixed_str(val=effect_id, prefix='d')


def effect_http_to_fw(effect_id: str) -> int | str:
    return cast_prefixed_to_int(val=effect_id, prefix='d')


def process_effect_map_request[T](*, effect_map: dict[int | str, T] | type[Absent]) -> dict[str, T] | type[Absent]:
    if effect_map is Absent:
        return effect_map
    return {cast_to_prefixed_str(val=k, prefix='d'): v for k, v in effect_map.items()}


def process_muta_add_request(*, mutation: MutaAdd | type[Absent]) -> MutaAdd | type[Absent]:
    if mutation is Absent:
        return mutation
    if not isinstance(mutation, tuple | list):
        return mutation
    mutator_id, attrs = mutation
    return mutator_id, _cast_map(data=attrs)


def process_muta_change_request(*, mutation: MutaAdd | MutaChange | type[Absent]) -> MutaAdd | MutaChange | type[Absent]:
    if mutation is Absent:
        return mutation
    if isinstance(mutation, dict):
        return _cast_map(data=mutation)
    if not isinstance(mutation, tuple | list):
        return mutation
    mutator_id, attrs = mutation
    return mutator_id, _cast_map(data=attrs)


def _cast_map[T](*, data: dict[int | str, T]) -> dict[str, T]:
    return {cast_to_prefixed_str(val=k, prefix='e'): v for k, v in data.items()}
