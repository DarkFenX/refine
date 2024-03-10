from __future__ import annotations

from collections import namedtuple


ModInfo = namedtuple('ModInfo', ('src_item_id', 'src_attr_id', 'val', 'op', 'penalized', 'aggr_mode'))


class ModInfoMap(dict):

    def find_by_src_item(self, tgt_attr_id: int, src_item_id: int) -> ModInfoList:
        return self.get(tgt_attr_id, ModInfoList()).find_by_src_item(src_item_id=src_item_id)

    def __repr__(self) -> str:
        class_name = type(self).__name__
        super_repr = super().__repr__()
        return f'{class_name}({super_repr})'


class ModInfoList(list):

    def find_by_src_item(self, src_item_id: int) -> ModInfoList:
        return ModInfoList(i for i in self if i.src_item_id == src_item_id)

    def one(self) -> ModInfo:
        if len(self) != 1:
            raise ValueError(f'expected 1 item, {len(self)} found')
        return self[0]

    def __repr__(self) -> str:
        class_name = type(self).__name__
        super_repr = super().__repr__()
        return f'{class_name}({super_repr})'
