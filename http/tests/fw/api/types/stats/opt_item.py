import dataclasses

from tests.fw.util import Absent
from .opt_shared import StatOptionAlias, StatOptionAliasEhp, dc_to_dict


@dataclasses.dataclass(kw_only=True)
class StatsItemOptions:

    default: bool = False
    agility: StatOptionAlias = Absent
    align_time: StatOptionAlias = Absent
    speed: StatOptionAlias = Absent
    hp: StatOptionAlias = Absent
    ehp: StatOptionAliasEhp = Absent
    wc_ehp: StatOptionAlias = Absent
    resists: StatOptionAlias = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
