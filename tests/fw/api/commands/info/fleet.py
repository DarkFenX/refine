import dataclasses
import typing

from fw.api.commands import BaseCommand
from fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from fw.api.aliases import InfoMode
    from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class SolInfoFleetCmd(BaseCommand):

    fleet_id: str
    fleet_mode: InfoMode | type[Absent]

    def serialize(self) -> dict:
        body = super().serialize()
        body['type'] = 'fleet_info'
        body['fleet_id'] = self.fleet_id
        conditional_insert(container=body, path=['fleet_mode'], value=self.fleet_mode)
        return body
