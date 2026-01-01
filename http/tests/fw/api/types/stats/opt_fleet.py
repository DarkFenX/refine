import dataclasses
import typing

from fw.util import Absent, dc_to_dict

if typing.TYPE_CHECKING:
    from .opt_dmg import StatsOptionFitDps, StatsOptionFitVolley
    from .opt_mining import StatsOptionFitMining
    from .opt_outgoing_cps import StatsOptionFitOutCps
    from .opt_outgoing_nps import StatsOptionFitOutNps
    from .opt_outgoing_rps import StatsOptionFitOutRps


@dataclasses.dataclass(kw_only=True)
class FleetStatsOptions:

    default: bool | type[Absent] = False
    dps: bool | tuple[bool, list[StatsOptionFitDps]] | type[Absent] = Absent
    volley: bool | tuple[bool, list[StatsOptionFitVolley]] | type[Absent] = Absent
    mps: bool | tuple[bool, list[StatsOptionFitMining]] | type[Absent] = Absent
    outgoing_nps: bool | tuple[bool, list[StatsOptionFitOutNps]] | type[Absent] = Absent
    outgoing_rps: bool | tuple[bool, list[StatsOptionFitOutRps]] | type[Absent] = Absent
    outgoing_cps: bool | tuple[bool, list[StatsOptionFitOutCps]] | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
