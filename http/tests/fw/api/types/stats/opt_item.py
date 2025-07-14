import dataclasses

from tests.fw.util import Absent
from .opt_shared import StatOptionAlias, StatOptionEhpAlias, StatOptionErpsAlias, StatOptionRpsAlias, dc_to_dict

type StatOptionItemRemoteRpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionItemRemoteRps]]
type StatOptionItemRemoteCpsAlias = StatOptionAlias | tuple[bool, list[StatsOptionItemRemoteCps]]


@dataclasses.dataclass(kw_only=True)
class ItemStatsOptions:

    default: bool = False
    agility: StatOptionAlias = Absent
    align_time: StatOptionAlias = Absent
    speed: StatOptionAlias = Absent
    hp: StatOptionAlias = Absent
    ehp: StatOptionEhpAlias = Absent
    wc_ehp: StatOptionAlias = Absent
    rps: StatOptionRpsAlias = Absent
    erps: StatOptionErpsAlias = Absent
    resists: StatOptionAlias = Absent
    remote_rps: StatOptionItemRemoteRpsAlias = Absent
    remote_cps: StatOptionItemRemoteCpsAlias = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemRemoteRps:

    spool: str | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemRemoteCps:

    ignore_state: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
