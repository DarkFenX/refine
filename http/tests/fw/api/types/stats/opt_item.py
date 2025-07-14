import dataclasses

from tests.fw.util import Absent
from .opt_shared import StatOptionAlias, StatOptionEhpAlias, StatOptionErpsAlias, StatOptionRpsAlias, dc_to_dict

type StatOptionItemRrAlias = StatOptionAlias | tuple[bool, list[StatsOptionItemRr]]


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
    rr_shield: StatOptionItemRrAlias = Absent
    rr_armor: StatOptionItemRrAlias = Absent
    rr_hull: StatOptionItemRrAlias = Absent
    rr_capacitor: StatOptionItemRrAlias = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemRr:

    spool: str | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
