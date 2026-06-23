import dataclasses
import typing

from fw.util import conditional_insert
from .base import BaseCommand

if typing.TYPE_CHECKING:
    from fw.api.aliases import DpsProfile
    from fw.consts import ApiNpcProp, ApiOptionalReload, ApiRearmMinion, ApiSecZone
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class BaseSolCmd(BaseCommand):

    sec_zone: ApiSecZone | type[Absent]
    default_incoming_dps: DpsProfile | type[Absent]
    default_spool: str | type[Absent]
    default_npc_prop: ApiNpcProp | type[Absent]
    default_optional_reloads: ApiOptionalReload | type[Absent]
    default_rearm_minions: ApiRearmMinion | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['sec_zone'], value=self.sec_zone)
        conditional_insert(container=body, path=['default_incoming_dps'], value=self.default_incoming_dps)
        conditional_insert(container=body, path=['default_spool'], value=self.default_spool)
        conditional_insert(container=body, path=['default_npc_prop'], value=self.default_npc_prop)
        conditional_insert(container=body, path=['default_optional_reloads'], value=self.default_optional_reloads)
        conditional_insert(container=body, path=['default_rearm_minions'], value=self.default_rearm_minions)
        return body


####################################################################################################
# Creating
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class RootSolCreateCmd(BaseSolCmd):

    src_alias: str | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        conditional_insert(container=body, path=['src_alias'], value=self.src_alias)
        return body


####################################################################################################
# Changing
####################################################################################################
@dataclasses.dataclass(kw_only=True)
class SolSolChangeCmd(BaseSolCmd):

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'change_sol'
        return body
