from __future__ import annotations

from dataclasses import dataclass


class AttrModInfoMap(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({
            int(k): ModInfoList(
                ModInfo(
                    op=m[0],
                    initial_val=m[1],
                    range_mult=m[2],
                    resist_mult=m[3],
                    stacking_mult=m[4],
                    applied_val=m[5],
                    affectors=ModAffectorInfoList(ModAffectorInfo(item_id=n[0], attr_id=n[1]) for n in m[6]))
                for m in v)
            for k, v in data.items()})

    def find_by_op(self, *, affectee_attr_id: int, op: str) -> ModInfoList:
        return self.get(affectee_attr_id, ModInfoList()).find_by_op(op=op)

    def find_by_affector_item(self, *, affectee_attr_id: int, affector_item_id: str) -> ModInfoList:
        return self.get(affectee_attr_id, ModInfoList()).find_by_affector_item(affector_item_id=affector_item_id)

    def find_by_affector_attr(self, *, affectee_attr_id: int, affector_attr_id: int) -> ModInfoList:
        return self.get(affectee_attr_id, ModInfoList()).find_by_affector_attr(affector_attr_id=affector_attr_id)

    def __repr__(self) -> str:
        class_name = type(self).__name__
        super_repr = super().__repr__()
        return f'{class_name}({super_repr})'


class ModInfoList(list):

    def find_by_op(self, *, op: str) -> ModInfoList:
        return ModInfoList(i for i in self if i.op == op)

    def find_by_affector_item(self, *, affector_item_id: str) -> ModInfoList:
        return ModInfoList(i for i in self if i.affectors.find_by_item(item_id=affector_item_id))

    def find_by_affector_attr(self, *, affector_attr_id: int) -> ModInfoList:
        return ModInfoList(i for i in self if i.affectors.find_by_attr(attr_id=affector_attr_id))

    def one(self) -> ModInfo:
        if len(self) != 1:
            msg = f'expected 1 item, {len(self)} found'
            raise ValueError(msg)
        return self[0]

    def __repr__(self) -> str:
        class_name = type(self).__name__
        super_repr = super().__repr__()
        return f'{class_name}({super_repr})'


@dataclass(kw_only=True)
class ModInfo:

    op: str
    initial_val: float
    range_mult: float | None
    resist_mult: float | None
    stacking_mult: float | None
    applied_val: float
    affectors: list[ModAffectorInfo]


class ModAffectorInfoList(list):

    def find_by_item(self, *, item_id: str) -> ModAffectorInfoList:
        return ModAffectorInfoList(i for i in self if i.item_id == item_id)

    def find_by_attr(self, *, attr_id: int) -> ModAffectorInfoList:
        return ModAffectorInfoList(i for i in self if i.attr_id == attr_id)

    def one(self) -> ModInfo:
        if len(self) != 1:
            msg = f'expected 1 item, {len(self)} found'
            raise ValueError(msg)
        return self[0]

    def __repr__(self) -> str:
        class_name = type(self).__name__
        super_repr = super().__repr__()
        return f'{class_name}({super_repr})'


@dataclass(kw_only=True)
class ModAffectorInfo:

    item_id: str
    attr_id: int | None
