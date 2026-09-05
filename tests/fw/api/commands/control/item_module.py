import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.api.commands.helpers import process_effect_map_request, process_muta_add_request, process_muta_change_request
from fw.util import Absent, conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.aliases import MutaAdd, MutaChange
    from fw.consts import ApiEffMode, ApiModAddMode, ApiModMvMode, ApiModuleState, ApiOptionalReload, ApiRack


@dataclasses.dataclass(kw_only=True)
class BaseCtlModuleCmd(BaseCommand):

    type_id: int | type[Absent]
    state: ApiModuleState | type[Absent]
    charge_type_id: int | type[Absent] | None
    spool_override: str | type[Absent] | None
    optional_reload_override: ApiOptionalReload | type[Absent] | None
    effect_modes: dict[int | str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['charge_type_id'], value=self.charge_type_id)
        conditional_insert(container=body, path=['spool_override'], value=self.spool_override)
        conditional_insert(container=body, path=['optional_reload_override'], value=self.optional_reload_override)
        conditional_insert(
            container=body,
            path=['effect_modes'],
            value=process_effect_map_request(effect_map=self.effect_modes))
        return body


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlModuleAddCmd(BaseCtlModuleCmd):

    rack: ApiRack
    add_mode: ApiModAddMode | dict[ApiModAddMode, int] | type[Absent]
    mutation: MutaAdd | type[Absent]
    proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['rack'], value=self.rack)
        conditional_insert(container=body, path=['add_mode'], value=self.add_mode)
        conditional_insert(
            container=body,
            path=['mutation'],
            value=process_muta_add_request(mutation=self.mutation))
        conditional_insert(container=body, path=['proj_item_ids'], value=self.proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemCtlModuleAddCmd(BaseCtlModuleAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'module'
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlModuleAddCmd(BaseCtlModuleAddCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'module_add'
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlModuleAddCmd(BaseCtlModuleAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'module_add'
        body['fit_id'] = self.fit_id
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class BaseCtlModuleChangeCmd(BaseCtlModuleCmd):

    move: ApiModMvMode | dict[ApiModMvMode, int] | type[Absent]
    mutation: MutaAdd | MutaChange | type[Absent] | None
    add_proj_item_ids: list[str] | type[Absent]
    rm_proj_item_ids: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['move'], value=self.move)
        conditional_insert(
            container=body,
            path=['mutation'],
            value=process_muta_change_request(mutation=self.mutation))
        conditional_insert(container=body, path=['add_proj_item_ids'], value=self.add_proj_item_ids)
        conditional_insert(container=body, path=['rm_proj_item_ids'], value=self.rm_proj_item_ids)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemCtlModuleChangeCmd(BaseCtlModuleChangeCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'module'
        return body


@dataclasses.dataclass(kw_only=True)
class FitCtlModuleChangeCmd(BaseCtlModuleChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'module_change'
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolCtlModuleChangeCmd(BaseCtlModuleChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'module_change'
        body['item_id'] = self.item_id
        return body
