import dataclasses

from tests.fw.api.aliases import DpsProfile
from tests.fw.util import Absent

type StatOptionAlias = bool | type[Absent]
type StatOptionEhpAlias = StatOptionAlias | tuple[bool, list[StatsOptionEhp]]
type StatOptionRpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionRps]]
type StatOptionErpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionErps]]


@dataclasses.dataclass(kw_only=True)
class StatsOptionEhp:

    incoming_dps: DpsProfile | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionRps:

    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionErps:

    incoming_dps: DpsProfile | type[Absent] = Absent
    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


def dc_to_dict(data: dataclasses.dataclass) -> dict:
    return dataclasses.asdict(
        data, dict_factory=lambda d: {k: dc_to_dict(v) if _is_dc_instance(v) else v for k, v in d if v is not Absent})


def _is_dc_instance(obj):
    return dataclasses.is_dataclass(obj) and not isinstance(obj, type)
