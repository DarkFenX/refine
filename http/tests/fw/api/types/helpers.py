from fw.util import Absent, cast_to_prefixed_str


def process_effect_map_request[T](*, effect_map: dict[int | str, T] | type[Absent]) -> dict[str, T] | type[Absent]:
    if effect_map is Absent:
        return effect_map
    return {cast_to_prefixed_str(val=k, prefix='d'): v for k, v in effect_map.items()}
