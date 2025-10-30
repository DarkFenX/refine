import dataclasses
import typing

from tests.fw.util import Absent
from .opt_shared import dc_to_dict

if typing.TYPE_CHECKING:
    from .opt_shared import (
        StatOptionAlias,
        StatOptionFitDpsAlias,
        StatOptionFitRemoteNpsAlias,
        StatOptionFitRemoteRpsAlias,
        StatOptionFitVolleyAlias,
    )


@dataclasses.dataclass(kw_only=True)
class FleetStatsOptions:

    default: bool = False
    dps: StatOptionFitDpsAlias = Absent
    volley: StatOptionFitVolleyAlias = Absent
    remote_rps: StatOptionFitRemoteRpsAlias = Absent
    remote_cps: StatOptionAlias = Absent
    remote_nps: StatOptionFitRemoteNpsAlias = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
