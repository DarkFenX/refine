import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseAutochargeCmd(BaseCommand):

    state: bool | type[Absent]
    effect_modes: dict[str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = {'type': 'autocharge'}
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['effect_modes'], value=self.effect_modes)
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemAutochargeChangeCmd(BaseAutochargeCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class FitAutochargeChangeCmd(ItemAutochargeChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolAutochargeChangeCmd(FitAutochargeChangeCmd):
    ...
