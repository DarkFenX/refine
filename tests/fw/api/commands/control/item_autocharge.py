import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_effect_map_request
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseCtlAutochargeCmd(BaseCommand):

    state: bool | type[Absent]
    effect_modes: dict[int | str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(
            container=body,
            path=['effect_modes'],
            value=process_effect_map_request(effect_map=self.effect_modes))
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemCtlAutochargeChangeCmd(BaseCtlAutochargeCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'autocharge'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlAutochargeChangeCmd(BaseCtlAutochargeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'autocharge_change'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlAutochargeChangeCmd(BaseCtlAutochargeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'autocharge_change'
        body['item_id'] = self.item_id
        return body
