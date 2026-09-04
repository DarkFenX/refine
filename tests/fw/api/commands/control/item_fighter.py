import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_effect_map_request
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode, ApiMinionState, ApiRearmMinion
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseCtlFighterCmd(BaseCommand):

    type_id: int | type[Absent]
    state: ApiMinionState | type[Absent]
    count_override: int | type[Absent] | None
    abilities: dict[int, bool] | type[Absent]
    rearm_minion: ApiRearmMinion | type[Absent] | None
    coordinates: tuple[float, float, float] | type[Absent]
    movement: tuple[float, float, float] | type[Absent]
    effect_modes: dict[int | str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['count_override'], value=self.count_override)
        conditional_insert(container=body, path=['abilities'], value=self.abilities)
        conditional_insert(container=body, path=['rearm_minion'], value=self.rearm_minion)
        conditional_insert(container=body, path=['coordinates'], value=self.coordinates)
        conditional_insert(container=body, path=['movement'], value=self.movement)
        conditional_insert(
            container=body,
            path=['effect_modes'],
            value=process_effect_map_request(effect_map=self.effect_modes))
        return body


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlFighterAddCmd(BaseCtlFighterCmd):

    proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['proj_item_ids'], value=self.proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemCtlFighterAddCmd(BaseCtlFighterAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fighter'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlFighterAddCmd(BaseCtlFighterAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fighter_add'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlFighterAddCmd(BaseCtlFighterAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fighter_add'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlFighterChangeCmd(BaseCtlFighterCmd):

    add_proj_item_ids: list[str] | type[Absent]
    rm_proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['add_proj_item_ids'], value=self.add_proj_item_ids)
        conditional_insert(container=body, path=['rm_proj_item_ids'], value=self.rm_proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemCtlFighterChangeCmd(BaseCtlFighterChangeCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fighter'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlFighterChangeCmd(BaseCtlFighterChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fighter_change'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlFighterChangeCmd(BaseCtlFighterChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fighter_change'
        body['item_id'] = self.item_id
        return body
