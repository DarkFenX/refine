import typing

from fw.util import Absent, cast_to_prefixed_str

if typing.TYPE_CHECKING:
    from fw.api.aliases import MutaAdd, MutaChange


####################################################################################################
# Control
####################################################################################################
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


def process_muta_change_request(
        *,
        mutation: MutaAdd | MutaChange | type[Absent] | None,
) -> MutaAdd | MutaChange | type[Absent]:
    if mutation is None or mutation is Absent:
        return mutation
    if isinstance(mutation, dict):
        return _cast_map(data=mutation)
    if not isinstance(mutation, tuple | list):
        return mutation
    mutator_id, attrs = mutation
    return mutator_id, _cast_map(data=attrs)


def _cast_map[T](*, data: dict[int | str, T]) -> dict[str, T]:
    return {cast_to_prefixed_str(val=k, prefix='e'): v for k, v in data.items()}


####################################################################################################
# Misc
####################################################################################################
def process_stats_options_request(*, options: typing.Any) -> typing.Any:
    if options is Absent:
        return options
    if isinstance(options, tuple | list):
        default, overrides = options
        return [default.to_dict(), [[o.to_dict(), list(ids)] for o, ids in overrides]]
    return options.to_dict()


def process_val_options_request(*, options: typing.Any) -> typing.Any:
    if options is Absent:
        return options
    return options.to_dict()
