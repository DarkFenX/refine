import dataclasses
import typing

from tests.fw.util import Absent, dc_to_dict

if typing.TYPE_CHECKING:
    from tests.fw.api.aliases import DpsProfile
    from .opt_cap import StatCapSrcKinds
    from .opt_dmg import StatDmgItemKinds
    from .opt_remote_nps import StatNeutItemKinds
    from .opt_remote_rps import StatRemoteRepItemKinds


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
class StatsOptionCapSim:

    cap_perc: float | type[Absent] = Absent
    stagger: bool | tuple[bool, list[str]] | type[Absent] = Absent

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
