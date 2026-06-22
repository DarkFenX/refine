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
        body = {'type': 'drone'}
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
class FitDroneAddCmd(BaseDroneCmd):

    mutation: MutaAdd | type[Absent]
    projs: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['mutation'], value=self.mutation)
        conditional_insert(container=body, path=['projs'], value=self.projs)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemDroneAddCmd(FitDroneAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolDroneAddCmd(ItemDroneAddCmd):
    ...


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemDroneChangeCmd(BaseDroneCmd):

    mutation: MutaAdd | MutaChange | type[Absent] | None
    add_projs: list[str] | type[Absent]
    rm_projs: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['mutation'], value=self.mutation)
        conditional_insert(container=body, path=['add_projs'], value=self.add_projs)
        conditional_insert(container=body, path=['rm_projs'], value=self.rm_projs)
        return body


@dataclasses.dataclass(kw_only=True)
class FitDroneChangeCmd(ItemDroneChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolDroneChangeCmd(FitDroneChangeCmd):
    ...
