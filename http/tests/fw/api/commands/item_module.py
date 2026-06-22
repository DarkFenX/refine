import dataclasses
import typing

from fw.util import Absent, conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.api.aliases import MutaAdd, MutaChange
    from fw.consts import ApiEffMode, ApiModAddMode, ApiModuleState, ApiOptionalReload, ApiRack


@dataclasses.dataclass(kw_only=True)
class BaseModuleCmd(BaseCommand):
    type_id: int | type[Absent]
    state: ApiModuleState | type[Absent]
    charge_type_id: int | type[Absent] | None
    spool: str | type[Absent] | None
    optional_reload: ApiOptionalReload | type[Absent] | None
    effect_modes: dict[str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = {'type': 'module'}
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
class FitModuleAddCmd(BaseModuleCmd):

    rack: ApiRack
    add_mode: ApiModAddMode | dict[ApiModAddMode, int] | type[Absent]
    mutation: MutaAdd | type[Absent]
    projs: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['rack'], value=self.rack)
        conditional_insert(container=body, path=['add_mode'], value=self.add_mode)
        conditional_insert(container=body, path=['mutation'], value=self.mutation)
        conditional_insert(container=body, path=['projs'], value=self.projs)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemModuleAddCmd(FitModuleAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolModuleAddCmd(ItemModuleAddCmd):
    ...


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemModuleChangeCmd(BaseModuleCmd):

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
class FitModuleChangeCmd(ItemModuleChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolModuleChangeCmd(FitModuleChangeCmd):
    ...
