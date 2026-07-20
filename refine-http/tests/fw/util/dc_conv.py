import dataclasses

from .singletons import Absent


def dc_to_dict(data) -> dict:  # ruff:ignore[missing-type-function-argument]
    return dataclasses.asdict(data, dict_factory=lambda d: {k: v for k, v in d if v is not Absent})
