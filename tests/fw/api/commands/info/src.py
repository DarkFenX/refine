import dataclasses

from fw.api.commands import BaseCommand


@dataclasses.dataclass(kw_only=True)
class SrcInfoItemsCmd(BaseCommand):

    type_ids: list[int]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'item_infos'
        body['type_ids'] = self.type_ids
        return body


@dataclasses.dataclass(kw_only=True)
class SrcInfoMutatedItemsCmd(BaseCommand):

    mutations: list[tuple[int, int]]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'mutated_item_infos'
        body['mutations'] = self.mutations
        return body
