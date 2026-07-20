import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseBoosterCmd(BaseCommand):

    type_id: int | type[Absent]
    state: bool | type[Absent]
    side_effects: dict[str, bool] | type[Absent]
    effect_modes: dict[str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['side_effects'], value=self.side_effects)
        conditional_insert(container=body, path=['effect_modes'], value=self.effect_modes)
        return body


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemBoosterAddCmd(BaseBoosterCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'booster'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitBoosterAddCmd(BaseBoosterCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_booster'
        return body


@dataclasses.dataclass(kw_only=True)
class SolBoosterAddCmd(BaseBoosterCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_booster'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemBoosterChangeCmd(BaseBoosterCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'booster'
        return body


@dataclasses.dataclass(kw_only=True)
class FitBoosterChangeCmd(BaseBoosterCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_booster'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolBoosterChangeCmd(BaseBoosterCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_booster'
        body['item_id'] = self.item_id
        return body
