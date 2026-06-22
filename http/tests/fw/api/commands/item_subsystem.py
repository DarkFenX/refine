import dataclasses
import typing

from fw.util import conditional_insert
from .base import Command

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseSubsystemCmd(Command):

    type_id: int | type[Absent]
    state: bool | type[Absent]
    effect_modes: dict[str, ApiEffMode] | type[Absent]

    def serialize(self) -> dict:
        body = {'type': 'subsystem'}
        conditional_insert(container=body, path=['type_id'], value=self.type_id)
        conditional_insert(container=body, path=['state'], value=self.state)
        conditional_insert(container=body, path=['effect_modes'], value=self.effect_modes)
        return body


####################################################################################################
# Addition
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class FitSubsystemAddCmd(BaseSubsystemCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class ItemSubsystemAddCmd(FitSubsystemAddCmd):

    fit_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['fit_id'] = self.fit_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolSubsystemAddCmd(ItemSubsystemAddCmd):
    ...


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class ItemSubsystemChangeCmd(BaseSubsystemCmd):
    ...


@dataclasses.dataclass(kw_only=True)
class FitSubsystemChangeCmd(ItemSubsystemChangeCmd):

    item_id: str

    def serialize(self) -> dict:
        body = super().serialize()
        body['item_id'] = self.item_id
        return body


@dataclasses.dataclass(kw_only=True)
class SolSubsystemChangeCmd(FitSubsystemChangeCmd):
    ...
