import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_effect_map_request, process_muta_add_request, process_muta_change_request
from fw.util import Absent, conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.aliases import MutaAdd, MutaChange
    from fw.consts import ApiEffMode, ApiMinionState, ApiNpcProp


@dataclasses.dataclass(kw_only=True)
class BaseCtlDroneCmd(BaseCommand):

    type_id: int | type[Absent]
    state: ApiMinionState | type[Absent]
    npc_prop_override: ApiNpcProp | type[Absent] | None
    coordinates: tuple[float, float, float] | type[Absent]
    movement: tuple[float, float, float] | type[Absent]
    effect_modes: dict[int | str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['npc_prop_override'], value=self.npc_prop_override)
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
class BaseCtlDroneAddCmd(BaseCtlDroneCmd):

    mutation: MutaAdd | type[Absent]
    proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(
            container=body,
            path=['mutation'],
            value=process_muta_add_request(mutation=self.mutation))
        conditional_insert(container=body, path=['proj_item_ids'], value=self.proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemCtlDroneAddCmd(BaseCtlDroneAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'drone'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlDroneAddCmd(BaseCtlDroneAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'drone_add'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlDroneAddCmd(BaseCtlDroneAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'drone_add'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlDroneChangeCmd(BaseCtlDroneCmd):

    mutation: MutaAdd | MutaChange | type[Absent] | None
    add_proj_item_ids: list[str] | type[Absent]
    rm_proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(
            container=body,
            path=['mutation'],
            value=process_muta_change_request(mutation=self.mutation))
        conditional_insert(container=body, path=['add_proj_item_ids'], value=self.add_proj_item_ids)
        conditional_insert(container=body, path=['rm_proj_item_ids'], value=self.rm_proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemCtlDroneChangeCmd(BaseCtlDroneChangeCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'drone'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlDroneChangeCmd(BaseCtlDroneChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'drone_change'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlDroneChangeCmd(BaseCtlDroneChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'drone_change'
        body['item_id'] = self.item_id
        return body
