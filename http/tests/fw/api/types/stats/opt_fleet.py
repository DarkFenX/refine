import dataclasses

from tests.fw.util import Absent
from .opt_shared import (
    StatOptionAlias,
    StatOptionFitDpsAlias,
    StatOptionFitRemoteRpsAlias,
    StatOptionFitVolleyAlias,
    dc_to_dict,
)


@dataclasses.dataclass(kw_only=True)
class FleetStatsOptions:

    default: bool = False
    dps: StatOptionFitDpsAlias = Absent
    volley: StatOptionFitVolleyAlias = Absent
    remote_rps: StatOptionFitRemoteRpsAlias = Absent
    remote_cps: StatOptionAlias = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
