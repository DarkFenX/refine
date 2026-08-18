import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.aliases import InfoMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseInfoItemCmd(BaseCommand):

    item_id: str
    item_mode: InfoMode | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'item_info'
        body['item_id'] = self.item_id
        conditional_insert(container=body, path=['item_mode'], value=self.item_mode)
        return body


@dataclasses.dataclass(kw_only=True)
class FitInfoItemCmd(BaseInfoItemCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class SolInfoItemCmd(BaseInfoItemCmd):
    ...
