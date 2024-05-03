from __future__ import annotations

from typing import NamedTuple, Union


class AttrModInfoMap(dict):

    def __init__(self, data: dict):
        super().__init__({
            int(k): ModInfoList(
                ModInfo(m[0], m[1], m[2], ModAffectorInfoList(ModAffectorInfo.from_mixed(n) for n in m[3])) for m in v)
            for k, v in data.items()})

    def find_by_op(self, affectee_attr_id: int, op: str) -> ModInfoList:
        return self.get(affectee_attr_id, ModInfoList()).find_by_op(op=op)

    def find_by_affector_item(self, affectee_attr_id: int, affector_item_id: str) -> ModInfoList:
        return self.get(affectee_attr_id, ModInfoList()).find_by_affector_item(affector_item_id=affector_item_id)

    def find_by_affector_attr(self, affectee_attr_id: int, affector_attr_id: int) -> ModInfoList:
        return self.get(affectee_attr_id, ModInfoList()).find_by_affector_attr(affector_attr_id=affector_attr_id)

    def __repr__(self) -> str:
        class_name = type(self).__name__
        super_repr = super().__repr__()
        return f'{class_name}({super_repr})'


class ModInfoList(list):

    def find_by_op(self, op: str) -> ModInfoList:
        return ModInfoList(i for i in self if i.op == op)

    def find_by_affector_item(self, affector_item_id: str) -> ModInfoList:
        return ModInfoList(i for i in self if i.affectors.find_by_item(item_id=affector_item_id))

    def find_by_affector_attr(self, affector_attr_id: int) -> ModInfoList:
        return ModInfoList(i for i in self if i.affectors.find_by_attr(attr_id=affector_attr_id))

    def one(self) -> ModInfo:
        if len(self) != 1:
            raise ValueError(f'expected 1 item, {len(self)} found')
        return self[0]

    def __repr__(self) -> str:
        class_name = type(self).__name__
        super_repr = super().__repr__()
        return f'{class_name}({super_repr})'


class ModInfo(NamedTuple):

    val: float
    op: str
    penalized: bool
    affectors: list[ModAffectorInfo]


class ModAffectorInfoList(list):

    def find_by_item(self, item_id: str) -> ModAffectorInfoList:
        return ModAffectorInfoList(i for i in self if i.item_id == item_id)

    def find_by_attr(self, attr_id: int) -> ModAffectorInfoList:
        return ModAffectorInfoList(i for i in self if i.attr_id == attr_id)

    def one(self) -> ModInfo:
        if len(self) != 1:
            raise ValueError(f'expected 1 item, {len(self)} found')
        return self[0]

    def __repr__(self) -> str:
        class_name = type(self).__name__
        super_repr = super().__repr__()
        return f'{class_name}({super_repr})'


class ModAffectorInfo(NamedTuple):

    item_id: str
    attr_id: Union[int, None]
    hardcoded: Union[float, None]

    @classmethod
    def from_mixed(cls, data: list) -> ModAffectorInfo:
        item_id, value_info = data
        attr_id = value_info.get('attr')
        hardcoded = value_info.get('hc')
        return cls(item_id, attr_id, hardcoded)
