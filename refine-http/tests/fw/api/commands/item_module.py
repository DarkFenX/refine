import dataclasses
import typing

from fw.util import Absent, conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.api.aliases import MutaAdd, MutaChange
    from fw.consts import ApiEffMode, ApiModAddMode, ApiModMvMode, ApiModuleState, ApiOptionalReload, ApiRack


@dataclasses.dataclass(kw_only=True)
class BaseModuleCmd(BaseCommand):

    type_id: int | type[Absent]
    state: ApiModuleState | type[Absent]
    charge_type_id: int | type[Absent] | None
    spool: str | type[Absent] | None
    optional_reload: ApiOptionalReload | type[Absent] | None
    effect_modes: dict[str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['charge_type_id'], value=self.charge_type_id)
        conditional_insert(container=body, path=['spool'], value=self.spool)
        conditional_insert(container=body, path=['optional_reload'], value=self.optional_reload)
        conditional_insert(container=body, path=['effect_modes'], value=self.effect_modes)
        return body


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseModuleAddCmd(BaseModuleCmd):

    rack: ApiRack
    add_mode: ApiModAddMode | dict[ApiModAddMode, int] | type[Absent]
    mutation: MutaAdd | type[Absent]
    proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['rack'], value=self.rack)
        conditional_insert(container=body, path=['add_mode'], value=self.add_mode)
        conditional_insert(container=body, path=['mutation'], value=self.mutation)
        conditional_insert(container=body, path=['proj_item_ids'], value=self.proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemModuleAddCmd(BaseModuleAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'module'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitModuleAddCmd(BaseModuleAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_module'
        return body


@dataclasses.dataclass(kw_only=True)
class SolModuleAddCmd(BaseModuleAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'add_module'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseModuleChangeCmd(BaseModuleCmd):

    move: ApiModMvMode | dict[ApiModMvMode, int] | type[Absent]
    mutation: MutaAdd | MutaChange | type[Absent] | None
    add_proj_item_ids: list[str] | type[Absent]
    rm_proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['move'], value=self.move)
        conditional_insert(container=body, path=['mutation'], value=self.mutation)
        conditional_insert(container=body, path=['add_proj_item_ids'], value=self.add_proj_item_ids)
        conditional_insert(container=body, path=['rm_proj_item_ids'], value=self.rm_proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemModuleChangeCmd(BaseModuleChangeCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'module'
        return body


@dataclasses.dataclass(kw_only=True)
class FitModuleChangeCmd(BaseModuleChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_module'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolModuleChangeCmd(BaseModuleChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_module'
        body['item_id'] = self.item_id
        return body
