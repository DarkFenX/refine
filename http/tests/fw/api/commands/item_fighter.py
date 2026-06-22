import dataclasses
import typing

from fw.util import conditional_insert
from .base import Command

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode, ApiMinionState, ApiRearmMinion
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseFighterCmd(Command):

    type_id: int | type[Absent]
    state: ApiMinionState | type[Absent]
    count: int | type[Absent] | None
    abilities: dict[int, bool] | type[Absent]
    rearm_minion: ApiRearmMinion | type[Absent] | None
    coordinates: tuple[float, float, float] | type[Absent]
    movement: tuple[float, float, float] | type[Absent]
    effect_modes: dict[str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = {'type': 'fighter'}
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['count'], value=self.count)
        conditional_insert(container=body, path=['abilities'], value=self.abilities)
        conditional_insert(container=body, path=['rearm_minion'], value=self.rearm_minion)
        conditional_insert(container=body, path=['coordinates'], value=self.coordinates)
        conditional_insert(container=body, path=['movement'], value=self.movement)
        conditional_insert(container=body, path=['effect_modes'], value=self.effect_modes)
        return body


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class FitFighterAddCmd(BaseFighterCmd):

    projs: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['projs'], value=self.projs)
        return body


@dataclasses.dataclass(kw_only=True)
class ItemFighterAddCmd(FitFighterAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolFighterAddCmd(ItemFighterAddCmd):
    ...


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemFighterChangeCmd(BaseFighterCmd):

    add_projs: list[str] | type[Absent]
    rm_projs: list[str] | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['add_projs'], value=self.add_projs)
        conditional_insert(container=body, path=['rm_projs'], value=self.rm_projs)
        return body


@dataclasses.dataclass(kw_only=True)
class FitFighterChangeCmd(ItemFighterChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolFighterChangeCmd(FitFighterChangeCmd):
    ...
