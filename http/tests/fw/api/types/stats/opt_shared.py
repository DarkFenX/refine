import dataclasses
import typing

from tests.fw.util import Absent

if typing.TYPE_CHECKING:
    from tests.fw.api.aliases import DpsProfile
    from .opt_dmg import StatDmgItemKinds
    from .opt_remote_rps import StatRemoteRpsItemKinds

type StatOptionAlias = bool | type[Absent]
type StatOptionEhpAlias = StatOptionAlias | tuple[bool, list[StatsOptionEhp]]
type StatOptionRpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionRps]]
type StatOptionErpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionErps]]
type StatOptionFitDpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionFitDps]]
type StatOptionFitVolleyAlias = StatOptionAlias | tuple[bool, list[StatsOptionFitVolley]]
type StatOptionFitRemoteRpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionFitRemoteRps]]


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


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitDps:

    item_kinds: StatDmgItemKinds | type[Absent] = Absent
    reload: bool | type[Absent] = Absent
    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitVolley:

    item_kinds: StatDmgItemKinds | type[Absent] = Absent
    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitRemoteRps:

    item_kinds: StatRemoteRpsItemKinds | type[Absent] = Absent
    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


def dc_to_dict(data) -> dict:  # noqa: ANN001
    return dataclasses.asdict(
        data, dict_factory=lambda d: {k: dc_to_dict(v) if _is_dc_instance(v) else v for k, v in d if v is not Absent})


def _is_dc_instance(obj: object) -> bool:
    return dataclasses.is_dataclass(obj) and not isinstance(obj, type)
