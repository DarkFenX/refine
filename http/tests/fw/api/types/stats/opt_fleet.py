import dataclasses
import typing

from tests.fw.util import Absent, dc_to_dict

if typing.TYPE_CHECKING:
    from .opt_dmg import StatsOptionFitDps, StatsOptionFitVolley
    from .opt_mining import StatsOptionFitMining
    from .opt_remote_nps import StatsOptionFitRemoteNps
    from .opt_remote_rps import StatsOptionFitRemoteRps


@dataclasses.dataclass(kw_only=True)
class FleetStatsOptions:

    default: bool | type[Absent] = False
    dps: bool | tuple[bool, list[StatsOptionFitDps]] | type[Absent] = Absent
    volley: bool | tuple[bool, list[StatsOptionFitVolley]] | type[Absent] = Absent
    mps: bool | tuple[bool, list[StatsOptionFitMining]] | type[Absent] = Absent
    remote_rps: bool | tuple[bool, list[StatsOptionFitRemoteRps]] | type[Absent] = Absent
    remote_cps: bool | type[Absent] = Absent
    remote_nps: bool | tuple[bool, list[StatsOptionFitRemoteNps]] | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
