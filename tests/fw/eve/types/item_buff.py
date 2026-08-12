from dataclasses import dataclass

from fw.util import Absent


@dataclass(kw_only=True)
class ItemBuffData:

    buffs: dict[int, float]
    item_list_id: int | type[Absent] = Absent

    @classmethod
    def from_raw(cls, data: dict[int, float] | tuple[dict[int, float], int]) -> ItemBuffData:
        if isinstance(data, tuple | list):
            buffs, item_list_id = data
            return ItemBuffData(buffs=buffs, item_list_id=item_list_id)
        return ItemBuffData(buffs=data)
