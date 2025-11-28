import dataclasses
import typing

from tests.fw.util import Absent, dc_to_dict

if typing.TYPE_CHECKING:
    from .opt_cap import StatsOptionCapBalance, StatsOptionCapSim
    from .opt_dmg import StatsOptionItemDps, StatsOptionItemVolley
    from .opt_ehp import StatsOptionEhp
    from .opt_mining import StatsOptionItemMining
    from .opt_remote_cps import StatsOptionItemRemoteCps
    from .opt_remote_nps import StatsOptionItemRemoteNps
    from .opt_remote_rps import StatsOptionItemRemoteRps
    from .opt_rps import StatsOptionErps, StatsOptionRps


@dataclasses.dataclass(kw_only=True)
class ItemStatsOptions:

    default: bool | type[Absent] = False
    speed: bool | type[Absent] = Absent
    agility: bool | type[Absent] = Absent
    align_time: bool | type[Absent] = Absent
    sig_radius: bool | type[Absent] = Absent
    mass: bool | type[Absent] = Absent
    warp_speed: bool | type[Absent] = Absent
    max_warp_range: bool | type[Absent] = Absent
    locks: bool | type[Absent] = Absent
    lock_range: bool | type[Absent] = Absent
    scan_res: bool | type[Absent] = Absent
    sensor: bool | type[Absent] = Absent
    dscan_range: bool | type[Absent] = Absent
    probing_size: bool | type[Absent] = Absent
    incoming_jam: bool | type[Absent] = Absent
    drone_control_range: bool | type[Absent] = Absent
    dps: bool | tuple[bool, list[StatsOptionItemDps]] | type[Absent] = Absent
    volley: bool | tuple[bool, list[StatsOptionItemVolley]] | type[Absent] = Absent
    mps: bool | tuple[bool, list[StatsOptionItemMining]] | type[Absent] = Absent
    hp: bool | type[Absent] = Absent
    ehp: bool | tuple[bool, list[StatsOptionEhp]] | type[Absent] = Absent
    wc_ehp: bool | type[Absent] = Absent
    rps: bool | tuple[bool, list[StatsOptionRps]] | type[Absent] = Absent
    erps: bool | tuple[bool, list[StatsOptionErps]] | type[Absent] = Absent
    resists: bool | type[Absent] = Absent
    remote_rps: bool | tuple[bool, list[StatsOptionItemRemoteRps]] | type[Absent] = Absent
    remote_cps: bool | tuple[bool, list[StatsOptionItemRemoteCps]] | type[Absent] = Absent
    remote_nps: bool | tuple[bool, list[StatsOptionItemRemoteNps]] | type[Absent] = Absent
    cap_amount: bool | type[Absent] = Absent
    cap_balance: bool | tuple[bool, list[StatsOptionCapBalance]] | type[Absent] = Absent
    cap_sim: bool | tuple[bool, list[StatsOptionCapSim]] | type[Absent] = Absent
    neut_resist: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
