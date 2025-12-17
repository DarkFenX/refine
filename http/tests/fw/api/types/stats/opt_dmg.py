import dataclasses

from fw.util import Absent, dc_to_dict


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitDps:

    item_kinds: StatDmgItemKinds | type[Absent] = Absent
    reload: bool | type[Absent] = Absent
    spool: str | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitVolley:

    item_kinds: StatDmgItemKinds | type[Absent] = Absent
    spool: str | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemDps:

    reload: bool | type[Absent] = Absent
    spool: str | type[Absent] = Absent
    include_charges: bool | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemVolley:

    spool: str | type[Absent] = Absent
    include_charges: bool | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)


@dataclasses.dataclass(kw_only=True)
class StatDmgItemKinds:

    default: bool | type[Absent] = Absent
    turret: bool | type[Absent] = Absent
    missile: bool | type[Absent] = Absent
    breacher: bool | type[Absent] = Absent
    vorton: bool | type[Absent] = Absent
    bomb: bool | type[Absent] = Absent
    smartbomb: bool | type[Absent] = Absent
    superweapon: bool | type[Absent] = Absent
    minion_mobile: bool | type[Absent] = Absent
    minion_static: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
