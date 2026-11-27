import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_attr_map_request
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class SrcInfoItemsCmd(BaseCommand):

    type_ids: list[int] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'item_infos'
        conditional_insert(container=body, path=['type_ids'], value=self.type_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class SrcInfoMutatedItemsCmd(BaseCommand):

    mutations: list[tuple[int, int]] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'mutated_item_infos'
        conditional_insert(container=body, path=['mutations'], value=self.mutations)
        return body


@dataclasses.dataclass(kw_only=True)
class SrcInfoMutatedAttrRollsCmd(BaseCommand):

    mutation: tuple[int, int]
    values: dict[int | str, float]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'mutated_attr_rolls'
        body['mutation'] = self.mutation
        body['values'] = process_attr_map_request(attr_map=self.values)
        return body
