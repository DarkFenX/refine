from __future__ import annotations

from typing import NamedTuple, Union


class AttrModInfoMap(dict):

    def __init__(self, data: dict):
        super().__init__({
            int(k): ModInfoList(
                ModInfo(m[0], m[1], m[2], m[3], ModSrcInfoList(ModSrcInfo.from_mixed(n) for n in m[4])) for m in v)
            for k, v in data.items()})

    def find_by_op(self, tgt_attr_id: int, op: str) -> ModInfoList:
        return self.get(tgt_attr_id, ModInfoList()).find_by_op(op=op)

    def find_by_src_item(self, tgt_attr_id: int, src_item_id: int) -> ModInfoList:
        return self.get(tgt_attr_id, ModInfoList()).find_by_src_item(src_item_id=src_item_id)

    def __repr__(self) -> str:
        class_name = type(self).__name__
        super_repr = super().__repr__()
        return f'{class_name}({super_repr})'


class ModInfoList(list):

    def find_by_op(self, op: str) -> ModInfoList:
        return ModInfoList(i for i in self if i.op == op)

    def find_by_src_item(self, src_item_id: int) -> ModInfoList:
        return ModInfoList(i for i in self if any(s.item_id == src_item_id for s in i.src))

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
    aggr_mode: str
    src: list[ModSrcInfo]


class ModSrcInfoList(list):

    def one(self) -> ModInfo:
        if len(self) != 1:
            raise ValueError(f'expected 1 item, {len(self)} found')
        return self[0]

    def __repr__(self) -> str:
        class_name = type(self).__name__
        super_repr = super().__repr__()
        return f'{class_name}({super_repr})'


class ModSrcInfo(NamedTuple):

    item_id: str
    attr_id: Union[int, None]
    hardcoded: Union[float, None]

    @classmethod
    def from_mixed(cls, data: list) -> ModSrcInfo:
        item_id, value_src = data
        attr_id = value_src.get('attr')
        hardcoded = value_src.get('hc')
        return cls(item_id, attr_id, hardcoded)
