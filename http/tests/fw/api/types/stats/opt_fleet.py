import dataclasses
import typing

from tests.fw.util import Absent, dc_to_dict

if typing.TYPE_CHECKING:
    from .opt_shared import StatsOptionFitDps, StatsOptionFitRemoteNps, StatsOptionFitRemoteRps, StatsOptionFitVolley


@dataclasses.dataclass(kw_only=True)
class FleetStatsOptions:

    default: bool | type[Absent] = False
    dps: bool | tuple[bool, list[StatsOptionFitDps]] | type[Absent] = Absent
    volley: bool | tuple[bool, list[StatsOptionFitVolley]] | type[Absent] = Absent
    remote_rps: bool | tuple[bool, list[StatsOptionFitRemoteRps]] | type[Absent] = Absent
    remote_cps: bool | type[Absent] = Absent
    remote_nps: bool | tuple[bool, list[StatsOptionFitRemoteNps]] | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
