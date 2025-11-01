import dataclasses
import typing

from tests.fw.util import Absent, dc_to_dict

if typing.TYPE_CHECKING:
    from tests.fw.api.aliases import DpsProfile
    from .opt_cap import StatCapSrcKinds
    from .opt_dmg import StatDmgItemKinds
    from .opt_remote_nps import StatNeutItemKinds
    from .opt_remote_rps import StatRemoteRepItemKinds

type StatOptionAlias = bool | type[Absent]
type StatOptionEhpAlias = StatOptionAlias | tuple[bool, list[StatsOptionEhp]]
type StatOptionRpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionRps]]
type StatOptionErpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionErps]]
type StatOptionCapBalanceAlias = StatOptionAlias | tuple[bool, list[StatsOptionCapBalance]]
type StatOptionFitDpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionFitDps]]
type StatOptionFitVolleyAlias = StatOptionAlias | tuple[bool, list[StatsOptionFitVolley]]
type StatOptionFitRemoteRpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionFitRemoteRps]]
type StatOptionFitRemoteNpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionFitRemoteNps]]


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
class StatsOptionCapBalance:

    src_kinds: StatCapSrcKinds | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitDps:

    item_kinds: StatDmgItemKinds | type[Absent] = Absent
    reload: bool | type[Absent] = Absent
    spool: str | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitVolley:

    item_kinds: StatDmgItemKinds | type[Absent] = Absent
    spool: str | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitRemoteRps:

    item_kinds: StatRemoteRepItemKinds | type[Absent] = Absent
    spool: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitRemoteNps:

    item_kinds: StatNeutItemKinds | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
