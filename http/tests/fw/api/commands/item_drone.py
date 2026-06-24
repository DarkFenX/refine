import dataclasses
import typing

from fw.util import Absent, conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.api.aliases import MutaAdd, MutaChange
    from fw.consts import ApiEffMode, ApiMinionState, ApiNpcProp


@dataclasses.dataclass(kw_only=True)
class BaseDroneCmd(BaseCommand):

    type_id: int | type[Absent]
    state: ApiMinionState | type[Absent]
    npc_prop: ApiNpcProp | type[Absent] | None
    coordinates: tuple[float, float, float] | type[Absent]
    movement: tuple[float, float, float] | type[Absent]
    effect_modes: dict[int | str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['npc_prop'], value=self.npc_prop)
        conditional_insert(container=body, path=['coordinates'], value=self.coordinates)
        conditional_insert(container=body, path=['movement'], value=self.movement)
        conditional_insert(container=body, path=['effect_modes'], value=self.effect_modes)
        return body


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseDroneAddCmd(BaseDroneCmd):

    mutation: MutaAdd | type[Absent]
    proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['mutation'], value=self.mutation)
        conditional_insert(container=body, path=['proj_item_ids'], value=self.proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemDroneAddCmd(BaseDroneAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'drone'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitDroneAddCmd(BaseDroneAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_drone'
        return body


@dataclasses.dataclass(kw_only=True)
class SolDroneAddCmd(BaseDroneAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_drone'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseDroneChangeCmd(BaseDroneCmd):

    mutation: MutaAdd | MutaChange | type[Absent] | None
    add_proj_item_ids: list[str] | type[Absent]
    rm_proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['mutation'], value=self.mutation)
        conditional_insert(container=body, path=['add_proj_item_ids'], value=self.add_proj_item_ids)
        conditional_insert(container=body, path=['rm_proj_item_ids'], value=self.rm_proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemDroneChangeCmd(BaseDroneChangeCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'drone'
        return body


@dataclasses.dataclass(kw_only=True)
class FitDroneChangeCmd(BaseDroneChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_drone'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolDroneChangeCmd(BaseDroneChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_drone'
        body['item_id'] = self.item_id
        return body
