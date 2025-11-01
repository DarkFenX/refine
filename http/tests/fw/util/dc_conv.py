import dataclasses

from .singletons import Absent


def dc_to_dict(data) -> dict:  # noqa: ANN001
    return dataclasses.asdict(
        data, dict_factory=lambda d: {k: dc_to_dict(v) if _is_dc_instance(v) else v for k, v in d if v is not Absent})


def _is_dc_instance(obj: object) -> bool:
    return dataclasses.is_dataclass(obj) and not isinstance(obj, type)
