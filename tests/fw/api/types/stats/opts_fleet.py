import dataclasses
import typing

from fw.util import Absent, dc_to_dict

if typing.TYPE_CHECKING:
    from .opt_dmg import StatsOptionFitDmg
    from .opt_mass import StatsOptionMass
    from .opt_mining import StatsOptionFitMining
    from .opt_outgoing_cps import StatsOptionFitOutCps
    from .opt_outgoing_nps import StatsOptionFitOutNps
    from .opt_outgoing_rps import StatsOptionFitOutRps


@dataclasses.dataclass(kw_only=True)
class FleetStatsOptions:

    default: bool | type[Absent] = False
    dmg: bool | list[StatsOptionFitDmg] | type[Absent] = Absent
    mps: bool | list[StatsOptionFitMining] | type[Absent] = Absent
    outgoing_nps: bool | list[StatsOptionFitOutNps] | type[Absent] = Absent
    outgoing_rps: bool | list[StatsOptionFitOutRps] | type[Absent] = Absent
    outgoing_cps: bool | list[StatsOptionFitOutCps] | type[Absent] = Absent
    mass: bool | list[StatsOptionMass] | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
