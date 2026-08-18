import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import Absent, conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiModRmMode


####################################################################################################
# Removal
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseItemRemoveCmd(BaseCommand):

    rm_mode: ApiModRmMode | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['rm_mode'], value=self.rm_mode)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemItemRemoveCmd(BaseItemRemoveCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class FitItemRemoveCmd(BaseItemRemoveCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'item_remove'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolItemRemoveCmd(BaseItemRemoveCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'item_remove'
        body['item_id'] = self.item_id
        return body
