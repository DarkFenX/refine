import dataclasses

from tests.fw.util import Absent, dc_to_dict


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
